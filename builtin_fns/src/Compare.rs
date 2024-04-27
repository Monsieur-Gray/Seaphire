use crate::fetch_data::fetch_bool;
use crate::fetch_data::get_val;
use my_modules::defkeys::*;
use my_modules::Throw;
use std::cmp::Ordering;

pub fn eval_condition(
    logic: &Vec<Builtins>,
    stack_hash: &std::collections::HashMap<String, Builtins>,
    heap_hash: &std::collections::HashMap<String, Builtins>,
) -> Option<bool> {
// Conditional = [x > y] or [x==y]
// Logical = [ x>y || [x==y] ]

    if logic.len() == 1 {       // For safety
        match logic[0].get_expression_type().unwrap() {
            ExpType::CONDITION => {
                return Some(make_tree(&logic[0], stack_hash, heap_hash));
            },
            _ => Throw!("Creating redundant CONDITIONAL_EXPRESSIONS is not valid")
        }
    }
    else {
        let a = make_tree(&logic[0], &stack_hash, &heap_hash);
        let b = make_tree(&logic[2], &stack_hash, &heap_hash);

        let ans = match logic[1] {          // The operator (&& ||)
            Builtins::Logic(Logical_Op::AND) => a && b,
            Builtins::Logic(Logical_Op::OR) => a || b,
            _ => Throw!("eval_condition")
        };
        return Some(ans);
    }
}

fn damedame(
    condition: &Vec<Builtins>,
    stack_hash: &std::collections::HashMap<String, Builtins>,
    heap_hash: &std::collections::HashMap<String, Builtins>,
) -> bool {
    
    if condition.len() == 1 && fetch_bool(condition.get(0).unwrap()).unwrap() {
        return fetch_bool(condition.get(0).unwrap()).unwrap();
    } else {
        let lhs = get_val(condition.get(0).unwrap(), &stack_hash, &heap_hash).unwrap();
        let rhs = get_val(condition.get(2).unwrap(), &stack_hash, &heap_hash).unwrap();

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
fn make_tree(expr: &Builtins, 
    stack_hash: &std::collections::HashMap<String, Builtins>,
    heap_hash: &std::collections::HashMap<String, Builtins>
) -> bool {
    match expr.get_expression_type() {
        Ok(eT) => {

            match eT {
                ExpType::LOGIC_EXP => {
                    let ex_vec = expr.unwrap_expr_vec().unwrap();
                    let a = &ex_vec[0];

                    match a.get_expression_type().unwrap() {
                        ExpType::LOGIC_EXP => { make_tree(a, stack_hash, heap_hash) },
                        ExpType::CONDITION => { damedame(a.unwrap_expr_vec().unwrap(), stack_hash, heap_hash) },
                        _ => Throw!("booyah")
                    }

                },
                ExpType::CONDITION => {
                    let c = expr.unwrap_expr_vec().unwrap();
                    return damedame(c, stack_hash, heap_hash);
                },         
                _ => panic!("bagua")      
            }
        },
        Err(_) => Throw!("nah")
    }
}
