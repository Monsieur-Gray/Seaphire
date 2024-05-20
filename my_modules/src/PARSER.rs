use std::collections::HashMap;
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;
use crate::SysThrow;
use crate::Throw;
use crate::defkeys::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CaxyParser;

pub fn pest_parse(unparsed_filestr: &String) -> (Option<Pair<Rule>>, Option<Pair<Rule>>){
    let file = match CaxyParser::parse(Rule::file, unparsed_filestr.as_str()) {
        Ok(mut outp) => outp.next().unwrap(),
        Err(fucking_error) => crate::SysThrow!(format!( "SyntaxError at line: {}\n\nNo matching expression with the following format found" ,fucking_error ))
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


pub fn calloc(vinp: Option<Pair<Rule>>) -> [HashMap<String, Value>; 3] {
    let mut line_num = -1;  // consider the extra first term (number of variables)
    let mut vsec_size = 0;

    let mut stack_hash: HashMap<String, Value> = HashMap::new();
    let mut heap_hash: HashMap<String, Value> = HashMap::new();

    for i in vinp.unwrap().into_inner() {
        match i.as_rule() {
            Rule::INT => {vsec_size = i.as_str().parse::<i32>().unwrap();},
            Rule::var_make => {
                let mut memtype = "";
                let mut id = String::new();
                let mut data = Builtins::Comment;

                for j in i.clone().into_inner() {
                    let jstr = j.as_str();

                    match j.as_rule() {
                        Rule::MemType => { memtype = jstr; },
                        Rule::ID => { id = jstr.to_string(); },
                        sometype => data = {
                            match (sometype, memtype) {
                                (Rule::INT, "int") => Builtins::D_type(D_type::int(jstr.parse::<i32>().unwrap())),
                                (Rule::FLOAT, "float") => Builtins::D_type(D_type::float(jstr.parse::<f32>().unwrap())),
                                (Rule::BOOL, "bool") => Builtins::D_type(D_type::bool(jstr.parse::<bool>().unwrap())),
                                (Rule::STRLIT, "str") => Builtins::D_type(D_type::str(jstr.to_string())),

                                (dtyp, vtyp) => Throw!( format!("VariableType [{:?}] and value of the variable [{:?}] do not match", vtyp, dtyp))
                            }
                         }
                    };

                };

                if id.starts_with('?') {
                    id.remove(0);
                    heap_hash.insert(
                    id, 
                    Value {
                        value: data,
                        scope: Scope::GlobalScope
                    }
                    );
                }
                else {
                    stack_hash.insert(
                        id, 
                        Value {
                            value: data,
                            scope: Scope::GlobalScope
                        }
                        );
    
                };
            },
            _ => continue
        };
        line_num += 1;
    };

    if line_num != vsec_size {  Throw!("Incorrect shit of variables");  }
    else {  
        let reg_hash: HashMap<String, Value> = HashMap::new();
        return [stack_hash, heap_hash, reg_hash];
    };
}

pub fn make_msec(msec: Option<Pair<Rule>>) -> Vec<Builtins> {
    let mut MAIN_SEC: Vec<Builtins> = vec![];
    let builtins_hash = Builtins::builtin_hash();

    for line in msec.unwrap().into_inner() {
        let line_vec = parse_exprs(&line, &builtins_hash, 0)
                            .unwrap_expr_vec().unwrap()[0].to_owned();
         // 0 because it will be the GlobalScope / Main Scope
        
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
        Rule::STRLIT => Some( Builtins::D_type( D_type::str(pstr.replace("\'", "")) ) ),
        Rule::REGISTER => Some(  Builtins::REGISTER(pstr.to_string()) ),
        Rule::ID =>     Some(  Builtins::ID    ( pstr.to_string() )   ),
        _ => None
        // bum =>  Throw!(format!("Bum -> {:?}", (bum, pstr) )),
    };
    return output;
}


//----------------------------------------------------------------------------------------------------------------------------------------------------------------------
fn parse_exprs(line: &Pair<Rule>, builtins_hash: &HashMap<String, Builtins>, scope_counter: u32) -> Builtins {
    let mut line_vec = vec![];
    // let mut scope_counter: u32 = 0;

    match line.as_rule() {

        Rule::math_expr  => {
            let mut exp_vec: Vec<Builtins> = vec![];

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
            let mut exp_vec: Vec<Builtins> = vec![];
            
            for expr_iter in line.clone().into_inner() {
                match expr_iter.as_rule() {
                    Rule::Std_fn | Rule::logical_oper => exp_vec.push(  builtins_hash.get(expr_iter.as_str()).unwrap().to_owned() ),

                    Rule::math_expr | Rule::logical_expr=> exp_vec.push(
                        parse_exprs(&expr_iter, builtins_hash, scope_counter)
                            .unwrap_expr_vec().unwrap()[0].to_owned()
                    ),
                    
                    Rule::condition => exp_vec.push(
                        parse_exprs(&expr_iter, builtins_hash, scope_counter)
                            .unwrap_expr_vec().unwrap()[0].to_owned()
                    ),
                    _ => exp_vec.push(  parse_dtype( expr_iter ).unwrap()  )
                }
            };

            line_vec.push(Builtins::Expr { exp_type: ExpType::STDFN_EXP,  expr: exp_vec  });
        },

        Rule::mem_inst_expr => {
            let mut exp_vec: Vec<Builtins> = vec![];

            for expr_iter in line.clone().into_inner() {
                match expr_iter.as_rule() {
                    Rule::MemInst => exp_vec.push(  builtins_hash.get(expr_iter.as_str()).unwrap().to_owned() ),

                    Rule::math_expr | Rule::stdfn_expr => exp_vec.push(
                        parse_exprs(&expr_iter, builtins_hash, scope_counter)
                            .unwrap_expr_vec().unwrap()[0].to_owned()
                    ),
                    _ => exp_vec.push(  parse_dtype( expr_iter ).unwrap()  )
                }
            };
            line_vec.push(Builtins::Expr { exp_type: ExpType::MEM_INST_EXP,  expr: exp_vec  });
        },

        Rule::jumpif_expr => {
            let mut n = 0;
            let mut cond: Vec<Builtins> = vec![];

            for val in line.clone().into_inner() {
                match val.as_rule() {
                    Rule::INT => {  n = val.as_str().parse::<i32>().unwrap();  },
                    Rule::BOOL => cond.push( parse_dtype(val).unwrap() ),

                    Rule::logical_expr => cond.push(
                        parse_exprs(&val, builtins_hash, scope_counter)
                            .unwrap_expr_vec().unwrap()[0].to_owned()
                    ),

                    _ => continue
                };
            };
            line_vec.push(Builtins::JUMPIF { n , expr: cond});
        },

    //-----------IF-----------------ELIF-------------ELSE--------------------------------------
        Rule::if_expr => {
            let mut exp_vec = vec![];
            for expr_iter in line.clone().into_inner() {
                match expr_iter.as_rule() {
                    Rule::BOOL | Rule::ID  =>  exp_vec.push(parse_dtype(expr_iter).unwrap()),
                    _ => {
                        let x = &parse_exprs(
                            &expr_iter, builtins_hash, scope_counter
                        ).unwrap_expr_vec().unwrap()[0].to_owned();
                        
                        exp_vec.push( x.clone() );
                    },
                };
            };
            line_vec.push( Builtins::Expr { exp_type: ExpType::IF_EXP, expr: exp_vec } )
        },

        Rule::else_expr => {
            let mut exp_vec = vec![];
            for expr_iter in line.clone().into_inner() 
            {
                exp_vec.push(
                    parse_exprs(&expr_iter, builtins_hash, scope_counter)
                        .unwrap_expr_vec().unwrap()[0].to_owned()
                );
            };
            line_vec.push( Builtins::Expr { exp_type: ExpType::ELSE_EXP, expr: exp_vec } )
        },

        Rule::elif_expr => {
            let mut exp_vec = vec![];
            for expr_iter in line.clone().into_inner() 
            {
                exp_vec.push(
                    parse_exprs(&expr_iter, builtins_hash, scope_counter)
                        .unwrap_expr_vec().unwrap()[0].to_owned()
                );
            };
            line_vec.push( Builtins::Expr { exp_type: ExpType::ELIF_EXP, expr: exp_vec } )
        },


        Rule::if_else_expr => {
            let mut exp_vec = vec![];
            for expr_iter in line.clone().into_inner() {
                // println!("\t---> {:?}\n", expr_iter.as_rule());
                exp_vec.push(
                    parse_exprs(&expr_iter, builtins_hash, scope_counter)
                        .unwrap_expr_vec().unwrap()[0].to_owned()
                );
            };
            line_vec.push( Builtins::Expr { exp_type: ExpType::IF_ELSE_EXP, expr: exp_vec } )
        },

        Rule::if_elif_expr => {
            let mut exp_vec = vec![];
            for expr_iter in line.clone().into_inner() {
                // println!("\t---> {:?}\n", expr_iter.as_rule());
                exp_vec.push(
                        parse_exprs(&expr_iter, builtins_hash, scope_counter)
                            .unwrap_expr_vec().unwrap()[0].to_owned()
                    );
            };
            line_vec.push( Builtins::Expr { exp_type: ExpType::IF_ELIF_EXP, expr: exp_vec } )
        },


        Rule::logical_expr => {
            let mut exp_vec: Vec<Builtins> = vec![];

            for expr_iter in line.clone().into_inner() {
                match expr_iter.as_rule() {

                    Rule::condition | Rule::logical_expr => exp_vec.push(
                        parse_exprs(&expr_iter, builtins_hash, scope_counter)
                            .unwrap_expr_vec().unwrap()[0].to_owned()
                        ),

                    Rule::logical_oper => exp_vec.push(  builtins_hash.get(expr_iter.as_str()).unwrap().to_owned() ),
                    _  => continue,
                }
            };
            line_vec.push(Builtins::Expr { exp_type: ExpType::LOGIC_EXP,  expr: exp_vec  });
        }

        Rule::condition => {
            let mut exp_vec: Vec<Builtins> = vec![];
            for expr_iter in line.clone().into_inner() {
                match expr_iter.as_rule() {
                    Rule::conditional_oper => exp_vec.push(  builtins_hash.get(expr_iter.as_str()).unwrap().to_owned() ),
                    _  => exp_vec.push(  
                        parse_dtype( expr_iter ).unwrap()
                  ),
                }
            };
            line_vec.push(Builtins::Expr { exp_type: ExpType::CONDITION,  expr: exp_vec  });
        },

        Rule::InnerScope => {
            let mut block: Vec<Builtins> = vec![];
            let mut inner_vvec: Vec<Builtins> = vec![];

            for expr_iter in line.clone().into_inner() {

                match expr_iter.as_rule() {

                    Rule::inner_vsec => {
                        for l in expr_iter.clone().into_inner() {
                            let x = parse_exprs(&l, builtins_hash, scope_counter+1)
                                    .unwrap_expr_vec().unwrap()[0].to_owned();
                            inner_vvec.push( x );
                        };
                    },

                    Rule::inner_msec => {
                        for l in expr_iter.clone().into_inner() {
                            let x = parse_exprs(&l, builtins_hash, scope_counter+1)
                                .unwrap_expr_vec().unwrap()[0].to_owned();

                            block.push( x );
                        };
                    },

                    errstuff => Throw!(
                        format!("parse_exp::InnerScope   This bullshit is not allowed ---> {:?}", errstuff)
                    )
                };
            };   

            let i_vsec = if inner_vvec.len() != 0 { 
                Some(inner_vvec) 
            } else { 
                None
            };

            line_vec.push(Builtins::InnerScope { 
                inner_vsec: i_vsec,
                block,  
                scope: Scope::Local(scope_counter+1)  
            });
        },
         
         
        Rule::local_var_make => {
            let mut exp_vec = vec![];
            let mut memtype = "";

            for expr_iter in line.clone().into_inner() {
                let jstr = expr_iter.as_str();
                match expr_iter.as_rule() {
                    Rule::MemType => { 
                        memtype = jstr; 
                    },

                    Rule::ID => { 
                        exp_vec.push( Builtins::ID( jstr.to_string() ) );
                    },

                    sometype => exp_vec.push({
                        match (sometype, memtype) {
                            (Rule::INT, "int") => Builtins::D_type(D_type::int(jstr.parse::<i32>().unwrap())),
                            (Rule::FLOAT, "float") => Builtins::D_type(D_type::float(jstr.parse::<f32>().unwrap())),
                            (Rule::BOOL, "bool") => Builtins::D_type(D_type::bool(jstr.parse::<bool>().unwrap())),
                            (Rule::STRLIT, "str") => Builtins::D_type(D_type::str(jstr.to_string())),
    
                            (dtyp, vtyp) => Throw!( format!("VariableType [{:?}] and value of the variable [{:?}] do not match", vtyp, dtyp))
                        }
                    })
                };
            };

            line_vec.push( 
                Builtins::Expr { exp_type: ExpType::LOCAL_VAR_MAKE, 
                expr: exp_vec } 
            )

        },

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        Rule::Comment => line_vec.push(Builtins::Comment),
        // Rule::ErrLine  => Throw!( format!("SyntaxError in the following expression -!> {:?}\n", line.as_str() ) ),
        _duh => SysThrow!( format!("I don't have any idea how you fucked up this bad (parse_exprs) >>>> {:?}", line) )
    };

    
    let line_vec_with_scope = Builtins::InnerScope {
        inner_vsec: None,
        block: line_vec,
        scope: {
            match scope_counter {
                0 => Scope::GlobalScope,
                other => Scope::Local(other)
            }
        }
    };

    return line_vec_with_scope;

}

