use crate::system::test::snake::run_snake_game;

mod snake;

pub struct Test {}

//todo implement this test: https://github.com/mwales/6502-tests/tree/master/hmc-6502
//todo implement this test: https://github.com/mwales/6502-tests/tree/master/kevtris_nestest
impl Test
{
    pub fn test_snake_game(&self)
    {
        run_snake_game().unwrap();
    }
}
