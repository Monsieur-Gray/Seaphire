use std::collections::HashMap;
use colored::Colorize;
use my_modules::{defkeys::*, fetch_data::{fetch_bool, fetch_num, fetch_str}};

pub fn print_line(line: &Vec<Builtins>,
    stack_hash: &HashMap<String, Builtins>,
    heap_hash: HashMap<String, Builtins>
) -> String {
    if line.len() > 1 {
        match &line[1] {
            Builtins::D_type(D_type::str(strn)) => {
                println!(
                ":> {}", strn.truecolor(150, 150, 100).bold()
                );
                return strn.to_string();
            },
            Builtins::D_type(D_type::bool(b)) => {
                println!(
                ":> {}", b.to_string().truecolor(150, 150, 100).bold()
                );
                return b.to_string();    
            },

            Builtins::ID(id) => {
                if stack_hash.contains_key(id) {
                    print_var(id.to_string(), stack_hash)
                }
                else {
                    print_var(id.to_string(), &heap_hash)
                };
                return id.to_string();
            },
            hmm => crate::Throw!( format!( "No variable named {:?}", hmm ) )
        }
    }
    else {
        crate::Throw!("Didn't yo mamma tell ya, you must add some SHIT after the PRNT keyword?");
    }

}

fn print_var(var_nam: String,
    mem_hash: &HashMap<String, Builtins>
) {
    let dat = match mem_hash.get(&var_nam) {
        Some(stuff) => stuff,
        None => crate::Throw!( format!( "No variable named {:?}", var_nam ))
    };
    if let Ok(dat) = fetch_num(dat) {
        println!("{:?} contains {}", var_nam, dat.to_string().green().bold());
    }
    else if let Ok(dat) = fetch_str(dat) {
        println!("{:?} contains {}", var_nam, dat.to_string().green().bold());
    }
    else {
        println!("{:?} contains {}", var_nam, fetch_bool(dat).unwrap().to_string().green().bold());
    };
}
