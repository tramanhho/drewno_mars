#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub globals: Vec<Box<Decl>>
}

#[derive(Clone, Debug, PartialEq)]
pub enum Decl {
    VarDecl(Box<VarDecl>),
    ClassDecl(Box<ClassDecl>),
    FnDecl(Box<FnDecl>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum VarDecl {
    VarDecl { var_type: Box<Type>, id: Box<Id>, init_val: Option<Box<Exp>> },
}


#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Prim(PrimType),
    Class(Box<Id>),
    PerfectPrim(PrimType),
    PerfectClass(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimType {
    Bool,
    Int,
    Void,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClassDecl {
    Class { id: Box<Id>, member_f: Box<Vec<Box<Decl>>> },
}

#[derive(Clone, Debug, PartialEq)]
pub enum FnDecl {
    FnDecl_Formals{id: Box<Id>, args: Vec<Box<FormalDecl>>, ret: Box<Type>, body: Vec<Box<Stmt>>},
}

#[derive(Clone, Debug, PartialEq)]
pub enum FormalDecl {
    VarDecl(VarDecl),
    FormalDecl(String, Box<Type>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Block(Box<BlockStmt>),
    Line(Box<LineStmt>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockStmt {
    While  {cond: Box<Exp>, body: Vec<Box<Stmt>> },
    If     {cond: Box<Exp>, body: Vec<Box<Stmt>> } ,
    IfElse {cond: Box<Exp>, true_branch: Vec<Box<Stmt>>, false_branch: Vec<Box<Stmt>> },
}

#[derive(Clone, Debug, PartialEq)]
pub enum LineStmt {
    // declarations
    Decl(Box<Decl>),

    // keyword centric
    Assign { dest: Box<Loc>, src: Box<Exp> },
    PostDec(Box<Loc>),
    PostInc(Box<Loc>),
    Give(Box<Exp>),
    Take(Box<Loc>),
    Return(Option<Box<Exp>>),
    Exit,
    Call(Box<CallExp>),
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryExp {
    Neg { exp: Box<Exp>},
    Not { exp: Box<Exp>},
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum CallExp {
    Fn {name: Box<Loc>, args: Vec<Box<Exp>>}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Loc {
    Id(Box<Id>),
    MemberFieldExp { base_class: Box<Loc>, field_name: String }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Id {
    Name(String)
}