use crate::defkeys::{Builtins, D_type};
use crate::{Throw, memory_layout::*};

pub fn fetch_int(data: &Builtins) -> Result<i32, f32> {
    match data {
        Builtins::D_type(D_type::int(s_int)) => Ok(*s_int),
        Builtins::D_type(D_type::float(s_flt)) => Err(*s_flt),
        _ => Throw!("fetch_int ::> Fetch Error!")
    }
}

pub fn fetch_float(data: &Builtins) -> Result<f32, i32> {
    match data {
        Builtins::D_type(D_type::float(s_flt)) => Ok(*s_flt),
        Builtins::D_type(D_type::int(s_int)) => Err(*s_int),
        _ => Throw!("fetch_int ::> Fetch Error!")
    }
}


pub fn fetch_str(data: &Builtins) -> Result<String, &str> {
    match data {
        Builtins::D_type(D_type::str( d )) => Ok(chk_annotation(d)),
        Builtins::ID(d) => Ok(chk_annotation(d)),
        _ => Throw!("fetch_str ::> Fetch Error!")
    }
}

pub fn fetch_bool(data: &Builtins) -> Result<bool, &str> {
    match data {
        Builtins::D_type(D_type::bool( b )) => Ok(*b),
        _ => Throw!("fetch_bool ::> Fetch Error!")
    }
}

/*================================================================================ */

/*================================================================================ */

// var = variable you want to fetch, other = memory (old model)
// (NEW) var = variable you want to fetch, mem: Memory = to fetch the value, env: Env = to fetch the address of the value
// var -> env => pointer -> mem => data  
// todo later fix the lifetime issue
pub fn get_data<'m>(var: &'m Builtins, 
    mem: &'m Memory, 
    env: &Env
) -> &'m Builtins
{
    let a: &'m Builtins = match var {
        Builtins::ID(id) | Builtins::REGISTER(id) => {
            let ptr = env.get(id);
            &mem.get(*ptr).data
        },
        Builtins::D_type(_) => var,
        x => crate::Throw!( format!("What in actual fuck is this {:?}", x))
    };
    
    return a;
}

// variable name => its memory_cell
pub fn resolve_cell<'m>(
    name: &str,
    mem: &'m Memory,
    env: &Env
) -> &'m MemCell
{
    return mem.get(*env.get(name));
}

// variable name => its mutable memory_cell
pub fn resolve_cell_mut<'m>(
    name: &str,
    mem: &'m mut Memory,
    env: &Env
) -> &'m mut MemCell
{
    return mem.get_mut(*env.get(name));
}















fn chk_annotation(s: &String) -> String {
    if s.starts_with('?') {
        s.get(1..).unwrap().to_string().replace("\'", "")
    }
    else { s.to_string().replace("\'", "") }
}
