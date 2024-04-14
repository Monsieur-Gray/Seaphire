use std::collections::HashMap;

use my_modules::defkeys::*;
use my_modules::mem_alloc::{mutate_mem, insert_to_mem};

use crate::ARITHMETIC::perf_math;
use crate::fetch_data::{fetch_str, fetch_num};

use crate::Throw;

pub fn check_exec_line(
    block: &Vec<Vec<Builtins>>,
    mut stack_hash: HashMap<String, Builtins>,
    mut heap_hash:HashMap<String, Builtins>,
) {
    let mut line_num: i32 = 0;
    loop {
        let inp_line: &Vec<Builtins> = block.get(line_num as usize).unwrap();

        match &inp_line[0] {
            Builtins::Operation(_) => {
                perf_math(&inp_line, &stack_hash, &heap_hash, true);
            },
                
//STANDARD FUNCTIONS-------------------------
        Builtins::Std_fns(func) => match func {
            Std_fns::PRNT => {
                let _ = crate::PRNT::print_line(&inp_line, &stack_hash, heap_hash.clone() ); 
            },
            _ => println!("quecera")        
        },
                    
//MEMORY INSTRUCTIONS-------------------------
        Builtins::MemInst(inst) => match inst {     // IMPORTANT ------------------------------------------------------
            MemInst::MOV => {
                heap_hash = match &inp_line[2] {
                    Builtins::Operation(_) => {
                        let math_buff = Builtins::D_type(D_type::float(perf_math(&inp_line[2..].to_vec(), &stack_hash, &heap_hash, false))) ;
                        insert_to_mem(&inp_line, heap_hash, math_buff)
                    },
                    Builtins::D_type(_) | Builtins::ID(_) => mutate_mem(&inp_line, &stack_hash, heap_hash),

                    _ => Throw!("The expression / function doesn't have a return type!")
                }
                
            },
            MemInst::DEL => {
                let var = &fetch_str(&inp_line[1]).unwrap();
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
            
            _ => println!("Pass it bro!")
        },

//CONTROL FLOW-------------------------
        Builtins::ControlFlow(inst) => match inst {
            ControlFlow::JUMP => {  // JUMPING JAPAACK!
                if let Ok(num) = fetch_num( inp_line.get(1).unwrap() ) {
                    if num < 0.0 { line_num += num as i32 - 1 }
                    else { line_num += num as i32}
                }
            }
        },


//ERROR HANDLING--------------------------------
            Builtins::Comment => (),
            _ => Throw!("UNIMPLEMENTED FUNCTIONALITY")
        };

        line_num += 1;
        if line_num >= block.len() as i32{ break }
    }
}