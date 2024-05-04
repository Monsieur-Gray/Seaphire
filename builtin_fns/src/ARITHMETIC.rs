use std::collections::HashMap;

use my_modules::defkeys::*;

use crate::fetch_data::{fetch_num, get_val};
use crate::Throw;

fn seaphire_add(operands: &Vec<Builtins>,
    stack_hash: &HashMap<String, Builtins>,
    heap_hash: &HashMap<String, Builtins>,
    reg_hash: &HashMap<String, Builtins>
) -> f32 {
    let answer: f32 = operands.iter().map(|i| {
        match get_val(i, stack_hash, heap_hash, reg_hash) {
            Some(v) => fetch_num(&v).unwrap(),
            None => Throw!(format!("ADD_2 isnt made for {:?}", i))
        }
    }).sum();
    
    return answer;
}

fn seaphire_sub(operands: &Vec<Builtins>,               //SUBTRACTION
    stack_hash: &HashMap<String, Builtins>,
    heap_hash: &HashMap<String, Builtins>,
    reg_hash: &HashMap<String, Builtins>
) -> f32 {
    
    let mut answer: f32 = match get_val(&operands[0], stack_hash, heap_hash, reg_hash) {
        Some(v) => fetch_num(&v).unwrap(),
        None => Throw!(format!("SUB_2 isnt made for {:?}", &operands[0]))
    };    // For the offset!

    operands.iter().skip(1).for_each(|i| {
        let num: f32 = match get_val(i, stack_hash, heap_hash, reg_hash) {
            Some(v) => fetch_num(&v).unwrap(),
            None => Throw!(format!("SUB_2 isnt made for {:?}", i))
        };
        answer -= num;
    });
        
    return answer;
}

fn seaphire_mul(operands: &Vec<Builtins>,
    stack_hash: &HashMap<String, Builtins>,
    heap_hash: &HashMap<String, Builtins>,
    reg_hash: &HashMap<String, Builtins>
) -> f32 {

    let mut answer: f32 = 1.0;
    operands.iter().for_each(|i| {
        let num = match get_val(i, stack_hash, heap_hash, reg_hash) {
            Some(v) => fetch_num(&v).unwrap(),
            None => Throw!(format!("MUL_2 isnt made for {:?}", i))
        };
        answer *= num;
    });

    return answer;
}

fn seaphire_div(operands: &Vec<Builtins>,   
    stack_hash: &HashMap<String, Builtins>,
    heap_hash: &HashMap<String, Builtins>,
    reg_hash: &HashMap<String, Builtins>
) -> f32 {

    let mut answer: f32 = match get_val(&operands[0], stack_hash, heap_hash, reg_hash) {
        Some(v) => fetch_num(&v).unwrap(),
        None => Throw!(format!("DIV_2 isnt made for {:?}", &operands[0]))
    }.powi(2);      // For the offset!

    operands.iter().for_each(|i| {
        let num = match get_val(i, stack_hash, heap_hash, reg_hash) {
            Some(v) => fetch_num(&v).unwrap(),
            None => Throw!(format!("DIV_2 isnt made for {:?}", i))
        };
        if num != 0.0 {
            answer /= num;
        } else {
            Throw!("Who in the actual fuck divides by 0? LIKE WHO IN THEIR RIGHT BLOODY MIND DIVIDES BY 0");
        }
    });
            
    return answer;
}

//-------------------------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------------------------

pub fn perf_math(line: &Vec<Builtins>, 
    stack_hash: &HashMap<String, Builtins>,
    heap_hash: &HashMap<String, Builtins>,
    reg_hash: &HashMap<String, Builtins>,
    should_print: bool
) -> f32 {
    use colored::*;
//----------------------------ADDITION----------------------------------------------
    if line[0] == Builtins::Operation(Operation::ADD) {
        let ans = seaphire_add(&Vec::from(&line[1..]), stack_hash, heap_hash, reg_hash);

        if should_print {
            println!("answer (+) ---= {}", 
            format!( "{:?}", ans)
            .on_truecolor(42, 42, 42).truecolor(150, 200, 255).bold()   );
        }

        return ans;
    }
//----------------------------SUBTRACTION----------------------------------------------
    else if line[0] == Builtins::Operation(Operation::SUB){
        let ans = seaphire_sub(&Vec::from(&line[1..]), stack_hash, heap_hash, reg_hash);

        if should_print {
            println!("answer (-) ---= {}", 
            format!( "{:?}", ans)
            .on_truecolor(42, 42, 42).truecolor(150, 200, 255).bold()   );
        }    
        return ans;
    }
//----------------------------MULTIPLICATION----------------------------------------------
    else if line[0] == Builtins::Operation(Operation::MUL){
        let ans = seaphire_mul(&Vec::from(&line[1..]), stack_hash, heap_hash, reg_hash);

        if should_print {
            println!("answer (*) ---= {}", 
            format!( "{:?}", ans)
            .on_truecolor(42, 42, 42).truecolor(150, 200, 255).bold()   );
        }  
        return ans; 
    }
//--------------------------------DIVISION----------------------------------------------
    else if line[0] == Builtins::Operation(Operation::DIV){
        let ans = seaphire_div(&Vec::from(&line[1..]), stack_hash, heap_hash, reg_hash);

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