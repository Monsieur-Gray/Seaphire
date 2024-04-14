use std::collections::{HashSet, HashMap};
use crate::defkeys::*;

pub fn get_type(inp: Vec<Vec<String>>) -> Vec<Vec< Builtins >> {
    let mut output: Vec<Vec< Builtins >> = Vec::new();
    let invalid_char: HashSet<_> = ";:[]{}.,".chars().collect();

    let builtin_hash: HashMap<String, Builtins> = Builtins::builtin_hash();
    
    for line in inp {
        let mut temp: Vec<Builtins> = Vec::new();

        for word in line {
            let word_info = match builtin_hash.get(&word) {
                Some(bruh) => bruh.to_owned(),
                None => parse_d_type(&word, &invalid_char)
            };
            temp.push(word_info);
        };
        output.push(temp);
    };
    return output;
}

/* -------------------------------------------------------------------------------------------------------- */


fn parse_d_type(s: &String, invalid_char: &HashSet<char>) -> Builtins {
    let output = match s.parse::<i32>()   {
        Ok(val) => Builtins::D_type(D_type::int(val)),
        Err(_) =>  match s.parse::<f32>() {
            
            Ok(val) => Builtins::D_type(D_type::float(val)),
            Err(_) => match s.parse::<bool>() {

                Ok(val) => Builtins::D_type(D_type::bool(val)),
                Err(_) => {
    //------------------------- important -------------------------------------------
                    if s.starts_with('\'') && s.ends_with('\'')  {
                        let _tmp = s[1..s.len()-1].to_string();
                        Builtins::D_type(D_type::str(_tmp))
                    }

                    else if !( s.starts_with('\'') || s.ends_with('\'')) { // make sure it neither starts or ends with ""
                        if verify_var_nam(s, invalid_char){
                            Builtins::ID(s.to_string())
                        } else {
                            crate::Throw!(format!("Bro , the name '{}' doesn't follow our nomenclature bro\n\tHow can you do this bro?", s ));
                        }
                    }

                    else {    crate::Throw!(format!("get_type:: NO CLUE WHAT OCCURED => {:?}", s))    }
                }
    //------------------------- important -------------------------------------------
            }
        }
    };
    return output;
}

fn verify_var_nam(name: &str, invalid_char: &HashSet<char>) -> bool {
    name.chars().all(|c| !invalid_char.contains(&c))
}