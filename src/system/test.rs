use crate::system::test::snake::run_snake_game;

mod snake;

pub struct Test {}

impl Test
{
    pub fn test_snake_game(&self)
    {
        run_snake_game().unwrap();
    }
}
