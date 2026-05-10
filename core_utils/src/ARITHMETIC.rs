use SysBase::defkeys::*;

use SysBase::fetch_data::{fetch_num, get_data};
use SysBase::memory_layout::{Env, Memory};
use crate::Throw;

fn seaphire_add(operands: &Vec<Builtins>,
    mem: &Memory,
    env: &Env
) -> f32 {
    let answer: f32 = operands.iter().map(|i| {
        let val = get_data(i, mem, env);
        match fetch_num(val) {
            Ok(v) => v,
            Err(_) => 0.0
        }
    }).sum();
    
    return answer;
}

fn seaphire_sub(operands: &Vec<Builtins>,               //SUBTRACTION
    mem: &Memory,
    env: &Env
) -> f32 {
    
    let mut answer: f32 =  match fetch_num(get_data(&operands[0], mem, env)) {
        Ok(v) => v,
        Err(_) => 0.0
    };    // For the offset!

    operands.iter().skip(1).for_each(|i| {
        let num = match fetch_num(
            get_data(i, mem, env)
            ) {
                Ok(v) => v,
                Err(_) => 0.0
        };
        answer -= num;
    });
        
    return answer;
}

fn seaphire_mul(operands: &Vec<Builtins>,
    mem: &Memory,
    env: &Env
) -> f32
{
    let mut answer: f32 = 1.0;
    operands.iter().for_each(|i| {
        let num = match fetch_num(
            get_data(i, mem, env)
            ) {
                Ok(v) => v,
                Err(_) => 1.0
        };
        answer *= num;
    });
    return answer;
}

fn seaphire_div(operands: &Vec<Builtins>,   
    mem: &Memory,
    env: &Env
) -> f32 
{
    let mut answer: f32 = fetch_num(get_data(&operands[0], mem, env)).unwrap();      // For the offset!

    operands.iter().skip(1).for_each(|i| {
        let num = match fetch_num(get_data(i, mem, env)) 
        {
            Ok(v) => {
                if v == 0.0 {
                    Throw!("ZeroDivisionError ::> Who in the actual fuck divides by 0? \n LIKE WHO IN THEIR RIGHT BLOODY MIND DIVIDES BY 0");
                } else { v }
            },
            Err(_) => 1.0
        };
        answer /= num;
    });
            
    return answer;
}

//-------------------------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------------------------

pub fn perf_math(line: &Vec<Builtins>, 
    mem: &Memory,
    env: &Env,
    should_print: bool
) -> f32 {
    use colored::*;
//----------------------------ADDITION----------------------------------------------
    if line[0] == Builtins::Operation(Operation::ADD) {
        let ans = seaphire_add(&Vec::from(&line[1..]), mem, env);

        if should_print {
            println!("answer (+) ---= {}", 
            format!( "{:?}", ans)
            .on_truecolor(42, 42, 42).truecolor(150, 200, 255).bold()   );
        }

        return ans;
    }
//----------------------------SUBTRACTION----------------------------------------------
    else if line[0] == Builtins::Operation(Operation::SUB){
        let ans = seaphire_sub(&Vec::from(&line[1..]), mem, env);

        if should_print {
            println!("answer (-) ---= {}", 
            format!( "{:?}", ans)
            .on_truecolor(42, 42, 42).truecolor(150, 200, 255).bold()   );
        }    
        return ans;
    }
//----------------------------MULTIPLICATION----------------------------------------------
    else if line[0] == Builtins::Operation(Operation::MUL){
        let ans = seaphire_mul(&Vec::from(&line[1..]), mem, env);

        if should_print {
            println!("answer (*) ---= {}", 
            format!( "{:?}", ans)
            .on_truecolor(42, 42, 42).truecolor(150, 200, 255).bold()   );
        }  
        return ans; 
    }
//--------------------------------DIVISION----------------------------------------------
    else if line[0] == Builtins::Operation(Operation::DIV){
        let ans = seaphire_div(&Vec::from(&line[1..]), mem, env);

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