use std::collections::HashMap;
use std::hash::Hash;
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;
use crate::Throw;
use crate::defkeys::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CaxyParser;

pub fn pest_parse(unparsed_filestr: &String) -> (Option<Pair<Rule>>, Option<Pair<Rule>>){
    let file = match CaxyParser::parse(Rule::file, unparsed_filestr.as_str()) {
        Ok(mut outp) => outp.next().unwrap(),
        Err(_) => panic!("Shit occured!")
    };
    let mut vvec = None;
    let mut mvec = None;
    for s in file.into_inner() {
        match s.as_rule() {
            Rule::vsec => {vvec = Some(s)},
            Rule::msec => {mvec = Some(s)},
            _ => continue
        }
    }
    return (mvec, vvec);
}

pub fn calloc(vinp: Option<Pair<Rule>>) -> (HashMap<String, Builtins>, HashMap<String, Builtins>) {
    let mut line_num = -1;  // consider the extra first term (number of variables)
    let mut vsec_size = 0;
    let mut stack_hash: HashMap<String, Builtins> = HashMap::new();
    let mut heap_hash: HashMap<String, Builtins> = HashMap::new();

    for i in vinp.unwrap().into_inner() {
        match i.as_rule() {
            Rule::INT => {vsec_size = i.as_str().parse::<i32>().unwrap();},
            Rule::var_make => {
                let mut id = String::new();
                let mut data = Builtins::Comment;

                for j in i.into_inner() {
                    let jstr = j.as_str();
                    match j.as_rule() {
                        Rule::MemType => continue,
                        Rule::ID => { id = jstr.to_string(); },
                        _ => { data = parse_dtype(j).unwrap() }
                    };
                };

                if id.starts_with('?') {
                    id.remove(0);
                    heap_hash.insert(id, data);
                }
                else {
                    stack_hash.insert(id, data);
                };
            },
            _ => continue
        };
        line_num += 1;
    };

    if line_num != vsec_size {  Throw!("Incorrect shit of variables");  }
    else {  return (stack_hash, heap_hash);  };
}

pub fn make_msec(msec: Option<Pair<Rule>>) -> Vec<Vec<Builtins>>{
    let mut MAIN_SEC: Vec<Vec<Builtins>> = vec![];
    let builtins_hash = Builtins::builtin_hash();

    for line in msec.unwrap().into_inner() {
        let line_vec = parse_exprs(&line, &builtins_hash);
        MAIN_SEC.push(line_vec);
    };
    return MAIN_SEC;
}


//----------------------------------------------------------------------------------------------------------------------------------------------------------------------
fn parse_dtype(p: Pair<Rule>) -> Option<Builtins> {
    let pstr = p.as_str();
    let output = match p.as_rule() {
        Rule::INT =>    Some(  Builtins::D_type( D_type::int(pstr.parse::<i32>().unwrap()) )    ),
        Rule::FLOAT =>  Some(  Builtins::D_type( D_type::float(pstr.parse::<f32>().unwrap()) )  ),
        Rule::BOOL =>   Some(  Builtins::D_type( D_type::bool(pstr.parse::<bool>().unwrap()) )  ),
        Rule::STRLIT => Some(  Builtins::D_type( D_type::str(pstr.to_string()) )    ),
        Rule::ID =>     Some(  Builtins::ID    ( pstr.to_string() )  ),
        _ =>   None,
    };
    return output;
}


//----------------------------------------------------------------------------------------------------------------------------------------------------------------------
fn parse_exprs(line: &Pair<Rule>, builtins_hash: &HashMap<String, Builtins>) -> Vec<Builtins>{
    let mut line_vec = vec![];

    match line.as_rule() {

        Rule::math_expr  => {
            let mut exp_vec = vec![];

            for expr_iter in line.clone().into_inner() {
                match parse_dtype(expr_iter.clone()) {
                    Some(val) => exp_vec.push(val),
                    None => exp_vec.push( builtins_hash.get(expr_iter.as_str())
                    .unwrap_or_else(|| Throw!("Operation Not found")).to_owned())
                }
            };
            line_vec.push(Builtins::Expr { exp_type: ExpType::MATH_EXP,  expr: exp_vec  });
        },


        Rule::stdfn_expr => {
            let mut exp_vec = vec![];

            for expr_iter in line.clone().into_inner() {
                match expr_iter.as_rule() {
                    Rule::Std_fn => exp_vec.push(  builtins_hash.get(expr_iter.as_str()).unwrap().to_owned() ),
                    Rule::math_expr => exp_vec.push( parse_exprs(&expr_iter, builtins_hash)[0].to_owned() ),
                    Rule::conditional_expr => exp_vec.push( parse_exprs(&expr_iter, builtins_hash)[0].to_owned() ),
                    _ => exp_vec.push(  parse_dtype( expr_iter ).unwrap()  )
                }
            };

            line_vec.push(Builtins::Expr { exp_type: ExpType::STDFN_EXP,  expr: exp_vec  });
        },

        Rule::mem_inst_expr => {
            let mut exp_vec = vec![];

            for expr_iter in line.clone().into_inner() {
                match expr_iter.as_rule() {
                    Rule::MemInst => exp_vec.push(  builtins_hash.get(expr_iter.as_str()).unwrap().to_owned() ),
                    Rule::math_expr => exp_vec.push( parse_exprs(&expr_iter, builtins_hash)[0].to_owned() ),
                    _ => exp_vec.push(  parse_dtype( expr_iter ).unwrap()  )
                }
            };
            line_vec.push(Builtins::Expr { exp_type: ExpType::MEM_INST_EXP,  expr: exp_vec  });
        },

        Rule::conditional_expr => {
            let mut exp_vec = vec![];

            for expr_iter in line.clone().into_inner() {
                match expr_iter.as_rule() {
                    Rule::conditional_oper => exp_vec.push(  builtins_hash.get(expr_iter.as_str()).unwrap().to_owned() ),
                    _  => exp_vec.push(  parse_dtype( expr_iter ).unwrap()  ),
                }
            };
            line_vec.push(Builtins::Expr { exp_type: ExpType::CONDITION,  expr: exp_vec  });
        }

        Rule::jump_expr => {
            let mut n = 0;
            let mut cond = vec![];

            for val in line.clone().into_inner() {
                match val.as_rule() {
                    Rule::INT => {  n = val.as_str().parse::<i32>().unwrap();  },
                    Rule::BOOL => { cond.push( parse_dtype(val).unwrap() ); },

                    Rule::conditional_expr => {
                        for i in val.into_inner() {
                            match parse_dtype(i.clone()) {
                                Some(val) => cond.push(val),
                                None => cond.push( builtins_hash.get(i.as_str())
                                .unwrap_or_else(|| Throw!("Function Not found")).to_owned())
                            };
                        };
                    },

                    _ => continue
                };
            };
            line_vec.push(Builtins::JUMPIF { n , expr: cond});
        },

        damn => Throw!( format!("parse_exprs: Bruh --> {:?}\n", damn) )
    };

    return line_vec

}