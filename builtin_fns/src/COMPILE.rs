// #![allow(unused)]
use std::collections::HashMap;

use my_modules::defkeys::*;
use crate::fetch_data::MyRes;

use crate::fetch_data::{fetch_MyRes, fetch_str};
use crate::Throw;

use crate::ARITHMETIC::perf_math;


fn gen_vars(
    stack_hash: &HashMap<String, Builtins>,
    heap_hash: &HashMap<String, Builtins>
) -> (String, String) {

    let mut stack_buff = String::new();
    for (nam, val) in stack_hash.iter() {
        let line = match fetch_MyRes(val) {
            MyRes::Int(i) => String::from(format!("\tint {} = {} ;\r", nam, i).as_str()),
            MyRes::Flt(f) => String::from(format!("\tfloat {} = {}f ;\r", nam, f).as_str()),
            MyRes::Str(s) => String::from(format!("\tstd::string {} = \"{}\" ;\r", nam, s).as_str()),
            MyRes::Bool(b) => String::from(format!("\tbool {} = {} ;\r", nam, b).as_str()),
        };
        
        stack_buff.push_str(&line);
    };
  //------------------------------------------------------------------------------------------------------
    let mut heap_buff = String::new();
    for (nam, val) in heap_hash.iter() {
        let line = match fetch_MyRes(val) {
            MyRes::Int(i) => String::from(format!("\tint {} = {}; \r", nam, i).as_str()),
            MyRes::Flt(f) => String::from(format!("\tfloat {} = {}f; \r", nam, f).as_str()),
            MyRes::Str(s) => String::from(format!("\tstd::string {} = \"{}\"; \r", nam, s).as_str()),
            MyRes::Bool(b) => String::from(format!("\tbool {} = {}; \r", nam, b).as_str()),
        };
        
        heap_buff.push_str(&line);
    };

    return (stack_buff, heap_buff);
}


pub fn compile_to_cpp(
    block: &Vec<Vec<Builtins>>,
    stack_hash: HashMap<String, Builtins>,
    heap_hash:HashMap<String, Builtins>
) {
    let mut main_buff = String::new();
    let (sbuff, mut hbuff) = gen_vars(&stack_hash, &heap_hash);

    for inp_line in block {
        match &inp_line[0] {
            Builtins::Operation(_) => {
                let math_out = perf_math(&inp_line, &stack_hash, &heap_hash);
                main_buff.push_str(format!("\tstd::cout << {:?} << std::endl;\r", math_out).as_str())

            },
                
//STANDARD FUNCTIONS-------------------------
        Builtins::Std_fns(func) => match func {
            Std_fns::PRNT => {
                let prnt_out = crate::PRNT::print_line(&inp_line, &stack_hash, heap_hash.clone() );
                main_buff.push_str(format!("\tstd::cout << {} << std::endl;\r", prnt_out.replace('"', "")).as_str())
            },
            _ => println!("stdfns quecera")        
        },
                    
//MEMORY INSTRUCTIONS-------------------------
        Builtins::MemInst(inst) => {
            
            match inst {
                MemInst::MOV => {
                    hbuff = mov_mem(&inp_line, hbuff);
                },
                _ => println!("other instr will be later")
            };
        },

        //ERROR HANDLING--------------------------------
            Builtins::Comment => (),
            other => Throw!(format!("UNIMPLEMENTED FUNCTIONALITY -->{:?}", other))
        };
    }

    use std::fs;

    let mut final_output = String::from("#include <iostream>\r\rint main() { \r");

    final_output.push_str(format!("{}\r{}\r", sbuff, hbuff).as_str());

    final_output.push_str(&main_buff);
    final_output.push_str("\treturn 0;\r}");

    let _ = fs::write("caxy_out.cpp", final_output.as_bytes());
}


//------------------------------------------
fn mov_mem(inp_line: &Vec<Builtins>, hbuff: String) -> String {

    let nam = fetch_str(inp_line.get(1).unwrap()).unwrap();
    let v = inp_line.last().unwrap();
    let mut hbuff = hbuff; 

    for line in hbuff.clone().lines() {
        if line[1] == &nam.as_str() { // if the given variable is in heap
            let nline = match fetch_MyRes(v) {
                MyRes::Int(i) => String::from(format!("\tint {} = {} ;\r", nam, i).as_str()),
                MyRes::Flt(f) => String::from(format!("\tfloat {} = {}f ;\r", nam, f).as_str()),
                MyRes::Str(s) => String::from(format!("\tstd::string {} = \"{}\" ;\r", nam, s).as_str()),
                MyRes::Bool(b) => String::from(format!("\tbool {} = {} ;\r", nam, b).as_str()),
            };
            hbuff = hbuff.replace(line, &nline);
        }
        else {
            Throw!(format!("No mutable variable named '{nam}' found!"));
        };
    };
    return hbuff;
}