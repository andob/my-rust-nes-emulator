use std::collections::HashMap;
use std::sync::mpsc::channel;
use anyhow::{anyhow, Context, Result};
use itertools::iproduct;
use rand::random;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::{address_from_high_low, codeloc};
use crate::system::debugger::RAMSnapshot;
use crate::system::{address, byte, System, SystemStartArgs};
use crate::system::cpu::program_rom::ProgramROM;
use crate::system::ppu::character_rom::CharacterROM;
use crate::system::rom::Mapper;

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

pub fn run_snake_game() -> Result<()>
{
    let executable_bytes = include_bytes!("snake/snake.bin");
    let program_rom = ProgramROM::new(Mapper::SnakeTestGame, executable_bytes);
    let character_rom = CharacterROM::new(Mapper::SnakeTestGame, &[]);

    let (cpu_bus_watcher_sender, cpu_bus_watcher_receiver) = channel::<RAMSnapshot>();
    let (cpu_bus_setter_sender, cpu_bus_setter_receiver) = channel::<RAMSnapshot>();

    let mut start_args = SystemStartArgs::with_parsed_rom(program_rom, character_rom);
    start_args.cpu_debugger.cpu_bus_watcher_targets = (0x0000..0x0400).collect();
    start_args.cpu_debugger.cpu_bus_watcher = Some(cpu_bus_watcher_sender);
    start_args.cpu_debugger.cpu_bus_setter = Some(cpu_bus_setter_receiver);
    start_args.headless = true;
    let running_system = System::start(start_args);

    let (driver_index, _driver) = sdl2::render::drivers().enumerate()
        .find(|(_index, driver)| driver.name=="opengl")
        .ok_or(anyhow!("OpenGL driver not found!")).context(codeloc!())?;

    let sdl_context = sdl2::init().map_err(|msg|anyhow!(msg)).context(codeloc!())?;
    let video_subsystem = sdl_context.video().map_err(|msg|anyhow!(msg)).context(codeloc!())?;

    let window = video_subsystem.window("Snake", (SCREEN_WIDTH*BLOCK_WIDTH) as u32,
        (SCREEN_HEIGHT*BLOCK_HEIGHT) as u32).position_centered().opengl().build().context(codeloc!())?;
    let mut canvas = window.into_canvas().index(driver_index as u32).build().context(codeloc!())?;

    let mut last_keycode = Keycode::Space;
    loop
    {
        if let Ok(ram) = cpu_bus_watcher_receiver.recv()
        {
            render(&mut canvas, &ram)?;
        }

        let mut event_pump = sdl_context.event_pump().map_err(|e|anyhow!(e.clone()))?;
        for event in event_pump.poll_iter()
        {
            match event
            {
                Event::KeyDown { keycode: Some(keycode@ Keycode::Up), .. } |
                Event::KeyDown { keycode: Some(keycode@ Keycode::Down), .. } |
                Event::KeyDown { keycode: Some(keycode@ Keycode::Left), .. } |
                Event::KeyDown { keycode: Some(keycode@ Keycode::Right), .. } => { last_keycode = keycode; }
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { running_system.shutdown(); return Ok(()); }
                _ => {}
            }
        }

        let mut variables_to_send : HashMap<address, byte> = HashMap::new();
        variables_to_send.insert(RANDOM_NUMBER_GENERATOR_ADDRESS, random::<u8>());
        variables_to_send.insert(LAST_PRESSED_BUTTON_ADDRESS, encode_keycode(last_keycode));
        cpu_bus_setter_sender.send(RAMSnapshot::new(variables_to_send))?;
    }
}

fn render(canvas : &mut WindowCanvas, ram : &RAMSnapshot) -> Result<()>
{
    let mut screen = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];

    let snake_length = ram.variables[&SPRITE_SNAKE_LENGTH_LOCATION];
    for i in 0..snake_length
    {
        if let Some((x, y)) = find_sprite_location(&ram, SPRITE_SNAKE_PART_START_LOCATION+2*(i as address))
        {
            screen[x][y] = true;
        }
    }

    if let Some((x, y)) = find_sprite_location(&ram, SPRITE_APPLE_LOCATION)
    {
        screen[x][y] = true;
    }

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
        ).map_err(|msg|anyhow!(msg.clone())).context(codeloc!())?;
    }

    canvas.present();
    return Ok(());
}

fn find_screen_location_by_address(target_address : address) -> Option<(usize, usize)>
{
    let location_to_address = |(x,y) : (usize, usize)|
        SCREEN_START_ADDRESS + (x+(y*SCREEN_WIDTH)) as address;

    let cartesian_product = iproduct!(0..SCREEN_WIDTH, 0..SCREEN_HEIGHT);
    return cartesian_product.map(|point| (location_to_address(point), point))
            .find(|(address, _point)| *address==target_address)
            .map(|(_address, point)| point);
}

fn find_sprite_location(ram : &RAMSnapshot, sprite_address : address) -> Option<(usize, usize)>
{
    let low = ram.variables[&sprite_address];
    let high = ram.variables[&(sprite_address+1)];
    let address = address_from_high_low!(high, low);
    return find_screen_location_by_address(address);
}

fn encode_keycode(keycode : Keycode) -> byte
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
