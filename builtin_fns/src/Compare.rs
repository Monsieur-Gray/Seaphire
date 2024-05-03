use crate::fetch_data::fetch_bool;
use crate::fetch_data::get_val;
use my_modules::defkeys::*;
use my_modules::Throw;
use std::cmp::Ordering;

pub fn eval_condition(
    logic: &Vec<Builtins>,
    stack_hash: &std::collections::HashMap<String, Builtins>,
    heap_hash: &std::collections::HashMap<String, Builtins>,
    reg_hash: &std::collections::HashMap<String, Builtins>,
) -> Option<bool> {
// Conditional = [x > y] or [x==y]
// Logical = [ x>y || [x==y] ]

    if logic[0].unwrap_expr_vec().unwrap().len() == 1 {       // For safety
        match logic[0].get_expression_type().unwrap() {
            ExpType::CONDITION => {
                return Some(make_tree(&logic[0], stack_hash, heap_hash, reg_hash));
            },
            ExpType::LOGIC_EXP => {
                match logic[0].unwrap_expr_vec().unwrap()[0] .get_expression_type().unwrap() {
                    ExpType::CONDITION => {
                        return Some(make_tree(&logic[0].unwrap_expr_vec().unwrap()[0], stack_hash, heap_hash, reg_hash));
                    },
                    _ => Throw!("Creating redundant CONDITIONAL_EXPRESSIONS is not valid")
                }
            },
            _ => Throw!("Creating redundant CONDITIONAL_EXPRESSIONS is not valid")
        }
    }
    else {
        let lhs = &logic[0];
        let rhs = &logic[2];

        let a = make_tree(lhs, stack_hash, heap_hash, reg_hash);
        let b = make_tree(rhs, stack_hash, heap_hash, reg_hash);
    // println!("  a -> {:?}\n", a);
    // println!("  b -> {:?}\n", b);
        let ans = match logic[1] {          // The operator (&& ||)
            Builtins::Logic(Logical_Op::AND) => a && b,
            Builtins::Logic(Logical_Op::OR) => a || b,
            _ => Throw!("eval_condition")
        };
        return Some(ans);
    }
}

//--------------------------------------------------------------------------------------------------------

fn make_tree(expr: &Builtins, 
    stack_hash: &std::collections::HashMap<String, Builtins>,
    heap_hash: &std::collections::HashMap<String, Builtins>,
    reg_hash: &std::collections::HashMap<String, Builtins>,
) -> bool {

    let outp = match expr.get_expression_type() {
        Ok(eT) => {
            match eT {
                ExpType::LOGIC_EXP => {
                    let ex_vec = expr.unwrap_expr_vec().unwrap();
                    let a = &ex_vec[0];
                    let b = &ex_vec[2];

                    let eval_a = match a.get_expression_type().unwrap() {
                        ExpType::LOGIC_EXP => { make_tree(a, stack_hash, heap_hash, reg_hash) },
                        ExpType::CONDITION => { damedame(a.unwrap_expr_vec().unwrap(), stack_hash, heap_hash, reg_hash) },
                        _ => Throw!("booyah a")
                    };

                    let eval_b = match b.get_expression_type().unwrap() {
                        ExpType::LOGIC_EXP => { make_tree(b, stack_hash, heap_hash, reg_hash) },
                        ExpType::CONDITION => { damedame(b.unwrap_expr_vec().unwrap(), stack_hash, heap_hash, reg_hash) },
                        _ => Throw!("booyah b")
                    };
                    match &ex_vec[1] {
                        Builtins::Logic(Logical_Op::OR) => eval_a || eval_b,
                        Builtins::Logic(Logical_Op::AND) => eval_a && eval_b,
                        _ => Throw!("maketree:: shit thorw"),
                    }
                },

                ExpType::CONDITION => {
                    let c = expr.unwrap_expr_vec().unwrap();
                    damedame(c, stack_hash, heap_hash, reg_hash)
                },         
                _ => panic!("make_tree:: bagua")      
            }
        },
        Err(_) => Throw!("maketree:: nah")
    };
    // println!("outp ----> {:?}\n", get_val(&Builtins::ID("hi".to_string()), stack_hash, heap_hash, reg_hash));
    outp
}

//--------------------------------------------------------------------------------------------------------
fn damedame(
    condition: &Vec<Builtins>,
    stack_hash: &std::collections::HashMap<String, Builtins>,
    heap_hash: &std::collections::HashMap<String, Builtins>,
    reg_hash: &std::collections::HashMap<String, Builtins>,
) -> bool {
    
    if condition.len() == 1 && fetch_bool(condition.get(0).unwrap()).unwrap() {
        return fetch_bool(condition.get(0).unwrap()).unwrap();
    } else {
        let lhs = get_val(condition.get(0).unwrap(), stack_hash, heap_hash, reg_hash).unwrap();
        let rhs = get_val(condition.get(2).unwrap(), stack_hash, heap_hash, reg_hash).unwrap();

        let oper = condition.get(1).unwrap();
        let ans = lhs.partial_cmp(&rhs).unwrap();        
        let eval_expr = match (oper, ans) {
            (Builtins::CMP(CompOp::EQUAL), Ordering::Equal) => true,
            (Builtins::CMP(CompOp::LESS), Ordering::Less) => true,
            (Builtins::CMP(CompOp::GREATER), Ordering::Greater) => true,
            (Builtins::CMP(CompOp::UNEQUAL), Ordering::Less | Ordering::Greater) => true,

            _ => false,
        };

        return eval_expr;
    }
}
