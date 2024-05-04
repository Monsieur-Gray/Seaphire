use my_modules::defkeys::*;
use colored::Colorize;

pub fn get_parsed_inp(exp_line: &Vec<Builtins>) -> Builtins {
    
    let prompt = match &exp_line[1] {
        Builtins::D_type( D_type::str( S ) ) => S.replace('\'', ""),
        er => crate::Throw!(format!("Can't make this as the prompt -> {:?}", er))
    };
    
    println!("{}", prompt.truecolor(100, 150, 100).bold());

    use std::io;
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Unable to get prompt");
    let inp = inp.trim();

    match inp.parse::<bool>() {
        Ok(b) => Builtins::D_type(D_type::bool(b)),

        Err(_) => match inp.parse::<i32>() {
            Ok(i) => Builtins::D_type(D_type::int(i)),

            Err(_) => match inp.parse::<f32>() {
                Ok(f) => Builtins::D_type(D_type::float(f)),
                Err(_) => Builtins::D_type(D_type::str(inp.to_string()))
            }
        }
    } 
}