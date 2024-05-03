use colored::Colorize;
use my_modules::defkeys::*;

use crate::fetch_data::{fetch_bool, fetch_num, fetch_str};
use crate::Compare;

use std::collections::HashMap;

pub fn print_line(
    line: &Vec<Builtins>,
    stack_hash: &HashMap<String, Builtins>,
    heap_hash: &HashMap<String, Builtins>,
    reg_hash: &HashMap<String, Builtins>,
    isCool: bool
) -> String {
    match &line[1] {
        Builtins::D_type(D_type::str(strn)) => {
            println!(":> {}", strn.truecolor(150, 150, 100).bold());
            return strn.to_string();
        }
        Builtins::D_type(D_type::int(i)) => {
            println!(":> {}", i.to_string().truecolor(150, 150, 100).bold());
            return i.to_string();
        }
        Builtins::D_type(D_type::bool(b)) => {
            println!(":> {}", b.to_string().truecolor(150, 150, 100).bold());
            return b.to_string();
        }

        Builtins::ID(id) => {
            if stack_hash.contains_key(id) {
                print_var(id.to_string(), stack_hash, &isCool);
            } else {
                print_var(id.to_string(), heap_hash, &isCool);
            };
            return id.to_string();
        },

        Builtins::REGISTER(reg_id) => {
            if reg_hash.contains_key(reg_id) {
                print_var(reg_id.to_string(), reg_hash, &isCool);
            }
            else {
                crate::Throw!( format!("The following register is empty/uninitialized > {}", reg_id));
            }
            return reg_id.to_string();
        },

        Builtins::Expr { exp_type: ExpType::MATH_EXP, expr } => {
            let ans =  crate::ARITHMETIC::perf_math(expr, stack_hash, heap_hash, reg_hash, false).to_string();
            println!(":> {}", ans.truecolor(150, 150, 100).bold());
            return ans;
        },

        Builtins::Expr { exp_type: ExpType::CONDITION, expr: condition } => {
            let eval_expr = Compare::eval_condition(condition, stack_hash, heap_hash, reg_hash)
                .unwrap()
                .to_string();
            
            println!(":> {}", eval_expr.truecolor(150, 150, 100).bold());
            return eval_expr;
        },
        Builtins::Expr { exp_type: ExpType::LOGIC_EXP, expr: condition } => {
            let eval_expr = Compare::eval_condition(condition, stack_hash, heap_hash, reg_hash)
                .unwrap()
                .to_string();

            println!(":> {}", eval_expr.truecolor(150, 150, 100).bold());
            return eval_expr;
        },


        hmm => crate::Throw!(format!("PRNT:> No variable named {:?}", hmm)),
    }
}


fn print_var(var_nam: String, mem_hash: &HashMap<String, Builtins>, isCool: &bool) {
    let dat = match mem_hash.get(&var_nam) {
        Some(stuff) => stuff,
        None => crate::Throw!(format!("No variable named {:?}", var_nam)),
    };

    if let Ok(dat) = fetch_num(dat) {
        if *isCool {
            println!("{:?} contains {}", var_nam, dat.to_string().green().bold());
        }
        else {
            println!(":> {}", dat.to_string().truecolor(150, 150, 100).bold());
        }
    } 
    else if let Ok(dat) = fetch_str(dat) {
        if *isCool {
            println!("{:?} contains {}", var_nam, dat.to_string().green().bold());
        }
        else {
            println!(":> {}", dat.to_string().truecolor(150, 150, 100).bold());
        }
    } 
    else {
        if *isCool {
            println!("{:?} contains {}", var_nam, fetch_bool(dat).unwrap().to_string().green().bold());
        }
        else {
            println!(":> {}", fetch_bool(dat).unwrap().to_string().truecolor(150, 150, 100).bold());
        }
    };
}


