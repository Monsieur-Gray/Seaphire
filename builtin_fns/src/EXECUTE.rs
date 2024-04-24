use std::cmp::Ordering;

use my_modules::defkeys::*;
use my_modules::mem_alloc::{mutate_mem, insert_to_mem};

use crate::ARITHMETIC::perf_math;
use crate::Throw;

pub fn new_check_exec_line(
    block: &Vec<Vec<Builtins>>,
    mut stack_hash:  std::collections::HashMap<String, Builtins>,
    mut heap_hash:   std::collections::HashMap<String, Builtins>,
) {
    let mut line_num: i32 = 0;
    loop {
        let inp_line: &Vec<Builtins> = block.get(line_num as usize).unwrap();

        match &inp_line[0] {

//Expressions:-------------------------------------------------------------------------------------------------------------------
    // maths expression  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        Builtins::Expr { exp_type: ExpType::MATH_EXP, expr} => {
            perf_math(expr, &stack_hash, &heap_hash, true);  
        },

    // standard functions ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        Builtins::Expr { exp_type: ExpType::STDFN_EXP, expr} => {       // For now , only PRNT exists
            let _ = crate::PRNT::print_line(expr, &stack_hash, heap_hash.clone() ); 
        },

    // MEMORY INSTRUCTIONS ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        Builtins::Expr { exp_type: ExpType::MEM_INST_EXP, expr } => {
            match expr[0] {     // IMPORTANT ------------------------------------------------------
                Builtins::MemInst(MemInst::MOV) => {
                    heap_hash = match &expr[2] {
                        Builtins::Operation(_) => {
                            let math_buff = Builtins::D_type(
                                    D_type::float(perf_math(&expr[2..].to_vec(), &stack_hash, &heap_hash, false))
                                ) ;
                            insert_to_mem(&expr, heap_hash, math_buff)
                        },
    
                        Builtins::D_type(_) | Builtins::ID(_) => mutate_mem(&expr, &stack_hash, heap_hash),
    
                        Builtins::Expr { exp_type: ExpType::MATH_EXP, expr} => {
                            let math_buff = Builtins::D_type( D_type::float(
                                perf_math(expr, &stack_hash, &heap_hash, false)
                            ));
                            insert_to_mem(&expr, heap_hash, math_buff)
                        },
                        
                        _ => Throw!("The expression / function doesn't have a return type!")
                    }; 
                },
                
                Builtins::MemInst(MemInst::DEL) => {
                    let var = &crate::fetch_data::fetch_str(&expr[1]).unwrap();
                    if stack_hash.contains_key(var) {
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
            if condition.len() == 1 {
                if crate::fetch_data::fetch_bool(condition.get(0).unwrap()).unwrap(){
                    if num < &0 { line_num += num - 1 }
                    else { line_num += num}
                };
            }
            else {      //if we have a conditional expression, parse it
                
                let lhs = get_val(condition.get(0).unwrap(), &stack_hash, &heap_hash).unwrap();
                let rhs = get_val(condition.get(2).unwrap(), &stack_hash, &heap_hash).unwrap();

                let oper = condition.get(1).unwrap();
                let ans = lhs.partial_cmp(&rhs).unwrap();
                
                let eval_expr = match (oper, ans) {
                    ( Builtins::CMP(CompOp::EQUAL), Ordering::Equal ) => true,
                    ( Builtins::CMP(CompOp::LESS), Ordering::Less ) => true,
                    ( Builtins::CMP(CompOp::GREATER), Ordering::Greater ) => true,
                    _ => false
                };

                if eval_expr {
                    if num < &0 { 
                        line_num += num - 1;
                    }
                    else { 
                        line_num += num;
                    }
                }
                
            };
        },

//ERROR HANDLING-----------------------------------------------------------------------------------------------------------------
            Builtins::Comment => (),
            bruh => Throw!(format!("UNIMPLEMENTED FUNCTIONALITY ==> {:?}", bruh))
        };

        line_num += 1;
        if line_num >= block.len() as i32{ break }
    }
}



fn get_val(var: &Builtins,
    stack_hash: &std::collections::HashMap<String, Builtins>,
    heap_hash: &std::collections::HashMap<String, Builtins> ) -> Option<Builtins>
{
    let a = match var {
        Builtins::D_type(_) => var,
        Builtins::ID(id) => {
            if let Some(v) = stack_hash.get( id ) { v }
            else if let Some(v) = heap_hash.get( id ){ v }
            else { return None; }
        },
        _ => Throw!("What in actual fuck is this")
    };
    
    return Some(a.clone());
}
