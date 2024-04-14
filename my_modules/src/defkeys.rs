use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)] 
pub enum D_type {
    int(i32),
    str(String),
    float(f32),
    bool(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Std_fns {PRNT, INCR, DECR}     // Standard - Builtin functions

#[derive(Debug, PartialEq, Clone)]
pub enum Operation { ADD, SUB, MUL, DIV}       // Arithmetic operations

#[derive(Debug, PartialEq, Clone)]
pub enum Section {VARS, END, MAIN, EOS}        // Sections keywords

#[derive(Debug, PartialEq, Clone)]          
pub enum MemType {int, float, str, bool}    // Memory Types

#[derive(Debug, PartialEq, Clone)]          
pub enum ID{ id(String) }               // For Variables or other data_types

#[derive(Debug, PartialEq, Clone)]          
pub enum ControlFlow {JUMP}

#[derive(Debug, PartialEq, Clone)]          
pub enum MemInst {MOV, DEL, INSERT}       // Memory Instruction CALLED IN MAINSEC
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
    ControlFlow(ControlFlow),
    Comment
}

impl Builtins {
    pub fn builtin_hash() -> HashMap<String, Builtins> {
        HashMap::from([
                ( "ADD".to_string(), Builtins::Operation(Operation::ADD) ),     //Operation
                ( "SUB".to_string(), Builtins::Operation(Operation::SUB) ), 
                ( "MUL".to_string(), Builtins::Operation(Operation::MUL) ), 
                ( "DIV".to_string(), Builtins::Operation(Operation::DIV) ), 

                ( "PRNT".to_string(), Builtins::Std_fns(Std_fns::PRNT) ),       // Std fns

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
                ("INSERT".to_string(), Builtins::MemInst(MemInst::INSERT)),

                ("JUMP".to_string(), Builtins::ControlFlow(ControlFlow::JUMP)),     // ControlFlow

                ("crap->".to_string(), Builtins::Comment)       // Comment

        ])
    }

}
