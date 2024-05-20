#![allow(non_snake_case)]
#![allow(unused)]

use colored::Colorize;
use my_modules::defkeys::Builtins;
use my_modules::*;
use my_modules::PARSER::pest_parse;

use my_modules::PARSER;
use builtin_fns::EXECUTE::check_exec_line;

fn main() {
    use std::fs::read_to_string;
    let file_name = "f5scopes.inu";
    let txt = if let Ok(bruh) = read_to_string(&file_name) {
        bruh
    }
    else { 
        crate::SysThrow!("I can't find this fucking file!\n\tYOU HAD ONE JOB!")
    };     

    let (mvec, vvec) = pest_parse(&txt);

    // for i in vvec.unwrap().into_inner() {
    //     println!("---> {:#?}\n", i);
    // };

    let MSEC = PARSER::make_msec(mvec);
    // for i in MSEC {
    //     println!("---> {:?}\n", i);
    //     match i {
    //         Builtins::InnerScope { inner_vsec, block, scope } => {
    //             println!(" BLOCK :->>> {:?}\n", block);
    //             println!(" \tscope :->>> {:?}\n", scope);
    //         },
    //         _ => continue
    //     }
    // }

    let [sh, hh, regh] = PARSER::calloc(vvec);

    println!("~~~~~~~\tCURRENTLY RUNNING ->  {}~~~~~~~\n", file_name.purple().bold());
    check_exec_line(&MSEC, sh, hh, regh);
}



#[cfg(test)]
mod tests {
    use super::*;
    fn verify_given_file(file_int: &i32) -> Result<(), ()>{
        let file_name = format!("src/f{}.inu", file_int);
        let txt = if let Ok(bruh) = std::fs::read_to_string(file_name) {  bruh  }
        else {
            crate::SysThrow!("I can't find this fucking file!\n\tYOU HAD ONE JOB!")
        };     
    
        let (mvec, vvec) = pest_parse(&txt);
        let MSEC = PARSER::make_msec(mvec);
        let [sh, hh, regh] = PARSER::calloc(vvec);

        check_exec_line(&MSEC, sh, hh, regh);
        Ok(())
    }

    #[test]
    fn verify_codebase() {
        for p in 1..6 {
            let _ = verify_given_file(&p);
            println!("\t TEST - {:?} WAS SUCCESSFUL!", p);
        }
    }
}