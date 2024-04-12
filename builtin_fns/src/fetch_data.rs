use my_modules::defkeys::*;

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

// impl<I, F, S, B> MyRes<I, F, S, B> {
//     pub fn flt_unwrap(self) -> F 
//         where S: std::fmt::Debug, B: std::fmt::Debug, F: std::fmt::Debug, I: std::fmt::Debug
//     {
//         match self {
//         MyRes::Flt(F) => F,
//         other => panic!("float unwrap not implemented for {:?}", other)
//         }
//     }
// //-----------------------------------------------------------------------------------------------------------------------
//     pub fn str_unwrap(self) -> S 
//         where S: std::fmt::Debug, B: std::fmt::Debug, F: std::fmt::Debug, I: std::fmt::Debug
//     {
//         match self {
//             MyRes::Str(S) => S,
//             other => panic!("str unwrap not implemented for {:?}", other)
//         }
//     }
// //-----------------------------------------------------------------------------------------------------------------------
//     pub fn bool_unwrap(self) -> B 
//         where S: std::fmt::Debug, B: std::fmt::Debug, F: std::fmt::Debug, I: std::fmt::Debug
//     {
//         match self {
//             MyRes::Bool(B) => B,
//             other => panic!("bool unwrap not implemented for {:?}", other)
//         }
//     }
// }