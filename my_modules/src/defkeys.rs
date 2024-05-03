use std::collections::HashMap;

// use crate::Throw;

#[derive(Debug, PartialEq, Clone)] 
pub enum D_type {
    int(i32),
    str(String),
    float(f32),
    bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Std_fns {PRNT_COOL, PRNT_PLAIN, SINPUT}     // Standard - Builtin functions

#[derive(Debug, PartialEq, Clone)]
pub enum Operation { ADD, SUB, MUL, DIV}       // Arithmetic operations

#[derive(Debug, PartialEq, Clone)]
pub enum Section {VARS, END, MAIN, EOS}        // Sections keywords

#[derive(Debug, PartialEq, Clone)]          
pub enum MemType {int, float, str, bool}    // Memory Types

#[derive(Debug, PartialEq, Clone)]          
pub enum ID{ id(String) }               // For Variables or other data_types

#[derive(Debug, PartialEq, Clone)]          
pub struct  JUMPIF {pub n: i32}

#[derive(Debug, PartialEq, Clone)]          
pub enum MemInst {MOV, DEL}       // Memory Instruction CALLED IN MAINSEC

#[derive(Debug, PartialEq, Clone)]          
pub enum CompOp {GREATER, LESS, EQUAL, UNEQUAL}       // Comparing (< > == !=)
#[derive(Debug, PartialEq, Clone)]          
pub enum Logical_Op {AND, OR}       // Logical Operators (&& ||)

#[derive(Debug, PartialEq, Clone)]          
pub enum ExpType {MATH_EXP, STDFN_EXP, MEM_INST_EXP, CONDITION, LOGIC_EXP}       // Types of expression


//--------------------------------------------------------------------------------\\

#[derive(Debug, PartialEq, Clone)]
pub enum Builtins {
    D_type(D_type),
    Operation(Operation),
    Std_fns(Std_fns),
    Section(Section),
    MemType(MemType),
    MemInst(MemInst),
    ID(String),
    REGISTER(String),
    Comment,
    CMP(CompOp),
    Logic(Logical_Op),
    Expr{
        exp_type: ExpType,
        expr: Vec<Builtins>
    },

    JUMPIF {
        n: i32,
        expr: Vec<Builtins>
    },
}

impl PartialOrd for Builtins {    
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Builtins::D_type(D_type::int(a)), Builtins::D_type(D_type::int(b))) => a.partial_cmp(b),
            (Builtins::D_type(D_type::float(a)), Builtins::D_type(D_type::float(b))) => a.partial_cmp(b),
            (Builtins::D_type(D_type::str(a)), Builtins::D_type(D_type::str(b))) => a.partial_cmp(b),
            (Builtins::D_type(D_type::bool(a)), Builtins::D_type(D_type::bool(b))) => a.partial_cmp(b),
            

            ( Builtins::D_type(D_type::int(a)) , Builtins::D_type(D_type::float(b))) => (*a as f32).partial_cmp( b ),
            ( Builtins::D_type(D_type::float(a)) , Builtins::D_type(D_type::int(b))) => a.partial_cmp( &(*b as f32) ),

            _ => None, // Return None if types are different and cannot be compared
        }
    }
}

impl Builtins {

    pub fn unwrap_expr_vec(&self) -> Result<&Vec<Builtins>, String>{
        match self {
            Builtins::Expr { exp_type: _, expr } => {
                return Ok(expr);
            },
            other => Err(format!("Not an expression! -> {:?}", other))
        }
    }

    pub fn get_expression_type(&self) -> Result<&ExpType, String> {
        match self {
            Builtins::Expr { exp_type, expr: _ } => {
                return Ok(exp_type);
            },
            other => Err(format!("Not an expression! -> {:?}", other))
        }
    }

    pub fn builtin_hash() -> HashMap<String, Builtins> {
        HashMap::from([
        ( "ADD".to_string(), Builtins::Operation(Operation::ADD) ),     //Operation
        ( "SUB".to_string(), Builtins::Operation(Operation::SUB) ), 
        ( "MUL".to_string(), Builtins::Operation(Operation::MUL) ), 
        ( "DIV".to_string(), Builtins::Operation(Operation::DIV) ), 

        ( "PRNT".to_string(), Builtins::Std_fns(Std_fns::PRNT_PLAIN) ),       // Std fns
        ( "PRNT_COOL".to_string(), Builtins::Std_fns(Std_fns::PRNT_COOL) ), 
        ( "SINPUT".to_string(), Builtins::Std_fns(Std_fns::SINPUT) ), 
                       
        ( "_VARS:".to_string(), Builtins::Section(Section::VARS) ),     // Section
        ( "_END:".to_string(), Builtins::Section(Section::END) ),
        ( "_MAIN:".to_string(), Builtins::Section(Section::MAIN) ),
        ( "EOS!".to_string(), Builtins::Section(Section::EOS) ),

        ("int".to_string(), Builtins::MemType(MemType::int)),           // MemType
        ("float".to_string(), Builtins::MemType(MemType::float)),
        ("str".to_string(), Builtins::MemType(MemType::str)),
        ("bool".to_string(), Builtins::MemType(MemType::bool)),

        ("MOV".to_string(), Builtins::MemInst(MemInst::MOV)),           // MemInst
        ("DEL".to_string(), Builtins::MemInst(MemInst::DEL)),

        ("==".to_string(), Builtins::CMP(CompOp::EQUAL)),
        ("!=".to_string(), Builtins::CMP(CompOp::UNEQUAL)),
        (">".to_string(), Builtins::CMP(CompOp::GREATER)),
        ("<".to_string(), Builtins::CMP(CompOp::LESS)),

        ( "&&".to_string(), Builtins::Logic(Logical_Op::AND) ),
        ( "||".to_string(), Builtins::Logic(Logical_Op::OR) ),

        ("crap:-".to_string(), Builtins::Comment)       // Comment
        ])
    }

}
