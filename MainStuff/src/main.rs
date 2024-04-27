#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused)]

use my_modules::*;
use my_modules::PARSER::pest_parse;

use my_modules::PARSER;
use builtin_fns::EXECUTE::new_check_exec_line;


fn main() {
    use std::fs::read_to_string;
    let txt = if let Ok(bruh) = read_to_string("f4.inu") {
        bruh
    }
    else {
        crate::SysThrow!("I can't find this fucking file!\n\tYOU HAD ONE JOB!")
    };     

    let (mvec, vvec) = pest_parse(&txt);

    
    // for i in mvec.unwrap().into_inner() {
    //     println!("---> {:#?}\n", i);
    // };

    let MSEC = PARSER::make_msec(mvec);
    // for i in &MSEC {
    //     println!("---> {:#?}\n", i);
    // };

    let (sh, hh) = PARSER::calloc(vvec);
    new_check_exec_line(&MSEC, sh, hh);

}
