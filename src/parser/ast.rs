// //AST program
// #[derive(Clone, Debug, PartialEq)]
// pub enum Program {
//     Program(Box<Globals>),
// }


// //AST globals
// #[derive(Clone, Debug, PartialEq)]
// pub enum Globals {
//     Globals_Decl(Box<Globals>, Box<Decl>),
//     Globals_Epsilon(),
// }


// //AST decl
// #[derive(Clone, Debug, PartialEq)]
// pub enum Decl {
//     Decl_VarDecl(Box<VarDecl>, TokenType),
//     Decl_ClassDecl(Box<ClassDecl>),
//     Decl_FnDecl(Box<FnDecl>),
// }


#[derive(Clone, Debug, PartialEq)]
pub enum VarDecl {
    Variable { var_type: Box<Type>, id: Box<Id> }
    // Variable { var_type: Type, id: Id, init_val: Option<Box<Exp>> }
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

// //AST classDecl
// #[derive(Clone, Debug, PartialEq)]
// pub enum ClassDecl {
//     ClassDecl(Box<Id>, TokenType, TokenType, TokenType, Box<ClassBody>, TokenType, TokenType),
// }

// //AST classBody
// #[derive(Clone, Debug, PartialEq)]
// pub enum ClassBody {
//     ClassBody_VarDecl(Box<ClassBody>, Box<VarDecl>, TokenType),
//     ClassBody_FnDecl(Box<ClassBody>, Box<FnDecl>, TokenType),
//     ClassBody_Epsilon(),
// }

// //AST fnDecl
// #[derive(Clone, Debug, PartialEq)]
// pub enum FnDecl {
//     FnDecl_Formals(Box<Id>, TokenType, TokenType, Box<Formals>, TokenType, Box<Type>, TokenType, Box<StmtList>, TokenType),
//     //fnDecl(Box<Id>, TokenType, TokenType, TokenType, Box<Type>, TokenType, Box<StmtList>, TokenType),
// }

// //AST formals
// #[derive(Clone, Debug, PartialEq)]
// pub enum Formals {
//     Formals_FormalsList(Box<FormalsList>),
//     Formals_Epsilon(),
// }

// //AST formalsList
// #[derive(Clone, Debug, PartialEq)]
// pub enum FormalsList {
//     FormalsList_FormalDecl(Box<FormalDecl>),
//     FormalsList(Box<FormalsList>, TokenType, Box<FormalDecl>),
// }


#[derive(Clone, Debug, PartialEq)]
pub enum FormalDecl {
    FormalDecl(String, Box<Type>),
}

// //AST stmtList
// #[derive(Clone, Debug, PartialEq)]
// pub enum StmtList {
//     StmtList_StmtList(Box<StmtList>, Box<Stmt>, TokenType),
//     StmtList_BlockStmt(Box<StmtList>, Box<BlockStmt>),
//     StmtList_Epsilon(),
// }


// //AST blockStmt
// #[derive(Clone, Debug, PartialEq)]
// pub enum BlockStmt {
//     BlockStmt_While(TokenType, TokenType, Box<Exp>, TokenType, TokenType, Box<StmtList>, TokenType),
//     BlockStmt_If(TokenType, TokenType, Box<Exp>, TokenType, TokenType, Box<StmtList>, TokenType),
//     BlockStmt_IfElse(TokenType, TokenType, Box<Exp>, TokenType, TokenType, Box<StmtList>, TokenType, TokenType, TokenType, Box<StmtList>, TokenType),
// }
// //AST stmt
// #[derive(Clone, Debug, PartialEq)]
// pub enum Stmt {
//     Stmt_VarDecl(Box<VarDecl>),
//     Stmt_Assign(Box<Loc>, TokenType, Box<Exp>),
//     Stmt_PostDec(Box<Loc>, TokenType),
//     Stmt_PostInc(Box<Loc>, TokenType),
//     Stmt_Give(TokenType, Box<Exp>),
//     Stmt_Take(TokenType, Box<Loc>),
//     Stmt_ReturnExp(TokenType, Box<Exp>),
//     Stmt_Return(TokenType),
//     Stmt_Exit(TokenType),
//     Stmt_CallExp(Box<CallExp>),
// }

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