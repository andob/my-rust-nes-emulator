use anyhow::Result;
use maplit2::hashmap;
use crate::system::test::cpu_kevtris_nestest::test_cpu_with_kevtris_nestest;
use crate::system::test::ppu_spritecans_test::test_ppu_with_spritecans_test;

mod cpu_kevtris_nestest;
mod ppu_spritecans_test;

pub struct Test {}

impl Test
{
    pub fn run_test(&self, test_name : String) -> Result<()>
    {
        let tests = hashmap!
        {
            "cpu_kevtris_nestest" => test_cpu_with_kevtris_nestest as fn() -> Result<()>,
            "ppu_spritecans_test" => test_ppu_with_spritecans_test,
        };

        if let Some(test) = tests.get(test_name.as_str())
        {
            return test();
        }

        println!("Available tests:\n{}", tests.iter()
            .map(|(test_name,_)| format!("{}\n", test_name))
            .collect::<String>());
        return Ok(());
    }
}
