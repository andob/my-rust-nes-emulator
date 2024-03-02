use anyhow::Result;
use maplit2::hashmap;
use crate::system::test::cpu_kevtris_nestest::test_cpu_with_kevtris_nestest;
use crate::system::test::ppu_blocks_test::test_ppu_with_blocks_testrom;
use crate::system::test::ppu_litewall_test::test_ppu_with_litewall_testrom;
use crate::system::test::ppu_physics_test::test_ppu_with_physics_testrom;
use crate::system::test::ppu_scroll_test::test_ppu_with_scroll_testrom;
use crate::system::test::ppu_sprite_overflow_test::test_ppu_with_sprite_overflow_testrom;
use crate::system::test::ppu_sprite_zero_hit_test::test_ppu_with_sprite_zero_hit_testrom;
use crate::system::test::ppu_spritecans_test::test_ppu_with_spritecans_testrom;

mod cpu_kevtris_nestest;
mod ppu_spritecans_test;
mod ppu_physics_test;
mod ppu_blocks_test;
mod ppu_litewall_test;
mod ppu_scroll_test;
mod ppu_sprite_overflow_test;
mod ppu_sprite_zero_hit_test;

pub struct Test {}

impl Test
{
    pub fn run_test(&self, test_name : String) -> Result<()>
    {
        let tests = hashmap!
        {
            "cpu_kevtris_nestest" => test_cpu_with_kevtris_nestest as fn() -> Result<()>,
            "ppu_blocks_test" => test_ppu_with_blocks_testrom,
            "ppu_litewall_test" => test_ppu_with_litewall_testrom,
            "ppu_physics_test" => test_ppu_with_physics_testrom,
            "ppu_spritecans_test" => test_ppu_with_spritecans_testrom,
            "ppu_scroll_test" => test_ppu_with_scroll_testrom,
            "ppu_sprite_overflow_test" => test_ppu_with_sprite_overflow_testrom,
            "ppu_sprite_zero_hit_test" => test_ppu_with_sprite_zero_hit_testrom,
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
