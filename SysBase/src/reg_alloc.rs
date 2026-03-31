use std::collections::HashMap;
use crate::defkeys::*;
// use crate::Throw;

//--------------------------------------------------------------------------------------------------------------------------------------
fn fetch_str(data: &Builtins) -> Result<String, &str> {
    match data {
        Builtins::D_type(D_type::str( d )) => Ok(chk_annotation(d)),
        Builtins::ID(d) | Builtins::REGISTER(d) => Ok(chk_annotation(d)),
        er => crate::Throw!( format!("fetch_str ::> Fetch Error! -# {:?}", er.clone()).as_str() )
    }
}

fn chk_annotation(s: &String) -> String {
    if s.starts_with('?') {
        s.get(1..).unwrap().to_string().replace("\'", "")
    }
    else { s.to_string().replace("\'", "") }
}

//--------------------------------------------------------------------------------------------------------------------------------------
//--------------------------------------------------------------------------------------------------------------------------------------
pub fn mutate_reg ( line: &Vec<Builtins>,     
        stack_hash: &HashMap<String, Value>,
        mut reg_clone: HashMap<String, Value>)     -> HashMap<String, Value>
{
    let register_id = fetch_str(&line[1]).unwrap();

    let shit = line[2].to_value(Scope::GlobalScope);

    let new_val =  match &line[2] {
        Builtins::D_type(_) => &shit,
        Builtins::ID(id) => {
            if let Some(v1) = stack_hash.get(id) {  
                v1
            }
            else if let Some(v2) = reg_clone.get(id) {  
                v2  
            }
            else {
                panic!("cry like a dog")
            }
        },
        _ => panic!("You werent suppose tto put htat")
    }.clone() ;

    match reg_clone.get(&register_id) {
        Some(ol_val) => {
            if check_compatible(&ol_val.value, &new_val.value, false) {
                // reg_clone.entry(register_id).and_modify(|e| *e = new_val );
                reg_clone.insert(register_id, new_val);
            };
        },
        None => {
            reg_clone.insert(register_id, new_val);
        }
    }

    


        reg_clone
    }

//--------------------------------------------------------------------------------------------------------------------------------------

pub fn insert_to_reg ( 
    line: &Vec<Builtins>,     
    mut reg_clone: HashMap<String, Value>,
    direct_value: Builtins )     -> HashMap<String, Value>
{
    // println!("---> {:?}", &line);
    let register_id = fetch_str(&line[1]).unwrap().clone();

    match reg_clone.get(&register_id) {
        Some(ol_val) => {
            if check_compatible(&ol_val.value, &direct_value, true) {
                reg_clone.insert(register_id, direct_value.to_value(Scope::GlobalScope));
            };
        },
        None => {
            reg_clone.insert(register_id, direct_value.to_value(Scope::GlobalScope));
        }
    };
    reg_clone
}

//--------------------------------------------------------------------------

fn check_compatible(v1: &Builtins, v2: &Builtins, allowModif: bool) -> bool {
    match (v1, v2){
        (Builtins::D_type(D_type::int(_)), Builtins::D_type(D_type::int(_))) => true,
        (Builtins::D_type(D_type::float(_)), Builtins::D_type(D_type::float(_))) => true,
        (Builtins::D_type(D_type::str(_)), Builtins::D_type(D_type::str(_))) => true,
        (Builtins::D_type(D_type::bool(_)), Builtins::D_type(D_type::bool(_))) => true,

        (Builtins::D_type(D_type::int(_)), Builtins::D_type(D_type::float(_))) | 
    (Builtins::D_type(D_type::float(_)), Builtins::D_type(D_type::int(_))) => {
            if allowModif { 
                true 
            } else {
                false
            }
    },
        _ => crate::Throw!("The old value and new value dont have the same type bro")
    }
}