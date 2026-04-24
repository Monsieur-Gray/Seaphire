#![allow(non_snake_case)]
#![allow(unused)]

use colored::Colorize;
use SysBase::{SysThrow, PARSER};

// use pipeline::CppCompiler::*;
use core_utils::EXECUTE_2::check_exec_line;

fn main() {
    use std::fs::read_to_string;

    // ENTER YOUR FILE NAME HERE ->
    let file_name = "src/debug.inu";
    let txt = if let Ok(bruh) = read_to_string(&file_name) {
        bruh
    } else {
        crate::SysThrow!("I can't find this ing file!\n\tYOU HAD ONE JOB!")
    };

    let (mvec, vvec) = PARSER::pest_parse(&txt);

    // println!("heh ==> {:?}", &mvec);
    let MSEC = PARSER::make_msec(mvec);
    // println!("heh ==> {:?}", MSEC);
    let [sh, hh, regh] = PARSER::calloc(vvec);

    println!("\nCURRENTLY RUNNING ->  {}\n", file_name.purple().bold());
    // compile_to_cpp(&MSEC, sh, hh);
    check_exec_line(&MSEC, sh, hh, regh);
}