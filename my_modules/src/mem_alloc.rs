use std::collections::HashMap;
use crate::defkeys::*;
use crate::fetch_data::*;
use crate::Throw;

// [MemType(int), ID("Age"), D_type(int(69))] ------------------------- new
pub fn calloc(vsec: Vec<Vec<Builtins>>) -> (HashMap<String, Builtins>, HashMap<String, Builtins>) {
    let mut stack_hash: HashMap<String, Builtins> = HashMap::new();
    let mut heap_hash: HashMap<String, Builtins> = HashMap::new();

    for line in vsec {
        let key = fetch_str(&line.get(1).unwrap() ).unwrap();
        let val = line.last().unwrap().clone();

        if key.starts_with('?') {
            heap_hash.insert(key[1..].to_string(), val);
        }
        else {
            stack_hash.insert(key, val);
        };
    };
    return (stack_hash, heap_hash);
}

//--------------------------------------------------------------------------------------------------------------------------------------
pub fn free_mem<'b>(var_nam: &String,
    stack_hash: &mut HashMap<String, Builtins>,
    heap_hash: &mut HashMap<String, Builtins>
) {
    if let Some(_) = stack_hash.get(var_nam) {
        stack_hash.remove(var_nam);
    }
    else if let Some(_) = heap_hash.get(var_nam) {
        heap_hash.remove(var_nam);
    }
    else {
        crate::Throw!(format!("FREE_MEM ::> No variable named '{}'", var_nam));
    };
}

//--------------------------------------------------------------------------------------------------------------------------------------
//--------------------------------------------------------------------------------------------------------------------------------------
// D_type(str("Engineer"))
pub fn mutate_mem <'b> ( line: &'b Vec<Builtins>,     
        stack_hash: &HashMap<String, Builtins>,
        mut heap_clone: HashMap<String, Builtins>)     -> HashMap<String, Builtins>
{
    let keyname = fetch_str(&line[1]).unwrap().clone();

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

        if check_compatible(ol_val, &new_val) {
            heap_clone.entry(keyname).and_modify(|e| *e = new_val );
        };

        heap_clone
    }

//--------------------------------------------------------------------------------------------------------------------------------------

pub fn insert_to_mem <'b> ( line: &'b Vec<Builtins>,     
        heap_clone: HashMap<String, Builtins>,
        direct_value: Builtins )     -> HashMap<String, Builtins>
{
    let mut heap_clone = heap_clone.clone();

    let keyname = fetch_str(&line[1]).unwrap().clone();
    heap_clone.entry(keyname).and_modify(move |e| *e = direct_value );
    heap_clone
}

//--------------------------------------------------------------------------

fn check_compatible(v1: &Builtins, v2: &Builtins) -> bool {
    match ( v1, v2){
        (Builtins::D_type(D_type::int(_)), Builtins::D_type(D_type::int(_))) => true,
        (Builtins::D_type(D_type::float(_)), Builtins::D_type(D_type::float(_))) => true,
        (Builtins::D_type(D_type::str(_)), Builtins::D_type(D_type::str(_))) => true,
        (Builtins::D_type(D_type::bool(_)), Builtins::D_type(D_type::bool(_))) => true,
        _ => crate::Throw!("The old value and new value dont have the same type bro")
    }
}