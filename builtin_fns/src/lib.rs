#![allow(non_snake_case)]
#![allow(unreachable_code)]     // For ARITHMETIC

extern crate colored;

pub mod ARITHMETIC;
pub mod fetch_data;
pub mod EXECUTE;
pub mod PRNT;
pub mod COMPILE;
pub mod compile_ass;


#[macro_export]
macro_rules! Throw {
    ($msg:literal) => {{        // &str
        use colored::*;
        panic!(
            "{}", format!("->!> {}", $msg)
            .red().bold()
            .on_truecolor(15, 15, 15)
        )
    }};

    ($msg:expr) => {{          // String
        use colored::*;
        panic!(
            "{}", format!("->!> {}", $msg)
            .red().bold()
            .on_truecolor(15, 15, 15)
        )
    }};
}
