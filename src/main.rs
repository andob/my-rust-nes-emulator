use std::env;
use crate::system::System;

mod system;

fn main()
{
    //todo https://bugzmanov.github.io/nes_ebook/chapter_1.html
    //todo https://www.nesdev.org/undocumented_opcodes.txt
    //todo http://www.6502.org/tutorials/6502opcodes.html
    //todo https://www.pagetable.com/c64ref/6502/?tab=2
    let args = env::args().collect::<Vec<String>>();
    if args.len()>=3 && args[1]=="test" && args[2]=="snake"
    {
        System::test().test_snake_game();
    }
    else
    {
        let mut nes = System::new(vec![]);
        nes.run();
    }
}
