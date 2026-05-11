use SysBase::defkeys::*;
use SysBase::memory_layout::*;
use SysBase::mem_alloc::{insert_to_mem, mutate_mem};
// use SysBase::reg_alloc::{insert_to_reg, mutate_reg};

// use core_utils::ARITHMETIC::perf_math;   // Acutal Location
// use core_utils::Compare;

use crate::Compare;
use crate::ARITHMETIC::perf_math;

use crate::Throw;
use SysBase::fetch_data::fetch_bool;

// use crate::Input;

// ! 8/5/26 --12:20AM-- Time for Big change (one of the biggest ever)
// ! HASHMAPS Y'ALL WILL BE MISSED
pub fn check_exec_line(
    block: &Vec<Builtins>,
    mem: &mut Memory,
    env: &mut Env,
) {
    let mut line_num: i32 = 0;
    loop {
        let inp_line = match block.get(line_num as usize) {
            Some(x) => x,
            None => Throw!("Can't go that backward/forward , bitch"),
        };
        // let inp_line = inp_block.unwrap_expr_vec().unwrap();
        // println!("---> {:?}\n\n", inp_line);

        match &inp_line {
            Builtins::Loop(Loop::WHILE_LOOP(
                WHILE_LOOP { condition, block }
            ))
            => {
                // OPTIMIZATION SUGGESTION (22/4/26) spawn another thread to only check if the loop condition is satisfied.
                let condition = &condition[0];

                let mut condition_isTrue = match condition.get_expression_type() {
                    Ok(_) => {
                        Compare::eval_condition(condition, mem, env)
                            .unwrap()
                    }
                    Err(_) => fetch_bool(condition).unwrap(),
                };

                while condition_isTrue {
                    execute_line(&Builtins::InnerScope(block.to_owned()), mem, env);

                    // same code as before
                    condition_isTrue = match condition.get_expression_type() {
                        Ok(_) => {
                            Compare::eval_condition(condition, mem, env)
                                .unwrap()
                        }
                        Err(_) => fetch_bool(condition).unwrap(),
                    };

                } 
            }

            _ => {
                execute_line(&inp_line, mem, env)
            }
        };

        line_num += 1;
        if line_num >= block.len() as i32 {
            break;
        }
    }
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn execute_line(
    inp_expr: &Builtins,
    mem: &mut Memory,
    env: &mut Env,
) {
    match inp_expr {
        // maths expression  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        Builtins::Expr {
            exp_type: ExpType::MATH_EXP,
            expr,
        } => {
            perf_math(expr, mem, env, true);
        }

        // standard functions ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        Builtins::Expr {
            exp_type: ExpType::STDFN_EXP,
            expr,
        } => {
            let _ = match &expr[0] {
                Builtins::Std_fns(Std_fns::PRINT) => {
                    crate::PRINT::print_line(expr, mem, env, false, false)
                }

                Builtins::Std_fns(Std_fns::PRINT_NEWLINE) => {
                    crate::PRINT::print_line(expr, mem, env, true, false)
                }

                Builtins::Std_fns(Std_fns::PRINT_COOL) => {
                    crate::PRINT::print_line(expr, mem, env, false, true)
                }
                Builtins::Std_fns(Std_fns::SINPUT) => {
                    println!("  ADVICE---> It is adviced to use 'SINPUT' where it's meant to be.")
                }
                _ => Throw!("exec::expr -> Expected something good"),
            };
        }

        // -------IF---------------------ELIF------------------ELSE------------------------------------
        // IF~~~~
        Builtins::Expr {
            exp_type: ExpType::IF_EXP,
            expr: if_exp,
        } => {
            // format of if_exp => [> Expr { exp_type: dtype/condition/logic, expr: [..] }, Expr { exp_type: __, expr: [..] } <]
            let if_condition = &if_exp[0];

            let if_condition_isTrue = match if_exp[0].get_expression_type() {
                    Ok(_) => {
                        Compare::eval_condition(if_condition, mem, env)
                            .unwrap()
                    }
                    Err(_) => fetch_bool(if_condition).unwrap(),
                };

            if if_condition_isTrue {
                let exp_to_parse = &if_exp[1];
                return execute_line(exp_to_parse, mem, env);
            }
        }

        // IF-ELSE ~~~~~~~~~~~~~~~~~~
        Builtins::Expr {
            exp_type: ExpType::IF_ELSE_EXP,
            expr: ifelse_exp,
        } => {
            let if_exp = ifelse_exp[0].unwrap_expr_vec().unwrap();
            let else_exp = ifelse_exp[1].unwrap_expr_vec().unwrap();

            let isTrue = match if_exp[0].get_expression_type() {
                // The condition
                Ok(_) => {
                    Compare::eval_condition(&if_exp[0], mem, env).unwrap()
                }
                Err(_) => fetch_bool(&if_exp[0]).unwrap(),
            };

             if isTrue {
                execute_line(&if_exp[1], mem, env)
            } else {
                execute_line(&else_exp[0], mem, env)
            };
        }
        // IF-ELSE-ELIF ~~~~~~~~~~~~~~~~~~
        Builtins::Expr {
            exp_type: ExpType::IF_ELIF_EXP,
            expr: ifelif_exp,
        } => {
            let if_exp = ifelif_exp[0].unwrap_expr_vec().unwrap();
            let else_exp = ifelif_exp.last().unwrap().unwrap_expr_vec().unwrap();

            let isTrue = match if_exp[0].get_expression_type() {
                // The condition
                Ok(_) => {
                    Compare::eval_condition(&if_exp[0], mem, env).unwrap()
                }
                Err(_) => fetch_bool(&if_exp[0]).unwrap(),
            };

            if isTrue {
                execute_line(&if_exp[1], mem, env);
            } else {
                let mut shouldRunElse = true; // executing the ELIF and ELSE block

                for elif_exp in ifelif_exp[1..ifelif_exp.len() - 1].iter() {
                    let elif_exp = elif_exp.unwrap_expr_vec().unwrap();

                    let isElifTrue = match elif_exp[0].get_expression_type() {
                        // The condition
                        Ok(_) => Compare::eval_condition(&elif_exp[0], mem, env)
                        .unwrap(),
                        Err(_) => fetch_bool(&if_exp[0]).unwrap(),
                    };

                    if isElifTrue {
                        execute_line(&elif_exp[1], mem, env);
                        shouldRunElse = false;
                        break;
                    };
                }
                if shouldRunElse {
                    execute_line(&else_exp[0], mem, env);
                };
            }

            // Only executed if elif and if dont execute
        }

        // MEMORY INSTRUCTIONS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        Builtins::Expr {
            exp_type: ExpType::MEM_INST_EXP,        // MOV , DEL
            expr,
        } => {
            match &expr[0] {
                // IMPORTANT ------------------------------------------------------
                Builtins::MemInst(MemInst::MOV) => {
                    match &expr[1] {
                        Builtins::ID(_) => {
                            // Input/New  = expr[2]
                            match &expr[2] {
                                // resolve the id internally and assign its value to target variable
                                Builtins::ID(_) => mutate_mem(&expr, mem, env),

                                Builtins::D_type(_) => {
                                    insert_to_mem(&expr, mem, env, &expr[2])
                                }

                                Builtins::Expr {
                                    exp_type: ExpType::MATH_EXP,
                                    expr: math_expr,
                                } => {
                                    // returns data of type builtins::d_type
                                    let math_ans = perf_math(
                                        math_expr,
                                        mem,
                                        env,
                                        false,
                                    );
                                    let math_buff = match math_ans {
                                        Number::int(i) => Builtins::D_type(D_type::int(i)),
                                        Number::float(f) => Builtins::D_type(D_type::float(f))
                                    };
                                    
                                    insert_to_mem(&expr, mem, env, &math_buff)
                                }

                                Builtins::Expr {
                                    exp_type: ExpType::STDFN_EXP,
                                    expr: std_expr,
                                } => {
                                    let input_buff = match &std_expr[0] {
                                                Builtins::Std_fns(Std_fns::SINPUT) => crate::Input::get_parsed_inp(&std_expr),
                                                other_fn => Throw!(format!("The following fucntion doesn't have a return type --> {:?}", other_fn))
                                            };
                                    insert_to_mem(&expr, mem, env, &input_buff)
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
                    SysBase::mem_alloc::remove_from_mem(&expr, mem, env);   
                    //? its that easy now !
                }

                _ => Throw!("I threw up in execute_line"),
            }
        }

        Builtins::Expr {
            exp_type: ExpType::LOCAL_VAR_MAKE,
            expr: local_vmake,
        } => {
            println!("\tbullshit{:?}", local_vmake);
        }

        Builtins::InnerScope( 
            InnerScope { 
                inner_vsec,
                block: code_block,
                scope,
            }
        ) => {

            if inner_vsec.is_some() {
                for var in inner_vsec.as_ref().unwrap().iter() {
                    let var_exp = var.unwrap_expr_vec().unwrap(); // [ID("__"), Dtype(__)]

                    let name: String;

                    let is_mutable = match &var_exp[0] {
                        Builtins::ID(id) => {
                            if id.starts_with('?') {
                                name = id[1..].to_string(); 
                                true
                            }
                            else{
                                name = id.to_string();
                                false
                            }
                        },
                        _ => Throw!("Juswt a tiny boi")
                    };

                // Allocation inside inners
                    let value = var_exp[1].to_data(scope.clone(), is_mutable);
                    let ptr = mem.alloc(value);
                    env.insert(name, ptr);

                }
            };
            check_exec_line(code_block, mem, env);
        }

        //ERROR HANDLING-----------------------------------------------------------------------------------------------------------------
        //ERROR HANDLING-----------------------------------------------------------------------------------------------------------------
        Builtins::Comment => (),
        bruh => Throw!(format!("UNIMPLEMENTED FUNCTIONALITY ==> {:?}", bruh)),
    };
}