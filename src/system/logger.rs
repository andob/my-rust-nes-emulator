
#[macro_export]
macro_rules! log_cpu_opcode
{
    ($($arg:tt)*) => { println!($($arg)*) }
}

#[macro_export]
macro_rules! log_cpu_too_slow
{
    ($($arg:tt)*) => { eprintln!($($arg)*) }
}

#[macro_export]
macro_rules! log_test_result
{
    ($($arg:tt)*) => { println!($($arg)*) }
}

#[macro_export]
macro_rules! log_syntax
{
    ($($arg:tt)*) => { println!($($arg)*) }
}
