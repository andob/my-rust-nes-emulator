use anyhow::Result;
use maplit2::hashmap;
use crate::log_syntax;
use crate::system::test::snake::run_snake_game;
use crate::system::test::cpu_kevtris_nestest::test_cpu_with_kevtris_nestest;

mod snake;
mod cpu_kevtris_nestest;

pub struct Test {}

impl Test
{
    pub fn run_test(&self, name : String) -> Result<()>
    {
        let tests = hashmap!
        {
            "snake" => run_snake_game as fn() -> Result<()>,
            "cpu_kevtris_nestest" => test_cpu_with_kevtris_nestest,
        };

        if let Some(test) = tests.get(name.as_str())
        {
            return test();
        }

        log_syntax!("Available tests:\n{}", tests.iter()
            .map(|(test_name,_)| format!("{}\n", test_name))
            .collect::<String>());
        return Ok(());
    }
}
