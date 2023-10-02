use std::env;
use crate::system::System;

mod system;

fn main()
{
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
