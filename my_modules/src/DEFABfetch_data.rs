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
pub fn fetch_Univ(data: &Builtins) -> Result<f32, String> {
    match data {
        Builtins::D_type(D_type::int(s_int)) => Ok(*s_int as f32),
        Builtins::D_type(D_type::float(s_flt)) => Ok(*s_flt),
        
        Builtins::D_type(D_type::str( d )) => Err(chk_qmark(d)),
        Builtins::ID(d) => Err(chk_qmark(d)),
        _ => crate::Throw!("fetch-UNIV cant do shit")
    }
}
/*================================================================================ */
/*================================================================================ */
pub fn fetch_MyRes(data: &Builtins) -> MyRes<i32, f32, String, bool> {
    match data {
        Builtins::D_type(D_type::int(s_int)) => MyRes::Int(*s_int),
        Builtins::D_type(D_type::float(s_flt)) => MyRes::Flt(*s_flt),

        Builtins::D_type(D_type::str( d )) => MyRes::Str(chk_qmark(d)),
        Builtins::ID(d) => MyRes::Str(chk_qmark(d)),

        Builtins::D_type(D_type::bool(b)) => MyRes::Bool(*b),
        _ => crate::Throw!("I cant do shit")
    }
}

pub fn get_val(var: &Builtins,
    stack_hash: &std::collections::HashMap<String, Builtins>,
    heap_hash: &std::collections::HashMap<String, Builtins> ) -> Option<Builtins>
{
    let a = match var {
        Builtins::D_type(_) => var,
        Builtins::ID(id) => {
            if let Some(v) = stack_hash.get( id ) { v }
            else if let Some(v) = heap_hash.get( id ){ v }
            else { return None; }
        },
        _ => crate::Throw!("What in actual fuck is this")
    };
    
    return Some(a.clone());
}

#[derive(Debug)]
pub enum MyRes<I, F, S, B>{
    Int(I),
    Flt(F), 
    Str(S),
    Bool(B)
}

fn chk_qmark(s: &String) -> String {
    if s.starts_with('?') {
        s.get(1..).unwrap().to_string()
    }
    else { s.to_string() }
}
