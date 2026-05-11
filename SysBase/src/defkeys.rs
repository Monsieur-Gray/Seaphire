use std::collections::HashMap ;


#[derive(Debug, PartialEq, Clone)] 
pub enum D_type {
    int(i32),
    str(String),
    float(f32),
    bool(bool),
}

// Just a wrapper around int and float types
#[derive(Debug, Clone)] 
pub enum Number {
    int(i32),
    float(f32)
}


#[derive(Debug, PartialEq, Clone)]
pub enum Std_fns {PRINT_COOL, PRINT, PRINT_NEWLINE, SINPUT}     // Standard - Builtin functions

#[derive(Debug, PartialEq, Clone)]
pub enum Operation { ADD, SUB, MUL, DIV}       // Arithmetic operations

#[derive(Debug, PartialEq, Clone)]          
pub enum MemType {int, float, str, bool, NaN}    // Memory Types

#[derive(Debug, PartialEq, Clone)]          
pub enum ID{ id(String) }               // For Variables or other data_types

#[derive(Debug, PartialEq, Clone)]          
pub enum MemInst {MOV, DEL}       // Memory Instruction CALLED IN MAINSEC

#[derive(Debug, PartialEq, Clone)]          
pub enum CompOp {GREATER, LESS, EQUAL, UNEQUAL, GREATER_EQ, LESS_EQ}       // Comparing (< > == !=)

#[derive(Debug, PartialEq, Clone)]          
pub enum Logical_Op {AND, OR}       // Logical Operators (&& ||)

#[derive(Debug, PartialEq, Clone)]          
pub enum Loop {WHILE_LOOP(WHILE_LOOP) , FOR_LOOP}


#[derive(Debug, PartialEq, Clone)]          
pub struct WHILE_LOOP {
    pub condition: Vec<Builtins>,
    pub block: InnerScope
}


#[derive(Debug, PartialEq, Clone)]          
pub enum ExpType {
    MATH_EXP, STDFN_EXP, MEM_INST_EXP, 
    CONDITION, LOGIC_EXP, 

    ELSE_EXP, ELIF_EXP,
    IF_EXP, IF_ELSE_EXP, IF_ELIF_EXP,

    WHILE_EXP,

    LOCAL_VAR_MAKE
}       // Types of expression


#[derive(Debug, PartialEq, Clone)]          
pub enum Scope{ 
    GlobalScope,
    Local(u32)
}

impl Scope {
    pub fn scope_to_u32(&self) -> u32 {
        match self {
            Self::GlobalScope => 0,
            Self::Local(l) => *l
        }
    }
}

#[derive(Debug, PartialEq, Clone)]          
pub struct VarDecl {
    pub name: String,
    pub data: Builtins,
    pub scope: Scope,
    pub is_mutable: bool, 
}


#[derive(Debug, PartialEq, Clone)]          
pub struct InnerScope { 
    pub inner_vsec: Option< Vec<Builtins> >,        // Not necessarily it will have some variables
    pub block: Vec<Builtins>,       // block is a collection of expressions (each will be of type Builtins::Expr)
    pub scope: Scope
}

/* 
#[derive(Debug, PartialEq, Clone)]          
pub struct Expr {
    pub exp_type: ExpType,
    pub expr: Vec<Builtins>

} */

//--------------------------------------------------------------------------------\\
//--------------------------------------------------------------------------------\\

#[derive(Debug, PartialEq, Clone)]
pub enum Builtins {
    InnerScope(InnerScope),

    D_type(D_type),
    Operation(Operation),
    Std_fns(Std_fns),
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

