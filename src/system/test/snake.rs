use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use anyhow::{anyhow, Context, Result};
use itertools::iproduct;
use rand::random;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::codeloc;
use crate::system::debugger::Debugger;
use crate::system::{address, byte, System};

const SCREEN_WIDTH : usize = 32;
const SCREEN_HEIGHT : usize = 32;
const BLOCK_WIDTH : usize = 15;
const BLOCK_HEIGHT : usize = 15;

const RANDOM_NUMBER_GENERATOR_ADDRESS : address = 0x00FE;
const LAST_PRESSED_BUTTON_ADDRESS : address = 0x00FF;
const SCREEN_START_ADDRESS : address = 0x0200;
const SPRITE_APPLE_LOCATION : address = 0x0000;
const SPRITE_SNAKE_LENGTH_LOCATION : address = 0x0003;
const SPRITE_SNAKE_PART_START_LOCATION : address = 0x0010;
const ROM_IN_RAM_ADDRESS : address = 0x0600;

struct FrontendChannels
{
    screen_receiver: Receiver<[[bool; SCREEN_WIDTH]; SCREEN_HEIGHT]>,
    pressed_key_sender : Sender<Keycode>,
}

struct BackendChannels
{
    screen_sender: Sender<[[bool; SCREEN_WIDTH]; SCREEN_HEIGHT]>,
    pressed_key_receiver : Receiver<Keycode>,
}

fn create_channels() -> (FrontendChannels, BackendChannels)
{
    let (screen_sender, screen_receiver) = channel::<[[bool; SCREEN_WIDTH]; SCREEN_HEIGHT]>();
    let (pressed_key_sender, pressed_key_receiver) = channel::<Keycode>();

    let frontend_channels = FrontendChannels { screen_receiver, pressed_key_sender };
    let backend_channels = BackendChannels { screen_sender, pressed_key_receiver };

    return (frontend_channels, backend_channels);
}

pub fn run_snake_game() -> Result<()>
{
    let executable_bytes = *include_bytes!("snake.bin");

    let (frontend_channels, backend_channels) = create_channels();

    let (driver_index, _driver) = sdl2::render::drivers().enumerate()
        .find(|(_index, driver)| driver.name=="opengl")
        .ok_or(anyhow!("OpenGL driver not found!")).context(codeloc!())?;

    let sdl_context = sdl2::init().map_err(|msg|anyhow!(msg)).context(codeloc!())?;
    let video_subsystem = sdl_context.video().map_err(|msg|anyhow!(msg)).context(codeloc!())?;

    let window = video_subsystem.window("Snake", (SCREEN_WIDTH*BLOCK_WIDTH) as u32,
        (SCREEN_HEIGHT*BLOCK_HEIGHT) as u32).position_centered().opengl().build().context(codeloc!())?;
    let mut canvas = window.into_canvas().index(driver_index as u32).build().context(codeloc!())?;

    thread::spawn(move ||
    {
        let debugger = SnakeGameDebugger::new(backend_channels);
        let mut nes = System::new(Box::new(executable_bytes));
        nes.run_with_debugger(Box::new(debugger));
    });

    let mut last_keycode : Option<Keycode> = None;
    loop
    {
        if let Ok(screen) = frontend_channels.screen_receiver.recv()
        {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

            let cartesian_product = iproduct!(0..SCREEN_WIDTH, 0..SCREEN_HEIGHT);
            for (x, y) in cartesian_product.filter(|(x,y)| screen[*x][*y])
            {
                canvas.set_draw_color(Color::WHITE);
                canvas.fill_rect(Rect::new(
                    /*x*/ (x*BLOCK_WIDTH) as i32,
                    /*y*/ (y*BLOCK_HEIGHT) as i32,
                    /*width*/ BLOCK_WIDTH as u32,
                    /*height*/ BLOCK_HEIGHT as u32)
                ).map_err(|msg|anyhow!(msg)).context(codeloc!())?;
            }

            canvas.present();
        }

        let mut event_pump = sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter()
        {
            match event
            {
                Event::KeyDown { keycode: Some(keycode@ Keycode::Up), .. } |
                Event::KeyDown { keycode: Some(keycode@ Keycode::Down), .. } |
                Event::KeyDown { keycode: Some(keycode@ Keycode::Left), .. } |
                Event::KeyDown { keycode: Some(keycode@ Keycode::Right), .. } => { last_keycode = Some(keycode); }
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { return Ok(()); }
                _ => {}
            }
        }

        if let Some(keycode) = last_keycode
        {
            if let Ok(_) = frontend_channels.pressed_key_sender.send(keycode) {}
        }
    }
}

struct SnakeGameDebugger { backend_channels : BackendChannels }

impl SnakeGameDebugger
{
    pub fn new(backend_channels : BackendChannels) -> SnakeGameDebugger
    {
        return SnakeGameDebugger { backend_channels };
    }

    fn find_screen_location_by_address(&self, target_address : address) -> Option<(usize, usize)>
    {
        let location_to_address = |(x,y) : (usize, usize)|
            SCREEN_START_ADDRESS + (x+(y*SCREEN_WIDTH)) as address;

        let cartesian_product = iproduct!(0..SCREEN_WIDTH, 0..SCREEN_HEIGHT);
        return cartesian_product.map(|point| (location_to_address(point), point))
                .find(|(address, _point)| *address==target_address)
                .map(|(_address, point)| point);
    }

    fn find_sprite_location(&self, nes : &mut System, sprite_address : address) -> Option<(usize, usize)>
    {
        let low = nes.ram.get(sprite_address);
        let high = nes.ram.get(sprite_address+1);
        let address = ((high as address)<<8) | (low as address);
        return self.find_screen_location_by_address(address);
    }

    fn encode_keycode(&self, keycode : Keycode) -> byte
    {
        return match keycode
        {
            Keycode::Up    => { 0x77 }
            Keycode::Right => { 0x64 }
            Keycode::Down  => { 0x73 }
            Keycode::Left  => { 0x61 }
            _ => { 0 }
        }
    }
}

impl Debugger for SnakeGameDebugger
{
    fn before_cpu_opcode(&self, nes : &mut System)
    {
        nes.ram.put(RANDOM_NUMBER_GENERATOR_ADDRESS, random::<u8>());

        if let Ok(keycode) = self.backend_channels.pressed_key_receiver.try_recv()
        {
            nes.ram.put(LAST_PRESSED_BUTTON_ADDRESS, self.encode_keycode(keycode));
        }
    }

    fn after_cpu_opcode(&self, nes : &mut System)
    {
        if nes.cpu.program_counter >= ROM_IN_RAM_ADDRESS
        {
            nes.cpu.program_counter -= ROM_IN_RAM_ADDRESS;
        }

        let mut screen = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];

        let snake_length = nes.ram.get(SPRITE_SNAKE_LENGTH_LOCATION) as address;
        for i in 0..snake_length
        {
            if let Some((x, y)) = self.find_sprite_location(nes, SPRITE_SNAKE_PART_START_LOCATION+2*i)
            {
                screen[x][y] = true;
            }
        }

        if let Some((x, y)) = self.find_sprite_location(nes, SPRITE_APPLE_LOCATION)
        {
            screen[x][y] = true;
        }

        if let Ok(_) = self.backend_channels.screen_sender.send(screen) {}
    }
}
