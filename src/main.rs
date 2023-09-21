use crate::system::System;

mod system;

fn main()
{
    //https://bugzmanov.github.io/nes_ebook/chapter_3_2.html
    //https://en.wikipedia.org/wiki/MOS_Technology_6502
    //https://www.masswerk.at/6502/6502_instruction_set.html
    //http://www.6502.org/tutorials/6502opcodes.html
    //https://www.pagetable.com/c64ref/6502/

    let mut system = System::new();
    system.run();
}
