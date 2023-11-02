pub mod display;
pub mod position;
use position::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub globals: Vec<Box<Decl>>,
    // pub position: Position
}

#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    VarDecl(Box<VarDecl>),
    ClassDecl(Box<ClassDecl>),
    FnDecl(Box<FnDecl>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub var_type: Box<Type>, 
    pub id: Box<Id>, 
    pub init_val: Option<Box<Exp>>,
    // pub position: Position
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Prim(PrimType),
    Class(Box<Id>),
    PerfectPrim(PrimType),
    PerfectClass(Box<Id>),
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PrimType {
    Bool,
    Int,
    Void,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    pub id: Box<Id>, 
    pub member_f: Box<Vec<Box<Decl>>>,
    // pub position: Position
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub id: Box<Id>, 
    pub args: Vec<Box<FormalDecl>>, 
    pub ret: Box<Type>, 
    pub body: Vec<Box<Stmt>>,
    // pub position: Position
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormalDecl {
    VarDecl(VarDecl),
    FormalDecl{
        id: Box<Id>, 
        formal_type: Box<Type>,
        // position: Position
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Block(Box<BlockStmt>),
    Line(Box<LineStmt>),
    VarDecl(Box<VarDecl>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockStmt {
    While  {cond: Box<Exp>, body: Vec<Box<Stmt>> },
    If     {cond: Box<Exp>, body: Vec<Box<Stmt>> } ,
    IfElse {cond: Box<Exp>, true_branch: Vec<Box<Stmt>>, false_branch: Vec<Box<Stmt>> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineStmt {
    Assign { dest: Box<Loc>, src: Box<Exp> },
    PostDec{ loc: Box<Loc>},
    PostInc{ loc: Box<Loc>},
    Give   { output: Box<Exp>},
    Take   { recipient: Box<Loc>},
    Return { result: Option<Box<Exp>>},
    Exit,
    Call(Box<CallExp>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Exp {
    True,
    False,
    Magic,
    UnaryExp(Box<UnaryExp>),
    BinaryExp(Box<BinaryExp>),
    CallExp(Box<CallExp>),
    IntLit(i32),
    StrLit(String),
    Loc(Box<Loc>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryExp {
    Neg { exp: Box<Exp>},
    Not { exp: Box<Exp>},
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryExp {
    And       { lhs: Box<Exp>, rhs: Box<Exp>},
    Or        { lhs: Box<Exp>, rhs: Box<Exp>},
    Equals    { lhs: Box<Exp>, rhs: Box<Exp>},
    NotEquals { lhs: Box<Exp>, rhs: Box<Exp>},
    Greater   { lhs: Box<Exp>, rhs: Box<Exp>},
    Less      { lhs: Box<Exp>, rhs: Box<Exp>},
    GreaterEq { lhs: Box<Exp>, rhs: Box<Exp>},
    LessEq    { lhs: Box<Exp>, rhs: Box<Exp>},
    Plus      { lhs: Box<Exp>, rhs: Box<Exp>},
    Minus     { lhs: Box<Exp>, rhs: Box<Exp>},
    Times     { lhs: Box<Exp>, rhs: Box<Exp>},
    Divide    { lhs: Box<Exp>, rhs: Box<Exp>},
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExp {
    pub name: Box<Loc>, 
    pub args: Vec<Box<Exp>>
}

#[derive(Debug, Clone, PartialEq)]
pub enum Loc {
    Id(Box<Id>),
    Loc { base_class: Box<Loc>, field_name: Box<Id> }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Id {
    pub name: String,
    pub position: Position
}