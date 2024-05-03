use std::collections::HashMap;
use crate::defkeys::*;
// use crate::Throw;

//--------------------------------------------------------------------------------------------------------------------------------------
fn fetch_str(data: &Builtins) -> Result<String, &str> {
    match data {
        Builtins::D_type(D_type::str( d )) => Ok(chk_qmark(d)),
        Builtins::ID(d) | Builtins::REGISTER(d) => Ok(chk_qmark(d)),
        er => crate::Throw!( format!("fetch_str ::> Fetch Error! -# {:?}", er.clone()).as_str() )
    }
}

fn chk_qmark(s: &String) -> String {
    if s.starts_with('?') {
        s.get(1..).unwrap().to_string()
    }
    else { s.to_string() }
}

//--------------------------------------------------------------------------------------------------------------------------------------
//--------------------------------------------------------------------------------------------------------------------------------------
pub fn mutate_reg ( line: &Vec<Builtins>,     
        stack_hash: &HashMap<String, Builtins>,
        mut reg_clone: HashMap<String, Builtins>)     -> HashMap<String, Builtins>
{
    let register_id = fetch_str(&line[1]).unwrap();

    let new_val =  match &line[2] {
        Builtins::D_type(_) => &line[2],
        Builtins::ID(id) => {
            if let Some(v1) = stack_hash.get(id) {  v1  }
            else if let Some(v2) = reg_clone.get(id) {  v2  }
            else {
                panic!("cry like a dog")
            }
        },
        _ => panic!("You werent suppose tto put htat")
    }.clone() ;

    match reg_clone.get(&register_id) {
        Some(ol_val) => {
            if check_compatible(ol_val, &new_val, false) {
                reg_clone.entry(register_id).and_modify(|e| *e = new_val );
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
    mut reg_clone: HashMap<String, Builtins>,
    direct_value: Builtins )     -> HashMap<String, Builtins>
{
    // println!("---> {:?}", &line);
    let register_id = fetch_str(&line[1]).unwrap().clone();

    match reg_clone.get(&register_id) {
        Some(ol_val) => {
            if check_compatible(ol_val, &direct_value, true) {
                reg_clone.entry(register_id).and_modify(|e| *e = direct_value );
            };
        },
        None => {
            reg_clone.insert(register_id, direct_value);
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