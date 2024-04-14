#![allow(non_camel_case_types, non_snake_case)]

pub mod defkeys;

pub mod read_file;
pub mod get_type;
pub mod split_sec;
pub mod fetch_data; // also available in builtin_fns

pub mod mem_alloc;


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


#[macro_export]
macro_rules! SysThrow {
    ($msg:literal) => {{        // &str
        use colored::*;
        panic!(
            "{}", format!("->!> {}", $msg)
            .truecolor(250, 150, 125)
            .bold()
            .on_truecolor(25, 25, 25)
        )
    }};

    ($msg:expr) => {{          // String
        use colored::*;
        panic!(
            "{}", format!("->!> {}", $msg)
            .truecolor(200, 150, 100)
            .bold()
            .on_truecolor(20, 20, 20)
        )
    }};
}
