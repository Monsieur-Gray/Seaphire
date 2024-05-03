use my_modules::defkeys::*;
use my_modules::mem_alloc::{mutate_mem, insert_to_mem};
use my_modules::reg_alloc::{mutate_reg, insert_to_reg};

use crate::fetch_data::fetch_bool;
use crate::ARITHMETIC::perf_math;
use crate::Throw;

pub fn check_exec_line(
    block: &Vec<Vec<Builtins>>,
    mut stack_hash:  std::collections::HashMap<String, Builtins>,
    mut heap_hash:   std::collections::HashMap<String, Builtins>,
    mut reg_hash:  std::collections::HashMap<String, Builtins>
) {
    let mut line_num: i32 = 0;
    loop {
        let inp_line: &Vec<Builtins> = block.get(line_num as usize).unwrap();
        // println!("---> {:?}", &inp_line);
        match &inp_line[0] {

//Expressions:-------------------------------------------------------------------------------------------------------------------
    // maths expression  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        Builtins::Expr { exp_type: ExpType::MATH_EXP, expr} => {
            perf_math(expr, &stack_hash, &heap_hash, &reg_hash, true);  
        },

    // standard functions ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        Builtins::Expr { exp_type: ExpType::STDFN_EXP, expr} => {       // For now , only PRNT exists
            match &expr[0] {
                Builtins::Std_fns(Std_fns::PRNT_COOL) => crate::PRNT::print_line(expr, &stack_hash, &heap_hash, &reg_hash, true),
                Builtins::Std_fns(Std_fns::PRNT_PLAIN) => crate::PRNT::print_line(expr, &stack_hash, &heap_hash, &reg_hash, false),
                Builtins::Std_fns(Std_fns::SINPUT) => crate::Input::get_input(Some("nigga tell".to_string())),
                _ => Throw!("exec::expr -> Expected something good")
            };
        },

    // MEMORY INSTRUCTIONS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        Builtins::Expr { exp_type: ExpType::MEM_INST_EXP, expr } => {
            match expr[0] {     // IMPORTANT ------------------------------------------------------
                Builtins::MemInst(MemInst::MOV) => {
                    match &expr[1] {
                    Builtins::ID(_) => {
                            heap_hash = match &expr[2] {        // Input/New value
                                Builtins::D_type(_) | Builtins::ID(_) => mutate_mem(&expr, &stack_hash, heap_hash),
            
                                Builtins::Expr { exp_type: ExpType::MATH_EXP, expr: math_expr} => {
                                    let math_buff = Builtins::D_type( D_type::float(
                                        perf_math(math_expr, &stack_hash, &heap_hash, &reg_hash, false)
                                    ));
                                    insert_to_mem(&expr, heap_hash, math_buff)
                                },
                                
                                _ => Throw!("The expression / function doesn't have a return type!")
                            };
                        },
                    Builtins::REGISTER(_) => {
                            reg_hash = match &expr[2] {        // Input/New value
                                Builtins::D_type(_) | Builtins::ID(_) => mutate_reg(&expr, &stack_hash, reg_hash),
            
                                Builtins::Expr { exp_type: ExpType::MATH_EXP, expr: math_expr} => {
                                    let math_buff = Builtins::D_type( D_type::float(
                                        perf_math(math_expr, &stack_hash, &heap_hash, &reg_hash, false)
                                    ));

                                    insert_to_reg(&expr, reg_hash, math_buff)
                                },
                                
                                _ => Throw!("The expression / function doesn't have a return type!")
                            };
                        },
                    _ => ()
                    }
                },
                
                Builtins::MemInst(MemInst::DEL) => {
                    let var = &crate::fetch_data::fetch_str(&expr[1]).unwrap();
                    if reg_hash.contains_key(var) {
                        reg_hash.remove(var);
                    }
                    else if stack_hash.contains_key(var) {
                        stack_hash.remove(var);
                    }
                    else if heap_hash.contains_key(var) {
                        heap_hash.remove(var);
                    }
                    else {
                        Throw!(format!("FREE_MEM ::> No variable named '{}'", var));
                    }
                },

                _ => continue
            }
        },

//CONTROL FLOW-----------------------------MUST BE CHANGED LATER!--------------------------------------------------------------------
        Builtins::JUMPIF { n: num, expr: condition } =>  {
            let condition_isTrue = match condition[0].get_expression_type() {
                Ok(_) => crate::Compare::eval_condition(condition, &stack_hash, &heap_hash, &reg_hash).unwrap(),
                Err(_) => fetch_bool(&condition[0]).unwrap()

            };

            if condition_isTrue{
                line_num += num - 1;
            }
            else { line_num += 0; }                
        },

//ERROR HANDLING-----------------------------------------------------------------------------------------------------------------
            Builtins::Comment => (),
            bruh => Throw!(format!("UNIMPLEMENTED FUNCTIONALITY ==> {:?}", bruh))
        };

        line_num += 1;
        if line_num >= block.len() as i32 { break }
    }
}



