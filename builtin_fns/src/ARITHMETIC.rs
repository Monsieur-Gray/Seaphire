use std::collections::HashMap;

use my_modules::defkeys::*;
use my_modules::fetch_data::fetch_Univ;

use crate::fetch_data::fetch_num;
use crate::Throw;

// fetch_univ(data) -> Result(f32 , String)
//      use .unwrap() to Ok value i.e. f32
//      use .unwrap_err() to Err value i.e. String

fn caxy_add(operands: &Vec<Builtins>,
    stack_hash: &HashMap<String, &Builtins>,
    heap_hash: &HashMap<String, &Builtins>
) -> f32 {
    let mut answer: f32 = 0.0;      
    for i  in operands {
        let num = match i {
            Builtins::D_type(_) => fetch_num(i).unwrap(),
            Builtins::ID(var) => {
                fetch_num( {
                if let Some(v1) = stack_hash.get(var) { 
                    v1 
                }
                else if let Some(v2) = heap_hash.get(var) { 
                    v2 
                }
                else { panic!("cry like a dog") }   
                }).unwrap()
            },

            other => Throw!( format!("ADD_2 isnt made for {:?}", other) )
        };
        answer += num;
    };
    return answer;
}

fn caxy_sub(operands: &Vec<Builtins>,               //SUBTRACTION
    stack_hash: &HashMap<String, &Builtins>,
    heap_hash: &HashMap<String, &Builtins>
) -> f32 {
    let mut answer: f32 = 2.0 * fetch_Univ(&operands[0]).unwrap();   // to adjust for the first number
    for i  in operands {
        let num = match i {
            Builtins::D_type(_) => fetch_num(i).unwrap(),
            Builtins::ID(var) => fetch_num( {

                if let Some(v1) = stack_hash.get(var) { v1 }
                else if let Some(v2) = heap_hash.get(var) { v2 }
                else { panic!("cry like a dog") }

                }).unwrap_or( Throw!( format!("SUB isnt made for {:?}", var) ) ),

            other => Throw!( format!("SUB isnt made for {:?}", other) )
        };
        answer -= num;
    };
    return answer;
}

fn caxy_mul(operands: &Vec<Builtins>,
    stack_hash: &HashMap<String, &Builtins>,
    heap_hash: &HashMap<String, &Builtins>
) -> f32 {
    let mut answer: f32 = 1.0;               // works fine for multiplication
    for i  in operands {
        let num = match i {
            Builtins::D_type(_) => fetch_num(i).unwrap(),
            Builtins::ID(var) => fetch_num( {

                if let Some(v1) = stack_hash.get(var) { v1 }
                else if let Some(v2) = heap_hash.get(var) { v2 }
                else { panic!("cry like a dog") }

                }).unwrap_or( Throw!( format!("MUL isnt made for {:?}", var) ) ),

            other => Throw!( format!("MUL isnt made for {:?}", other) )
        };
        answer *= num;
    };
    return answer;
}

fn caxy_div(operands: &Vec<Builtins>,   
    stack_hash: &HashMap<String, &Builtins>,
    heap_hash: &HashMap<String, &Builtins>
) -> f32 {
    let mut answer: f32 = fetch_Univ(&operands[0]).unwrap() * fetch_Univ(&operands[0]).unwrap();    // to adjust for the first number
    for i  in operands {
        let num = match i {
            Builtins::D_type(_) => fetch_num(i).unwrap(),
            Builtins::ID(var) => fetch_num( {

                if let Some(v1) = stack_hash.get(var) { v1 }
                else if let Some(v2) = heap_hash.get(var) { v2 }
                else { panic!("cry like a dog") }

                }).unwrap_or( Throw!( format!("DIV isnt made for {:?}", var) ) ),

            other => Throw!( format!("DIV isnt made for {:?}", other) )
        };
        
        if num == 0.0 { Throw!("Yedzhavya!, tujhya baapane 0 ne Divide kelela ka?"); }
        else { answer /= num; } //main!    
    };
    return answer;
}

//-------------------------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------------------------

pub fn perf_math(line: &Vec<Builtins>, 
    stack_hash: &HashMap<String, &Builtins>,
    heap_hash: &HashMap<String, &Builtins>
) -> f32{
    use colored::*;

    for i in &line[1..] {   
        match i {
            Builtins::D_type(D_type::int(_)) | Builtins::D_type(D_type::float(_)) | Builtins::ID(_) => true,
            // Builtins::MemInst(MemInst::INSERT)
            other => {
                Throw!( format!(
                    "\nINVALID SYNTAX ---> cant perform arithmetic on non-numerical values {:?}\n",
                    other
                ));
            }
        };
    };

    if line[0] == Builtins::Operation(Operation::ADD) {
        let ans = caxy_add(&Vec::from(&line[1..]), &stack_hash, heap_hash);

        println!("answer (+) ---= {}", 
            format!( "{:?}", ans)
            .on_truecolor(42, 42, 42).truecolor(150, 200, 255).bold()   );
        return ans;
    }

    else if line[0] == Builtins::Operation(Operation::SUB){
        let ans = caxy_sub(&Vec::from(&line[1..]), &stack_hash, heap_hash);

        println!("answer (-) ---= {}", 
            format!( "{:?}", ans)
            .on_truecolor(42, 42, 42).truecolor(150, 200, 255).bold()   );    
        return ans;
    }

    else if line[0] == Builtins::Operation(Operation::MUL){
        let ans = caxy_mul(&Vec::from(&line[1..]), &stack_hash, heap_hash);

        println!("answer (*) ---= {}", 
            format!( "{:?}", ans)
            .on_truecolor(42, 42, 42).truecolor(150, 200, 255).bold()   );  
        return ans; 
    }

    else if line[0] == Builtins::Operation(Operation::DIV){
        let ans = caxy_div(&Vec::from(&line[1..]), &stack_hash, heap_hash);

        println!("answer (/) ---= {}", 
            format!( "{:?}", ans)
            .on_truecolor(42, 42, 42).truecolor(150, 200, 255).bold()   );    
        return ans;
    }

    else {
        Throw!("Perfmath cant do shit")
    }
}
//-------------------------------------------------------------------------------------------------------------------------------------
//-------------------------------------------------------------------------------------------------------------------------------------

