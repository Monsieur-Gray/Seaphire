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
            MyRes::Int(i) => String::from(format!("\t{} DD {} \r", nam, i).as_str()),
            MyRes::Flt(f) => String::from(format!("\t{} DQ {}\r", nam, f).as_str()),
            MyRes::Str(s) => String::from(format!("\t{} DB '{}'\r", nam, s).as_str()),
            MyRes::Bool(b) => {
                let _tmp = if b {0} else {1};
                String::from(format!("\t{} DD {} \t\t\r", nam, _tmp).as_str())
            },
        };
        
        stack_buff.push_str(&line);
    };
  //------------------------------------------------------------------------------------------------------
    let mut heap_buff = String::new();
    for (nam, val) in heap_hash.iter() {
        let line = match fetch_MyRes(val) {
            MyRes::Int(i) => String::from(format!("\t{} DD {} \r", nam, i).as_str()),
            MyRes::Flt(f) => String::from(format!("\t{} DQ {}\r", nam, f).as_str()),
            MyRes::Str(s) => String::from(format!("\t{} DB '{}'\r", nam, s).as_str()),
            MyRes::Bool(b) => {
                let _tmp: u32 = if b {0} else {1};
                String::from(format!("\t{} DD {} \t\t; boolean\r", nam, _tmp).as_str())
            },
        };
        
        heap_buff.push_str(&line);
    };

    return (stack_buff, heap_buff);
}


pub fn compile_to_asm(
    block: &Vec<Vec<Builtins>>,
    stack_hash: HashMap<String, Builtins>,
    heap_hash:HashMap<String, Builtins>
) {
    let mut main_buff = String::new();
    let (sbuff, mut hbuff) = gen_vars(&stack_hash, &heap_hash);
    let newline_var = String::from("\tnewline db 10\r");

    for inp_line in block{
        // let inp_line = block.get(line_num as usize).unwrap();
        match &inp_line[0] {
            Builtins::Operation(_) => {
                let math_out = perf_math(&inp_line, &stack_hash, &heap_hash);
                main_buff.push_str(gen_prnt_asm(math_out.to_string()).as_str() );
            },
                
        //STANDARD FUNCTIONS-------------------------
        Builtins::Std_fns(func) => match func {
            Std_fns::PRNT => {
                let prnt_out = crate::PRNT::print_line(&inp_line, &stack_hash, heap_hash.clone() );
                main_buff.push_str(gen_prnt_asm(prnt_out).as_str() );
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
    };

    use std::fs;

    let mut final_output = String::new();

    final_output.push_str("section .data\r");
    
    final_output.push_str(format!("{}\r ", sbuff).as_str());
    final_output.push_str(format!("{}\r ", hbuff).as_str());
    final_output.push_str(format!("{}\r", newline_var).as_str());

    final_output.push_str("section .text\r\tglobal _start\r\r_start:\r");

    final_output.push_str(&main_buff);

    final_output.push_str(format!("{}\r{}\r{}", 
        "mov rax, 60",
        "mov rdi, 0",
        "syscall")
    .as_str());

    // println!("-----------------Asm file is:- {}", &final_output);
    let _ = fs::write("heythere.asm", final_output.as_bytes());

}


//------------------------------------------
fn mov_mem(inp_line: &Vec<Builtins>, hbuff: String) -> String{

    let nam = fetch_str(inp_line.get(1).unwrap()).unwrap();
    let v = inp_line.last().unwrap();
    let mut hbuff = hbuff; 

    for line in hbuff.clone().lines() {
        if line.contains(&nam.as_str()) { // if the given variable is in heap
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

//-------------------------

fn gen_prnt_asm(val: String) -> String{

    let mut out = String::new();

    out.push_str("\tmov rax, 1\r\tmov rdi, 1\r");
    out.push_str(format!("\tlea rsi, [{}]\r", val).as_str());
    out.push_str(format!("\tmov rdx, {}\r", val.len()).as_str());
    out.push_str("syscall\r");

    out.push_str("\tmov rax, 1\r\tmov rdi, 1\r");
    out.push_str("\tlea rsi, [newline]\r");
    out.push_str("\tmov rdx, 1\r");
    out.push_str("syscall\r\r");

    return out;
}