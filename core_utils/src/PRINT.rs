use colored::Colorize;
use SysBase::defkeys::*;

use SysBase::fetch_data::{fetch_bool, fetch_num, fetch_str};

use std::collections::HashMap;

pub fn print_line(
    line: &Vec<Builtins>,
    stack_hash: &HashMap<String, Value>,
    heap_hash: &HashMap<String, Value>,
    reg_hash: &HashMap<String, Value>,
    is_cool: bool
)  {
    match &line[1] {
        Builtins::D_type(D_type::str(strn)) => {
            println!(":> {}", strn.replace('\'', "").truecolor(150, 150, 100).bold());
        }
        Builtins::D_type(D_type::int(i)) => {
            println!(":> {}", i.to_string().truecolor(150, 150, 100).bold());
        }
        Builtins::D_type(D_type::bool(b)) => {
            println!(":> {}", b.to_string().truecolor(150, 150, 100).bold());
        }

        Builtins::ID(id) => {
            if stack_hash.contains_key(id) {
                print_var(id.to_string(), stack_hash, &is_cool);
            } else {
                print_var(id.to_string(), heap_hash, &is_cool);
            };
        },

        Builtins::REGISTER(reg_id) => {
            if reg_hash.contains_key(reg_id) {
                print_var(reg_id.to_string(), reg_hash, &is_cool);
            }
            else {
                crate::Throw!( format!("The following register is empty/uninitialized > {}", reg_id));
            }
        },

        hmm => crate::Throw!(format!("PRINT:> No variable named {:?}", hmm)),
    }
}


fn print_var(var_nam: String, mem_hash: &HashMap<String, Value>, is_cool: &bool) {
    let dat = match mem_hash.get(&var_nam) {
        Some(stuff) => stuff,
        None => crate::Throw!(format!("No variable named {:?}", var_nam)),
    };

    if let Ok(dat) = fetch_num(&dat.value) {
        if *is_cool {
            println!("{:?} contains {}", var_nam, dat.to_string().green().bold());
        }
        else {
            println!(":> {}", dat.to_string().truecolor(150, 150, 100).bold());
        }
    } 
    else if let Ok(dat) = fetch_str(&dat.value) {
        if *is_cool {
            println!("{:?} contains {}", var_nam, dat.to_string().green().bold());
        }
        else {
            println!(":> {}", dat.to_string().truecolor(150, 150, 100).bold());
        }
    } 
    else {
        if *is_cool {
            println!("{:?} contains {}", var_nam, fetch_bool(&dat.value).unwrap()
                .to_string().green().bold());
        }
        else {
            println!(":> {}", fetch_bool(&dat.value).unwrap()
                .to_string().truecolor(150, 150, 100).bold());
        }
    };
}


//// Changes 2025
/*
Taking conditions or math_expressions as input, evaluating them and then printing is too much hastle 
 and high-level. Do it fucking yourself!
*/