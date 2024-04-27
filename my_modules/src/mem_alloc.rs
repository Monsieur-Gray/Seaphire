use std::collections::HashMap;
use crate::defkeys::*;
use crate::fetch_data::*;
use crate::Throw;

//--------------------------------------------------------------------------------------------------------------------------------------
pub fn mutate_mem <'b> ( line: &'b Vec<Builtins>,     
        stack_hash: &HashMap<String, Builtins>,
        mut heap_clone: HashMap<String, Builtins>)     -> HashMap<String, Builtins>
{
    let keyname = fetch_str(&line[1]).unwrap();

    let ol_val =  if let Some(nam) = heap_clone.get(&keyname) { 
        nam 
    } 
    else {
        Throw!( format!( "No MUTABLE variable named '{}' found\nMake sure its mutable", keyname) )
    };

    let new_val =  match &line[2] {
        Builtins::D_type(_) => &line[2],
        Builtins::ID(id) => {
            if let Some(v1) = stack_hash.get(id) {
                v1
            }
            else if let Some(v2) = heap_clone.get(id) {
                v2
            }
            else {
                panic!("cry like a dog")
            }
        },
        _ => panic!("You werent suppose tto put htat")
        }.clone() ;

        if check_compatible(ol_val, &new_val, false) {
            heap_clone.entry(keyname).and_modify(|e| *e = new_val );
        };

        heap_clone
    }

//--------------------------------------------------------------------------------------------------------------------------------------

pub fn insert_to_mem <'b> ( 
    line: &'b Vec<Builtins>,     
    mut heap_clone: HashMap<String, Builtins>,
    direct_value: Builtins )     -> HashMap<String, Builtins>
{
    let keyname = fetch_str(&line[1]).unwrap().clone();

    let ol_val =  if let Some(nam) = heap_clone.get(&keyname) {  nam } 
        else {  Throw!( format!( "No MUTABLE variable named '{}' found\nMake sure its mutable", keyname) )  };

    if check_compatible(ol_val, &direct_value, true) {
        heap_clone.entry(keyname).and_modify(|e| *e = direct_value );
    };
    heap_clone
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