use crate::defkeys::*;

pub fn fetch_num(data: &Builtins) -> Result<f32, &str> {
    match data {
        Builtins::D_type(D_type::int(s_int)) => Ok(*s_int as f32),
        Builtins::D_type(D_type::float(s_flt)) => Ok(*s_flt),
        _ => Err("fetch_num ::> Fetch Error!")
    }
}

pub fn fetch_str(data: &Builtins) -> Result<String, &str> {
    match data {
        Builtins::D_type(D_type::str( d )) => Ok(d.to_string()),
        Builtins::ID(d) => Ok(d.to_string()),
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
pub fn fetch_Univ(data: &Builtins) -> Result<f32, String> {
    match data {
        Builtins::D_type(D_type::int(s_int)) => Ok(*s_int as f32),
        Builtins::D_type(D_type::float(s_flt)) => Ok(*s_flt),
        
        Builtins::D_type(D_type::str( d )) => Err(chk_qmark(d)),
        Builtins::ID(d) => Err(chk_qmark(d)),
        _ => crate::Throw!("fetchUNIV cant do shit")
    }
}
/*================================================================================ */
/*================================================================================ */

fn chk_qmark(s: &String) -> String {
    if s.starts_with('?') {
        s.get(1..).unwrap().to_string()
    }
    else { s.to_string() }
}