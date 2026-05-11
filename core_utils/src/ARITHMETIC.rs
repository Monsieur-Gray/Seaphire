use SysBase::defkeys::*;

use SysBase::fetch_data::{fetch_int, get_data};
use SysBase::memory_layout::{Env, Memory};
use crate::Throw;

fn seaphire_add(operands: &Vec<Builtins>,
    mem: &Memory,
    env: &Env
) -> Number {
    let mut return_type: char = 'i';
    let answer: f32 = operands.iter().map(|n: &Builtins| {
        let val = get_data(n, mem, env);
        match fetch_int(val) {
            Ok(i) => {
                return_type = 'i';
                i as f32
            },
            Err(f) => {
                return_type = 'f';
                f
            }
        }
    }).sum();
    
    if return_type == 'i' {
        return Number::int(answer as i32);
    }
    else {
        return Number::float(answer);
    }

}

fn seaphire_sub(operands: &Vec<Builtins>,               //SUBTRACTION
    mem: &Memory,
    env: &Env
) -> Number {
    
    let mut return_type: char;
    let mut answer: f32 =  match fetch_int(get_data(&operands[0], mem, env)) {
        Ok(i) => {
            return_type = 'i';
            i as f32
        },
        Err(f) => {
            return_type = 'f';
            f
        }
    };    // For the offset!

    operands.iter().skip(1).for_each(|n| {
        let num = match fetch_int(get_data(n, mem, env)) {
            Ok(i) => {
                return_type = 'i';
                i as f32
            },
            Err(f) => {
                return_type = 'f';
                f
            }
        };
        answer -= num;
    });
        
    if return_type == 'i' {
        return Number::int(answer as i32);
    }
    else {
        return Number::float(answer);
    }
}

fn seaphire_mul(operands: &Vec<Builtins>,
    mem: &Memory,
    env: &Env
) -> Number
{
    let mut return_type: char = 'i';
    let mut answer: f32 = 1.0;
    operands.iter().for_each(|n| {

        let num = match fetch_int(get_data(n, mem, env)) {
            Ok(i) => {
                return_type = 'i';
                i as f32
            },
            Err(f) => {
                return_type = 'f';
                f
            }
        };

        answer *= num;
    });

    if return_type == 'i' {
        return Number::int(answer as i32);
    }
    else {
        return Number::float(answer);
    }

}

fn seaphire_div(operands: &Vec<Builtins>,   
    mem: &Memory,
    env: &Env
) -> Number 
{
    let mut return_type: char;
    let mut answer: f32 =  match fetch_int(get_data(&operands[0], mem, env)) {
        Ok(i) => {
            return_type = 'i';
            i as f32
        },
        Err(f) => {
            return_type = 'f';
            f
        }
    };    // For the offset!

    operands.iter().skip(1).for_each(|i| {
        let num = match fetch_int(get_data(i, mem, env)) 
        {
            Ok(i) => {
                return_type = 'i';
                if i == 0 {
                    Throw!("ZeroDivisionError ::> Who in the actual fuck divides by 0? \n LIKE WHO IN THEIR RIGHT BLOODY MIND DIVIDES BY 0");
                }
                i as f32
            },
            Err(f) => {
                return_type = 'f';
                if f == 0.0 {
                    Throw!("ZeroDivisionError ::> Who in the actual fuck divides by 0? \n LIKE WHO IN THEIR RIGHT BLOODY MIND DIVIDES BY 0");
                }
                f
            }
        };
        answer /= num;
    });
            
    if return_type == 'i' {
        return Number::int(answer as i32);
    }
    else {
        return Number::float(answer);
    }

}

//-------------------------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------------------------

pub fn perf_math(line: &Vec<Builtins>, 
    mem: &Memory,
    env: &Env,
    should_print: bool
) -> Number {
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