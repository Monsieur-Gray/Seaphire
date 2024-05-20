use std::collections::HashMap;
use crate::defkeys::*;
use crate::Throw;

//--------------------------------------------------------------------------------------------------------------------------------------
fn fetch_str(data: &Builtins) -> Result<String, &str> {
    match data {
        Builtins::D_type(D_type::str( d )) => Ok(chk_annotation(d)),
        Builtins::ID(d) => Ok(chk_annotation(d)),
        _ => Err("fetch_str ::> Fetch Error!")
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
pub fn mutate_mem ( line: &Vec<Builtins>,     
    stack_hash: &HashMap<String, Value>,
    heap_clone: HashMap<String, Value>)     -> HashMap<String, Value>
{
    let keyname = fetch_str(&line[1]).unwrap();
    let ol_val =  if let Some(nam) = heap_clone.get(&keyname) { &nam.value } 
    else {
        Throw!( format!( "No MUTABLE variable named '{}' found\nMake sure its mutable", keyname) )
    };

    let shit = line[2].to_value(Scope::GlobalScope);

    let new_val =  match &line[2] {
        Builtins::D_type(_) => &shit,
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
    };

    // println!("\t--o> {:?}", ol_val);
    // println!("\t--n> {:?}", new_val);
    let mut new_heap = heap_clone.clone();

    if check_compatible(ol_val, &new_val.value, false) {
        new_heap.entry(keyname).and_modify(|e| *e = new_val.clone() );
    };

    new_heap
}

//--------------------------------------------------------------------------------------------------------------------------------------

pub fn insert_to_mem ( 
    line: &Vec<Builtins>,     
    mut heap_clone: HashMap<String, Value>,
    direct_value: Builtins )     -> HashMap<String, Value>
{
    let keyname = fetch_str(&line[1]).unwrap().clone();

    let ol_val =  if let Some(nam) = heap_clone.get(&keyname) {  
        &nam.value 
    } 
    else {  
        Throw!( format!( "No MUTABLE variable named '{}' found\nMake sure its mutable", keyname) )  
    };

    if check_compatible(ol_val, &direct_value, true) {
        heap_clone.entry(keyname).and_modify(|e| *e = direct_value.to_value(Scope::GlobalScope) );
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
        _ => crate::Throw!( 
            format!("Can't insert a value of type {:?} into a variable of type {:?}",
                v1.get_data_type(), v2.get_data_type()
            )
        )
    }
}