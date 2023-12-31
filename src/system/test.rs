use anyhow::Result;
use maplit2::hashmap;
use crate::system::test::cpu_kevtris_nestest::test_cpu_with_kevtris_nestest;

mod cpu_kevtris_nestest;

pub struct Test {}

impl Test
{
    pub fn run_test(&self, test_name : String, test_args: Vec<String>) -> Result<()>
    {
        let tests = hashmap!
        {
            "cpu_kevtris_nestest" => test_cpu_with_kevtris_nestest as fn(Vec<String>) -> Result<()>,
        };

        if let Some(test) = tests.get(test_name.as_str())
        {
            return test(test_args);
        }

        println!("Available tests:\n{}", tests.iter()
            .map(|(test_name,_)| format!("{}\n", test_name))
            .collect::<String>());
        return Ok(());
    }
}
