use my_modules::defkeys::{Builtins, D_type};

pub fn fetch_num(data: &Builtins) -> Result<f32, &str> {
    match data {
        Builtins::D_type(D_type::int(s_int)) => Ok(*s_int as f32),
        Builtins::D_type(D_type::float(s_flt)) => Ok(*s_flt),
        _ => Err("fetch_num ::> Fetch Error!")
    }
}

pub fn fetch_str(data: &Builtins) -> Result<String, &str> {
    match data {
        Builtins::D_type(D_type::str( d )) => Ok(chk_qmark(d)),
        Builtins::ID(d) => Ok(chk_qmark(d)),
        _ => Err("fetch_str ::> Fetch Error!")
    }
}

pub fn fetch_bool(data: &Builtins) -> Result<bool, &str> {
    match data {
        Builtins::D_type(D_type::bool( b )) => Ok(*b),
        _ => Err("fetch_bool ::> Fetch Error!")
    }
}

/*================================================================================ */

//*================================================================================ */
/*================================================================================ */

pub fn get_val(var: &Builtins,
    stack_hash: &std::collections::HashMap<String, Builtins>,
    heap_hash:  &std::collections::HashMap<String, Builtins>,
    reg_hash:   &std::collections::HashMap<String, Builtins>) -> Option<Builtins>
{
    let a = match var {
        Builtins::D_type(_) => var,
        Builtins::ID(id) => {
            if let Some(v) = stack_hash.get( id ) { v }
            else if let Some(v) = heap_hash.get( id ){ v }
            else { 
                crate::Throw!(format!("fetch::- No variable named {:?}", id))
            }
        },
        Builtins::REGISTER(reg) => {
            if let Some(v) = reg_hash.get( reg ) { v }
            else { 
                crate::Throw!(format!("The following register is uninitiallized -> {:?}", reg))
            }
        },
        x => crate::Throw!( format!("What in actual fuck is this {:?}", x))
    };
    
    return Some(a.clone());
}

fn chk_qmark(s: &String) -> String {
    if s.starts_with('?') {
        s.get(1..).unwrap().to_string()
    }
    else { s.to_string() }
}