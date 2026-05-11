use SysBase::memory_layout::{Memory, Env};
use colored::Colorize;
use SysBase::defkeys::*;

use SysBase::fetch_data::{fetch_int, fetch_float, fetch_bool, fetch_str};

pub fn print_line(
    line: &Vec<Builtins>,
    mem: &Memory,
    env: &Env,
    is_newline: bool,
    is_cool: bool
) {
    match &line[1] {
        Builtins::D_type(D_type::str(strn)) => {
            if is_newline {
                println!("{}", strn.replace('\'', "").truecolor(150, 150, 100).bold());
            } else {
                print!("{}", strn.replace('\'', "").truecolor(150, 150, 100).bold());
            }
        }
        Builtins::D_type(D_type::int(i)) => {
            if is_newline {
                println!("{}", i.to_string().truecolor(150, 150, 100).bold());
            } else {
                print!("{}", i.to_string().truecolor(150, 150, 100).bold());
            }
        }
        Builtins::D_type(D_type::bool(b)) => {
            if is_newline {
                println!("{}", b.to_string().truecolor(150, 150, 100).bold());
            } else {
                print!("{}", b.to_string().truecolor(150, 150, 100).bold());
            }
        }

        Builtins::ID(id) => {
            
                print_cool(id.to_string(), mem, env, is_newline, is_cool);
        },

        // ! Deprecated
/*         Builtins::REGISTER(reg_id) => {
            if reg_hash.contains_key(reg_id) {
                print_cool(reg_id.to_string(), reg_hash, is_newline, is_cool);
            } else {
                crate::Throw!(format!("The following register is empty/uninitialized > {}", reg_id));
            }
        },
 */
        _ => crate::Throw!(format!("PRINT:> No variable named {:?}", line[1])),
    }
}

// Not a stand alone
fn print_cool(
    var_nam: String, 
    mem: &Memory,
    env: &Env,
    is_newline: bool,
    is_cool: bool
) {
    let dat = &mem.get(*env.get(&var_nam)).data;

    if let Ok(dat) = fetch_int(&dat) {
        if is_cool {
            println!("{:?} contains {}", var_nam, dat.to_string().green().bold());
        } else {
            if is_newline{
                print!("{}\n", dat.to_string().truecolor(150, 150, 100).bold());
            } else {
                print!("{}", dat.to_string().truecolor(150, 150, 100).bold());
            }
        }
    } 
    else if let Ok(dat) = fetch_float(&dat) {
        if is_cool {
            println!("{:?} contains {}", var_nam, dat.to_string().green().bold());
        } else {
            if is_newline{
                print!("{}\n", dat.to_string().truecolor(150, 150, 100).bold());
            } else {
                print!("{}", dat.to_string().truecolor(150, 150, 100).bold());
            }
        }
    } 
    else if let Ok(dat) = fetch_str(&dat) {
        if is_cool {
            println!("{:?} contains {}", var_nam, dat.to_string().green().bold());
        } else {
            if is_newline{
                print!("{}\n", dat.to_string().truecolor(150, 150, 100).bold());
            } else {
                print!("{}", dat.to_string().truecolor(150, 150, 100).bold());
            }
        }
    } 
    else {
        if is_cool {
            println!("{:?} contains {}", var_nam, fetch_bool(&dat).unwrap()
                .to_string().green().bold());
        }
        else {
            if is_newline {
                print!("{}\n", fetch_bool(&dat).unwrap().to_string().truecolor(150, 150, 100).bold());
            } else {
                print!("{}", fetch_bool(&dat).unwrap().to_string().truecolor(150, 150, 100).bold());
            }
        }
    };
}
