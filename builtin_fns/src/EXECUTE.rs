
use my_modules::defkeys::*;
use my_modules::mem_alloc::{mutate_mem, insert_to_mem};
use my_modules::reg_alloc::{mutate_reg, insert_to_reg};

use crate::ARITHMETIC::perf_math;
use crate::Compare;

use my_modules::fetch_data::fetch_bool;
use crate::Throw;

pub fn check_exec_line(
    block: &Vec<Builtins>,
    mut stack_hash:  std::collections::HashMap<String, Value>,
    mut heap_hash:   std::collections::HashMap<String, Value>,
    mut reg_hash:  std::collections::HashMap<String, Value>,
) -> [std::collections::HashMap<String, Value>; 3]
{
    let mut line_num: i32 = 0;
    loop {

        let inp_line =  match block.get(line_num as usize) {
            Some( x ) => x,
            None => Throw!("Can't go that backward/forward , bitch")
        };
        // let inp_line = inp_block.unwrap_expr_vec().unwrap(); 
        // println!("---> {:?}\n\n", inp_line);

        match &inp_line {
            Builtins::JUMPIF { n: num, expr: condition } =>  {

                let condition = &condition[0];
                // println!("exe> {:?}", condition);

                let condition_isTrue = match condition.get_expression_type() {
                    Ok(_) => Compare::eval_condition(
                        &condition, 
                        &stack_hash, &heap_hash, &reg_hash
                    ).unwrap(),
                    Err(_) => fetch_bool(&condition).unwrap()
    
                };
                if condition_isTrue{ line_num += num - 1; }
                else { line_num += 0; }                
            },

            _ => { [stack_hash, heap_hash, reg_hash] = execute_line(
            &inp_line, 
                    stack_hash, heap_hash, reg_hash
            )}
        };

        line_num += 1;
        if line_num >= block.len() as i32 { 
            break [stack_hash, heap_hash, reg_hash];
        }
    }
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn execute_line(
    inp_expr: &Builtins,
    mut stack_hash: std::collections::HashMap<String, Value>,
    mut heap_hash: std::collections::HashMap<String, Value>,
    mut reg_hash: std::collections::HashMap<String, Value>,
) -> [std::collections::HashMap<String, Value>; 3] {
    match inp_expr {
        // maths expression  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        Builtins::Expr {
            exp_type: ExpType::MATH_EXP,
            expr,
        } => {
            perf_math(expr, &stack_hash, &heap_hash, &reg_hash, true);
        }

        // standard functions ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        Builtins::Expr {
            exp_type: ExpType::STDFN_EXP,
            expr,
        } => {
            // For now , only PRNT exists
            let _ = match &expr[0] {
                Builtins::Std_fns(Std_fns::PRNT_COOL) => {
                    crate::PRNT::print_line(expr, &stack_hash, &heap_hash, &reg_hash, true)
                }
                Builtins::Std_fns(Std_fns::PRNT_PLAIN) => {
                    crate::PRNT::print_line(expr, &stack_hash, &heap_hash, &reg_hash, false)
                }
                Builtins::Std_fns(Std_fns::SINPUT) => {
                    println!("  ADVICE---> It is adviced to use 'SINPUT' where it's meant to be.")
                }
                _ => Throw!("exec::expr -> Expected something good"),
            };
        },

    // -------IF---------------------ELIF------------------ELSE------------------------------------
            // IF~~~~
            Builtins::Expr { exp_type: ExpType::IF_EXP, expr: if_exp } => {
                // panic!("\nfadsfsdf --> {:?}\n", if_exp[0].unwrap_expr_vec().unwrap());
            let if_condition = if_exp[0].unwrap_expr_vec().unwrap();

                let isTrue = match if_condition[0].get_expression_type() {    // The condition
                    Ok(_) => Compare::eval_condition(
                        &if_condition[0].clone(), &stack_hash, &heap_hash, &reg_hash,
                        ).unwrap(),
                    Err(_) => fetch_bool(&if_condition[0]).unwrap(),
                };
                                
                if isTrue {
                    let exp_to_parse = &if_exp[1];
                    return execute_line(exp_to_parse, stack_hash, heap_hash, reg_hash);
                }
            },

        // IF-ELSE ~~~~~~~~~~~~~~~~~~
            Builtins::Expr { exp_type: ExpType::IF_ELSE_EXP, expr: ifelse_exp } => {
                let if_exp = ifelse_exp[0].unwrap_expr_vec().unwrap();
                let else_exp = ifelse_exp[1].unwrap_expr_vec().unwrap();

                let isTrue = match if_exp[0].get_expression_type() {    // The condition
                    Ok(_) => Compare::eval_condition(
                        &if_exp[0], &stack_hash, &heap_hash, &reg_hash,
                        ).unwrap(),
                    Err(_) => fetch_bool(&if_exp[0]).unwrap(),
                };
                
                [stack_hash, heap_hash, reg_hash] = if isTrue {
                    println!("\tif> {:?}", &if_exp[1]);
                    execute_line(&if_exp[1], stack_hash, heap_hash, reg_hash)
                }
                else {
                    println!("\telse> {:?}", &else_exp[0]);
                    execute_line(&else_exp[0], stack_hash, heap_hash, reg_hash)                    
                };
            },
        // IF-ELSE-ELIF ~~~~~~~~~~~~~~~~~~
            Builtins::Expr { exp_type: ExpType::IF_ELIF_EXP, expr: ifelif_exp } => {
                let if_exp = ifelif_exp[0].unwrap_expr_vec().unwrap();
                let else_exp = ifelif_exp
                        .last().unwrap().unwrap_expr_vec().unwrap();

                let isTrue = match if_exp[0].get_expression_type() {    // The condition
                    Ok(_) => Compare::eval_condition(
                        &if_exp[0], &stack_hash, &heap_hash, &reg_hash,
                        ).unwrap(),
                    Err(_) => fetch_bool(&if_exp[0]).unwrap(),
                };
                
                if isTrue {
                    [stack_hash, heap_hash, reg_hash] = execute_line(&if_exp[1], stack_hash, heap_hash, reg_hash);
                }
                else {
                    let mut shouldRunElse = true;       // executing the ELIF and ELSE block

                    for elif_exp in ifelif_exp[1..ifelif_exp.len()-1].iter() {
                        let elif_exp = elif_exp.unwrap_expr_vec().unwrap();

                        let isElifTrue = match elif_exp[0].get_expression_type() {    // The condition
                            Ok(_) => Compare::eval_condition(
                                &elif_exp[0].clone(), &stack_hash, &heap_hash, &reg_hash,
                                ).unwrap(),
                            Err(_) => fetch_bool(&if_exp[0]).unwrap(),
                        };

                        if isElifTrue {
                            [stack_hash, heap_hash, reg_hash] = execute_line(&elif_exp[1], stack_hash, heap_hash, reg_hash);
                            shouldRunElse = false;
                            break
                        };

                    };
                    if shouldRunElse {
                        [stack_hash, heap_hash, reg_hash] = execute_line(&else_exp[0], stack_hash, heap_hash, reg_hash);
                    };
                }

                
                // Only executed if elif and if dont execute
                
            },


    // MEMORY INSTRUCTIONS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        Builtins::Expr {
            exp_type: ExpType::MEM_INST_EXP,
            expr,
        } => {
            match &expr[0] {
                // IMPORTANT ------------------------------------------------------
                Builtins::MemInst(MemInst::MOV) => {
                    match &expr[1] {
                        Builtins::ID(_) => {
                            heap_hash = match &expr[2] {
                                // Input/New value
                                Builtins::ID(_) => {
                                    mutate_mem(&expr, &stack_hash, heap_hash)
                                },

                                Builtins::D_type(_) => {
                                    let data_to_insert = expr[2].clone();
                                    insert_to_mem(&expr, heap_hash, data_to_insert)
                                }

                                Builtins::Expr {
                                    exp_type: ExpType::MATH_EXP,
                                    expr: math_expr,
                                } => {
                                    let math_buff = Builtins::D_type(D_type::float(perf_math(
                                        math_expr,
                                        &stack_hash,
                                        &heap_hash,
                                        &reg_hash,
                                        false,
                                    )));
                                    insert_to_mem(&expr, heap_hash, math_buff)
                                }

                                Builtins::Expr {
                                    exp_type: ExpType::STDFN_EXP,
                                    expr: std_expr,
                                } => {
                                    let input_buff = match &std_expr[0] {
                                                Builtins::Std_fns(Std_fns::SINPUT) => crate::Input::get_parsed_inp(&std_expr),
                                                other_fn => Throw!(format!("The following fucntion doesn't have a return type --> {:?}", other_fn))
                                            };
                                    insert_to_mem(&expr, heap_hash, input_buff)
                                }

                                _ => {
                                    Throw!("The expression / function doesn't have a return type!")
                                }
                            };
                        }
                        Builtins::REGISTER(_) => {
                            reg_hash = match &expr[2] {
                                // Input/New value
                                Builtins::D_type(_) | Builtins::ID(_) => {
                                    mutate_reg(&expr, &stack_hash, reg_hash)
                                }

                                Builtins::Expr {
                                    exp_type: ExpType::STDFN_EXP,
                                    expr: std_expr,
                                } => {
                                    let input_buff = match &std_expr[0] {
                                                Builtins::Std_fns(Std_fns::SINPUT) => crate::Input::get_parsed_inp(&std_expr),
                                                other_fn => Throw!(format!("The following fucntion doesn't have a return type --> {:?}", other_fn))
                                            };
                                    insert_to_reg(&expr, reg_hash, input_buff)
                                }

                                Builtins::Expr {
                                    exp_type: ExpType::MATH_EXP,
                                    expr: math_expr,
                                } => {
                                    let math_buff = Builtins::D_type(D_type::float(perf_math(
                                        math_expr,
                                        &stack_hash,
                                        &heap_hash,
                                        &reg_hash,
                                        false,
                                    )));
                                    insert_to_reg(&expr, reg_hash, math_buff)
                                }

                                _ => {
                                    Throw!("The expression / function doesn't have a return type!")
                                }
                            };
                        }
                        _ => (),
                    }
                }

                Builtins::MemInst(MemInst::DEL) => {
                    let var = &my_modules::fetch_data::fetch_str(&expr[1]).unwrap();
                    if reg_hash.contains_key(var) {
                        reg_hash.remove(var);
                    } else if stack_hash.contains_key(var) {
                        stack_hash.remove(var);
                    } else if heap_hash.contains_key(var) {
                        heap_hash.remove(var);
                    } else {
                        Throw!(format!("FREE_MEM ::> No variable named '{}'", var));
                    }
                },

                _ => Throw!("I threw up in execute_line")
            }
        },

        Builtins::Expr { exp_type: ExpType::LOCAL_VAR_MAKE, expr: local_vmake } => {
            println!("\tbullshit{:?}", local_vmake);
        },


        Builtins::InnerScope { inner_vsec, block: code_block, scope } => {
            let mut new_stack: std::collections::HashMap<String, Value>  = stack_hash.clone(); 
            let mut new_heap: std::collections::HashMap<String, Value>  = heap_hash.clone();

            if inner_vsec.is_some() {
                for var in inner_vsec.as_ref().unwrap().iter() {
                    let var_exp = var.unwrap_expr_vec().unwrap();   // [ID("__"), Dtype(__)]
                    let new_value = var_exp[1] .to_value(scope.clone());

                    match &var_exp[0] {     // id
                        Builtins::ID( id ) => {
                            if id.starts_with('?') {
                                let new_id = id.get(1..).unwrap().to_string().replace("\'", "");
                                let _ = new_heap.insert(new_id, new_value);
                            }
                            else { 
                                let new_id = id.to_string().replace("\'", "");
                                let _ = new_stack.insert(new_id, new_value);
                            }
                        }, 
                        _ => Throw!("Juswt a tiny boi")
                    };
                };
            } ;    

            // [stack_hash, heap_hash, reg_hash] = check_exec_line(code_block, new_stack, new_heap, reg_hash);
            check_exec_line(code_block, new_stack, new_heap, reg_hash.clone());
        }

        //ERROR HANDLING-----------------------------------------------------------------------------------------------------------------
        //ERROR HANDLING-----------------------------------------------------------------------------------------------------------------
        Builtins::Comment => (),
        bruh => Throw!(format!("UNIMPLEMENTED FUNCTIONALITY ==> {:?}", bruh)),
    };

    return [stack_hash, heap_hash, reg_hash];
}