    Loop(Loop)

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

/* impl Add for D_type {
    type Output = Number;
    fn add(self, rhs: Self) -> Number {
        match (self, rhs) {
            (D_type::int(i) , D_type::float(f)) => Number::float(i as f32 + f),
            (D_type::float(f) , D_type::int(i)) => Number::float(i as f32 + f),
            (D_type::int(i1) , D_type::int(i2)) => Number::int(i1 + i2),
            (D_type::float(f1) , D_type::float(f2)) => Number::float(f1 + f2),
            _ => Throw!("Unsupported data types! Cannot perform addition.")
        }
    }
}

impl Sum for Builtins::D_type {
    fn sum() -> Number {

    }
}
 */
impl Builtins {

    pub fn unwrap_dtype_int(&self) -> i32 {
        match &self {
            Builtins::D_type(D_type::int(i)) => return *i,
            other => crate::Throw!(format!("Can't unwrap this type to an INT ---> {:?}", other)),
        }
    }
    pub fn unwrap_dtype_float(&self) -> f32 {
        match &self {
            Builtins::D_type(D_type::float(f)) => return *f,
            other => crate::Throw!(format!("Can't unwrap this type to an FLOAT ---> {:?}", other)),
        }
    }
    pub fn unwrap_dtype_bool(&self) -> bool {
        match &self {
            Builtins::D_type(D_type::bool(b)) => return *b,
            other => crate::Throw!(format!("Can't unwrap this type to an BOOL ---> {:?}", other)),
        }
    }
    pub fn unwrap_dtype_str(&self) -> String {
        match &self {
            Builtins::D_type(D_type::str(s)) => return s.replace("'", ""),
            other => crate::Throw!(format!("Can't unwrap this type to an STR ---> {:?}", other)),
        }
    }

    pub fn to_data(&self, scope: Scope, is_mutable: bool) -> crate::memory_layout::MemCell {
        match self {
            Builtins::D_type(_) => crate::memory_layout::MemCell{ data: self.clone(), scope , is_mutable},
            _ => crate::Throw!( format!("The following type -> '{:?}' cannot be interpreted as a MemCellType", &self))
        }
    }

    pub fn unwrap_expr_vec(&self) -> Result< &Vec<Builtins>, String >{
        match self {
            Builtins::Expr { exp_type: _, expr } => {
                return Ok(expr);
            },

            Builtins::InnerScope (InnerScope{ inner_vsec: _, block, scope: _ }) => {
                return Ok(block);
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

    pub fn get_data_type(&self) -> MemType {
        match self {
            Builtins::D_type( D_type::int(_) ) => MemType::int,
            Builtins::D_type( D_type::float(_) ) => MemType::float,
            Builtins::D_type( D_type::str(_) ) => MemType::str,
            Builtins::D_type( D_type::bool(_) ) => MemType::bool,
            _ => MemType::NaN
        }
    }

    pub fn builtin_hash() -> HashMap<String, Builtins> {
        HashMap::from([
        ( "ADD".to_string(), Builtins::Operation(Operation::ADD) ),     //Operation
        ( "SUB".to_string(), Builtins::Operation(Operation::SUB) ), 
        ( "MUL".to_string(), Builtins::Operation(Operation::MUL) ), 
        ( "DIV".to_string(), Builtins::Operation(Operation::DIV) ), 

        ( "PRINT".to_string(), Builtins::Std_fns(Std_fns::PRINT) ),       // Std fns
        ( "PRINTLN".to_string(), Builtins::Std_fns(Std_fns::PRINT_NEWLINE)),
        ( "PRINT_COOL".to_string(), Builtins::Std_fns(Std_fns::PRINT_COOL) ), 
        
        ( "SINPUT".to_string(), Builtins::Std_fns(Std_fns::SINPUT) ), 
                       
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
        (">=".to_string(), Builtins::CMP(CompOp::GREATER_EQ)),
        ("<=".to_string(), Builtins::CMP(CompOp::LESS_EQ)),

        ( "&&".to_string(), Builtins::Logic(Logical_Op::AND) ),
        ( "||".to_string(), Builtins::Logic(Logical_Op::OR) ),

        ("crap:-".to_string(), Builtins::Comment)       // Comment
        ])
    }

}
