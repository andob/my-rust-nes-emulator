use crate::system::System;

mod system;
mod type_alias;

fn main()
{
    //todo https://bugzmanov.github.io/nes_ebook/chapter_1.html
    //todo https://www.nesdev.org/wiki/Programming_with_unofficial_opcodes
    //todo http://www.oxyron.de/html/opcodes02.html
    //todo https://www.nesdev.org/undocumented_opcodes.txt
    //todo http://www.6502.org/tutorials/6502opcodes.html
    //todo https://www.pagetable.com/c64ref/6502/?tab=2

    let mut system = System::new();
    system.run();
}
