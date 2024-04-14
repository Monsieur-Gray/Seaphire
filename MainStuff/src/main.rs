#![allow(non_snake_case)]
#![allow(unused)]

use my_modules::*;
use my_modules::defkeys::Builtins;

use builtin_fns::EXECUTE::check_exec_line;
// use builtin_fns::COMPILE::compile_to_cpp;
// use builtin_fns::compile_ass::compile_to_asm;

fn main() {
    let text = read_file::read_from_str("f1.caxy");

    let CODE: Vec<Vec<Builtins>> = get_type::get_type(text);

    let (_msec, _vsec) = split_sec::split_code(&CODE);
    
    let (mut STACK, mut HEAP) = mem_alloc::calloc(_vsec);
        
// //--=-=-=-=-==-=-=-==============------------------------------------=-=-=-=-=-==-=-------------------------------
    println!("<================-================>");
    check_exec_line(&_msec, STACK, HEAP);
    // compile_to_cpp(&_msec, STACK, HEAP);
    // compile_to_asm(&_msec, STACK, HEAP);
    println!("<================-================>");
//--=-=-=-=-==-=-=-==============------------------------------------=-=-=-=-=-==-=-------------------------------

}
