use std::env;
use crate::system::System;

mod system;

fn main()
{
    //todo https://bugzmanov.github.io/nes_ebook/chapter_1.html
    //todo https://www.nesdev.org/undocumented_opcodes.txt
    let args = env::args().collect::<Vec<String>>();
    if args.len()>=3 && args[1]=="test" && args[2]=="snake"
    {
        System::test().test_snake_game();
    }
    else
    {
        let mut nes = System::new(Box::new([]));
        nes.run();
    }
}
