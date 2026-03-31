use SysBase::defkeys::*;
use crate::Throw;

pub fn math_to_cpp(math_expr: &Vec<Builtins>) -> String{

    let operator = match &math_expr[0] {     // Operator
        Builtins::Operation( Operation::ADD ) => {
            "+"
        }
        Builtins::Operation( Operation::SUB ) => {
            "-"
        }
        Builtins::Operation( Operation::MUL ) => {
            "*"
        }
        Builtins::Operation( Operation::DIV ) => {
            "/"
        },

        err => Throw!(format!("compiler::maths | Unsupported shit --> {:?}", err))
    };


    let mut output_str = String::new();

    for operand in math_expr[1..].iter() {
        let math_out = match operand {
            Builtins::D_type( D_type::int(i) ) => format!("{} {}", i, operator),
            Builtins::D_type( D_type::float(f) ) => format!("{} {}", f, operator),
            Builtins::ID( id ) => format!("{} {} ", id, operator),
            Builtins::REGISTER( reg ) => {
                let new_reg = format!( "REG{}" ,&reg[1..]);
                format!(" {} {}", new_reg, operator)
            },
            _ => crate::Throw!("Data type not supported")
        };
        output_str.push_str(&math_out);
    }

    output_str.remove(output_str.len()-1);
    return output_str;  // To account for the extra operator and whitespace placed
//eg;- 10 + 23 +
}


pub fn data_to_string(builtin: &Builtins) -> String {
    match builtin {
        Builtins::D_type(D_type::bool(b)) => return b.to_string(),
        Builtins::D_type(D_type::int(i)) => return i.to_string(),
        Builtins::D_type(D_type::float(f)) => return f.to_string(),
        Builtins::D_type(D_type::str(s)) => return s.to_string(),
        Builtins::ID(id) => return id.to_owned(),
        Builtins::REGISTER(reg) => return format!("REG_{}", reg),
        
        other => crate::Throw!(
            format!("The following shit is not a datatype => {:?}", other)
        )
    }
}