pub fn insert_math(line: &Vec<Builtins>,     // [Builtins::ID(oldval), D_type::newval]
    stack_hash: &HashMap<String, &Builtins>,
    heap_clone: &HashMap<String, &Builtins>
) -> Builtins
{
    if line[0] == Builtins::Operation(Operation::ADD) {
        let len_line = line.len();
        let ans = caxy_add(&Vec::from(&line[1..len_line-2]), &stack_hash, &heap_clone);

        let should_convert = match (line.last().unwrap(), ans){
            (Builtins::D_type(D_type::int(_)), _) => true,
            (Builtins::D_type(D_type::float(_)), _) => false,
            _ => panic!("insertmath_add:: I've not thought about this")
        };   // takes only the first part, the second part is ()

        if should_convert {
            Builtins::D_type(D_type::int(ans as i32))
        }
        else {
            Builtins::D_type(D_type::float(ans))
        }
    }
//----------------------SUB------------------------------------------------------
    // else if line[0] == Builtins::Operation(Operation::SUB){
    //     let len_line = line.len();
    //     let ans = caxy_add(&Vec::from(&line[1..len_line-2]), &stack_hash, &heap_clone);

    //     let var_to_modif = line.last().unwrap();
    //     let should_convert = match (var_to_modif, ans){
    //         (Builtins::D_type(D_type::int(_)), _) => (true, drop(var_to_modif)),
    //         (Builtins::D_type(D_type::float(_)), _) => (false, drop(var_to_modif)),
    //         _ => panic!("insertmath_add:: I've not thought about this")
    //     }.0 ;   // takes only the first part, the second part is ()

    //     let nans = if should_convert { &Builtins::D_type(D_type::int(ans as i32)) } 
    //             else {&Builtins::D_type(D_type::float(ans)) };
        
    //     let mut new_mut_hash: &'b HashMap<String, &Builtins> = heap_clone.borrow_mut();
    //     new_mut_hash.entry( fetch_str(var_to_modif).unwrap() )
    //     .and_modify(|e| *e = &nans);
        
    //     new_mut_hash
    // }

    // else if line[0] == Builtins::Operation(Operation::MUL){
    //     let len_line = line.len();
    //     let ans = caxy_add(&Vec::from(&line[1..len_line-2]), &stack_hash, &heap_clone);

    //     let var_to_modif = line.last().unwrap();
    //     let should_convert = match (var_to_modif, ans){
    //         (Builtins::D_type(D_type::int(_)), _) => (true, drop(var_to_modif)),
    //         (Builtins::D_type(D_type::float(_)), _) => (false, drop(var_to_modif)),
    //         _ => panic!("insertmath_add:: I've not thought about this")
    //     }.0 ;   // takes only the first part, the second part is ()

    //     let nans = if should_convert { &Builtins::D_type(D_type::int(ans as i32)) } 
    //             else {&Builtins::D_type(D_type::float(ans)) };
        
    //     heap_clone.borrow_mut()
    //     new_mut_hash.entry( fetch_str(var_to_modif).unwrap() )
    //     .and_modify(|e| *e = &nans);
        
    //     new_mut_hash
    // }

    // else if line[0] == Builtins::Operation(Operation::DIV){
    //     let len_line = line.len();
    //     let ans = caxy_add(&Vec::from(&line[1..len_line-2]), &stack_hash, &heap_clone);

    //     let var_to_modif = line.last().unwrap();
    //     let should_convert = match (var_to_modif, ans){
    //         (Builtins::D_type(D_type::int(_)), _) => (true, drop(var_to_modif)),
    //         (Builtins::D_type(D_type::float(_)), _) => (false, drop(var_to_modif)),
    //         _ => panic!("insertmath_add:: I've not thought about this")
    //     }.0 ;   // takes only the first part, the second part is ()

    //     let nans = if should_convert { &Builtins::D_type(D_type::int(ans as i32)) } 
    //             else {&Builtins::D_type(D_type::float(ans)) };
        
    //             let mut new_mut_hash: &'b HashMap<String, &Builtins> = heap_clone.borrow_mut();        
    //     new_mut_hash.entry( fetch_str(var_to_modif).unwrap() )
    //     .and_modify(|e| *e = &nans);
        
    //     new_mut_hash
    // }
    else {
        Throw!("Perfmath cant do shit")
    }
}

