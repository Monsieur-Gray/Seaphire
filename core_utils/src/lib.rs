#![allow(non_snake_case)]

extern crate colored;

pub mod EXECUTE;
// pub mod compiler_tools;
// pub mod CppCompiler;

pub mod PRINT;
pub mod Input;

pub mod ARITHMETIC;
pub mod Compare;

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
