#![allow(non_snake_case)]
#![allow(unused)]

use colored::Colorize;
use SysBase::{SysThrow, PARSER};
use SysBase::memory_layout::{Memory, Env, MemCell};

// use pipeline::CppCompiler::*;
use core_utils::EXECUTE_2::check_exec_line;

// ! 7:05pm

fn main() {
    use std::fs::read_to_string;

    // ENTER YOUR FILE NAME HERE ->
    let file_name = "src/calc2.inu";
    let txt = if let Ok(bruh) = read_to_string(&file_name) {
        bruh
    } else {
        crate::SysThrow!("I can't find this ing file!\n\tYOU HAD ONE JOB!")
    };

    let (mvec, vvec) = PARSER::pest_parse(&txt);

    // println!("heh ==> {:?}", &mvec);
    let MSEC = PARSER::make_msec(mvec);
    // println!("heh ==> {:?}", MSEC);

    let vars = PARSER::collect_vars(vvec);
    let mut memory =  &mut Memory::new();
    let mut env = &mut Env::new();

    for var in vars {
        let memcell: MemCell = MemCell {
            data: var.data,
            scope: var.scope,
            is_mutable: var.is_mutable
        };

        let ptr: usize = memory.alloc(memcell);

        env.insert(var.name, ptr);
    }


    println!("\nCURRENTLY RUNNING ->  {}\n", file_name.purple().bold());
    // compile_to_cpp(&MSEC, sh, hh);
    check_exec_line(&MSEC, memory, env);
}