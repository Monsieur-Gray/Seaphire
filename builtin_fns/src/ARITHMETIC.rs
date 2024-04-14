use std::collections::HashMap;

use my_modules::defkeys::*;
use my_modules::fetch_data::fetch_Univ;

use crate::fetch_data::fetch_num;
use crate::Throw;

fn caxy_add(operands: &Vec<Builtins>,
    stack_hash: &HashMap<String, Builtins>,
    heap_hash: &HashMap<String, Builtins>
) -> f32 {
    let answer: f32 = operands.iter().map(|i| {
        match i {
            Builtins::D_type(_) => fetch_num(i).unwrap(),
            Builtins::ID(var) => {
                fetch_num(
                    stack_hash.get(var).or_else(|| heap_hash.get(var))
                        .unwrap_or_else(|| panic!("caxy-add ::> cant find the variable you asked for"))
                ).unwrap()
            },
            other => Throw!(format!("ADD_2 isnt made for {:?}", other))
        }
    }).sum();
    
    return answer;
}

fn caxy_sub(operands: &Vec<Builtins>,               //SUBTRACTION
    stack_hash: &HashMap<String, Builtins>,
    heap_hash: &HashMap<String, Builtins>
) -> f32 {
    
    let mut answer: f32 = 2.0 * fetch_Univ(&operands[0]).unwrap();
    operands.iter().skip(1).for_each(|i| {
        let num = match i {
            Builtins::D_type(_) => fetch_num(i).unwrap(),
            Builtins::ID(var) => {
                fetch_num(
                    stack_hash.get(var).or_else(|| heap_hash.get(var))
                        .unwrap_or_else(|| panic!("caxy-sub ::> Cannot find the variable"))

                    ).unwrap_or_else(|var| Throw!(format!("SUB isnt made for {:?}", var)) )
            },
            other => Throw!(format!("SUB isnt made for {:?}", other))
        };
        answer -= num;
    });
        
    return answer;
}

fn caxy_mul(operands: &Vec<Builtins>,
    stack_hash: &HashMap<String, Builtins>,
    heap_hash: &HashMap<String, Builtins>
) -> f32 {
    let mut answer: f32 = 1.0;
    for i in operands {
        let num = match i {
            Builtins::D_type(_) => fetch_num(i).unwrap(),
            Builtins::ID(var) => {
                let num = stack_hash.get(var)
                    .or_else(|| heap_hash.get(var))
                    .unwrap_or_else(|| panic!("caxy-mul ::> Cannot find the variable {:?}", var));

                fetch_num(num).unwrap_or_else(|var| Throw!(format!("MUL isn't made for {:?}", var)))
            },
            other => Throw!(format!("MUL isn't made for {:?}", other)),
        };
        answer *= num;
    }
        return answer;
}

fn caxy_div(operands: &Vec<Builtins>,   
    stack_hash: &HashMap<String, Builtins>,
    heap_hash: &HashMap<String, Builtins>
) -> f32 {
    let mut answer: f32 = fetch_Univ(&operands[0]).unwrap().powi(2);
    for i in operands {
        let num = match i {
            Builtins::D_type(_) => fetch_num(i).unwrap(),
            Builtins::ID(var) => {
                let num = stack_hash.get(var)
                    .or_else(|| heap_hash.get(var))
                    .unwrap_or_else(|| panic!("caxy-div ::> Cannot find the variable {:?}", var));

                fetch_num(num).unwrap_or_else(|var| Throw!(format!("DIV isn't made for {:?}", var)))
            },
            other => Throw!(format!("DIV isn't made for {:?}", other)),
        };
    
        if num != 0.0 {
            answer /= num;
        } else {
            Throw!("Who in the actual fuck divides by 0? Because I can't, and neither can Einstein!");
        }
    }
        return answer;
}

//-------------------------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------------------------

pub fn perf_math(line: &Vec<Builtins>, 
    stack_hash: &HashMap<String, Builtins>,
    heap_hash: &HashMap<String, Builtins>,
    should_print: bool
) -> f32 {
    use colored::*;

    for i in &line[1..] {   
        match i {
            Builtins::D_type(D_type::int(_)) | Builtins::D_type(D_type::float(_)) | Builtins::ID(_) => true,
            other =>  Throw!( format!(
                    "\nINVALID SYNTAX ---> cant perform arithmetic on non-numerical values {:?}\n", other
            ))
    };};
        
//----------------------------ADDITION----------------------------------------------
    if line[0] == Builtins::Operation(Operation::ADD) {
        let ans = caxy_add(&Vec::from(&line[1..]), &stack_hash, heap_hash);

        if should_print {
            println!("answer (+) ---= {}", 
            format!( "{:?}", ans)
            .on_truecolor(42, 42, 42).truecolor(150, 200, 255).bold()   );
        }

        return ans;
    }
//----------------------------SUBTRACTION----------------------------------------------
    else if line[0] == Builtins::Operation(Operation::SUB){
        let ans = caxy_sub(&Vec::from(&line[1..]), &stack_hash, heap_hash);

        if should_print {
            println!("answer (-) ---= {}", 
            format!( "{:?}", ans)
            .on_truecolor(42, 42, 42).truecolor(150, 200, 255).bold()   );
        }    
        return ans;
    }
//----------------------------MULTIPLICATION----------------------------------------------
    else if line[0] == Builtins::Operation(Operation::MUL){
        let ans = caxy_mul(&Vec::from(&line[1..]), &stack_hash, heap_hash);

        if should_print {
            println!("answer (*) ---= {}", 
            format!( "{:?}", ans)
            .on_truecolor(42, 42, 42).truecolor(150, 200, 255).bold()   );
        }  
        return ans; 
    }
//--------------------------------DIVISION----------------------------------------------
    else if line[0] == Builtins::Operation(Operation::DIV){
        let ans = caxy_div(&Vec::from(&line[1..]), &stack_hash, heap_hash);

        if should_print {
            println!("answer (/) ---= {}", 
            format!( "{:?}", ans)
            .on_truecolor(42, 42, 42).truecolor(150, 200, 255).bold()   ); 
        }   
        return ans;
    }

    else {
        Throw!("Perfmath cant do shit")
    }
}
//-------------------------------------------------------------------------------------------------------------------------------------