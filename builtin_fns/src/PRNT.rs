use colored::Colorize;
use my_modules::defkeys::*;

use crate::fetch_data::{fetch_bool, fetch_num, fetch_str, get_val};

use std::collections::HashMap;

pub fn print_line(
    line: &Vec<Builtins>,
    stack_hash: &HashMap<String, Builtins>,
    heap_hash: HashMap<String, Builtins>,
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
                print_var(id.to_string(), stack_hash)
            } else {
                print_var(id.to_string(), &heap_hash)
            };
            return id.to_string();
        },

        Builtins::Expr { exp_type: ExpType::MATH_EXP, expr } => {
            let ans =  crate::ARITHMETIC::perf_math(&expr, &stack_hash, &heap_hash, false).to_string();
            println!(":> {}", ans.truecolor(150, 150, 100).bold());
            return ans;
        },

        Builtins::Expr { exp_type: ExpType::CONDITION, expr: condition } => {
            let lhs = get_val(condition.get(0).unwrap(), &stack_hash, &heap_hash).unwrap();
                let rhs = get_val(condition.get(2).unwrap(), &stack_hash, &heap_hash).unwrap();

                let oper = condition.get(1).unwrap();
                let ans = lhs.partial_cmp(&Builtins::D_type(D_type::int(100))).unwrap();
                
                let eval_expr = match (oper, ans) {
                    ( Builtins::CMP(CompOp::EQUAL), std::cmp::Ordering::Equal ) => lhs == rhs,
                    ( Builtins::CMP(CompOp::LESS), std::cmp::Ordering::Less ) => lhs < rhs,
                    ( Builtins::CMP(CompOp::GREATER), std::cmp::Ordering::Greater ) => lhs > rhs,
                    _ => false
                }.to_string();

            println!(":> {}", eval_expr.truecolor(150, 150, 100).bold());
            return eval_expr;
        },


        hmm => crate::Throw!(format!("No variable named {:?}", hmm)),
    }
}


fn print_var(var_nam: String, mem_hash: &HashMap<String, Builtins>) {
    let dat = match mem_hash.get(&var_nam) {
        Some(stuff) => stuff,
        None => crate::Throw!(format!("No variable named {:?}", var_nam)),
    };

    if let Ok(dat) = fetch_num(dat) {
        println!("{:?} contains {}", var_nam, dat.to_string().green().bold());
    } 
    else if let Ok(dat) = fetch_str(dat) {
        println!("{:?} contains {}", var_nam, dat.to_string().green().bold());
    } 
    else {
        println!( "{:?} contains {}",
            var_nam, fetch_bool(dat).unwrap().to_string().green().bold()
        );
    };
}

