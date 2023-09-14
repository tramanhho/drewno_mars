use crate::scanner::tokens::TokenType;

//AST program
#[derive(Clone, Debug, PartialEq)]
pub enum Program {
    Program(Box<Globals>),
}


//AST globals
#[derive(Clone, Debug, PartialEq)]
pub enum Globals {
    Globals_Decl(Box<Globals>, Box<Decl>),
    Globals_Epsilon(),
}


//AST decl
#[derive(Clone, Debug, PartialEq)]
pub enum Decl {
    Decl_VarDecl(Box<VarDecl>, TokenType),
    Decl_ClassDecl(Box<ClassDecl>),
    Decl_FnDecl(Box<FnDecl>),
}

//AST varDecl
#[derive(Clone, Debug, PartialEq)]
pub enum VarDecl {
    VarDecl_Colon(Box<Id>, TokenType, Box<Type>),
    VarDecl_ColonAssign(Box<Id>, TokenType, Box<Type>, TokenType, Box<Exp>),
}


//AST type
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Type_PrimType(Box<PrimType>),
    Type_PerfectPrimType(TokenType, Box<PrimType>),
    Type_Id(Box<Id>),
    Type_PerfectId(TokenType, Box<Id>),
}

//AST primType
#[derive(Clone, Debug, PartialEq)]
pub enum PrimType {
    PrimType_Int(TokenType),
    PrimType_Bool(TokenType),
    PrimType_Void(TokenType),
}

//AST classDecl
#[derive(Clone, Debug, PartialEq)]
pub enum ClassDecl {
    ClassDecl(Box<Id>, TokenType, TokenType, TokenType, Box<ClassBody>, TokenType, TokenType),
}

//AST classBody
#[derive(Clone, Debug, PartialEq)]
pub enum ClassBody {
    ClassBody_VarDecl(Box<ClassBody>, Box<VarDecl>, TokenType),
    ClassBody_FnDecl(Box<ClassBody>, Box<FnDecl>, TokenType),
    ClassBody_Epsilon(),
}

//AST fnDecl
#[derive(Clone, Debug, PartialEq)]
pub enum FnDecl {
    FnDecl_Formals(Box<Id>, TokenType, TokenType, Box<Formals>, TokenType, Box<Type>, TokenType, Box<StmtList>, TokenType),
    //fnDecl(Box<Id>, TokenType, TokenType, TokenType, Box<Type>, TokenType, Box<StmtList>, TokenType),
}

//AST formals
#[derive(Clone, Debug, PartialEq)]
pub enum Formals {
    Formals_FormalsList(Box<FormalsList>),
    Formals_Epsilon(),
}

//AST formalsList
#[derive(Clone, Debug, PartialEq)]
pub enum FormalsList {
    FormalsList_FormalDecl(Box<FormalDecl>),
    FormalsList(Box<FormalsList>, TokenType, Box<FormalDecl>),
}

//AST formalDecl 
#[derive(Clone, Debug, PartialEq)]
pub enum FormalDecl {
    FormalDecl(Box<Id>, TokenType, Box<Type>),
}

//AST stmtList
#[derive(Clone, Debug, PartialEq)]
pub enum StmtList {
    StmtList_StmtList(Box<StmtList>, Box<Stmt>, TokenType),
    StmtList_BlockStmt(Box<StmtList>, Box<BlockStmt>),
    StmtList_Epsilon(),
}


//AST blockStmt
#[derive(Clone, Debug, PartialEq)]
pub enum BlockStmt {
    BlockStmt_While(TokenType, TokenType, Box<Exp>, TokenType, TokenType, Box<StmtList>, TokenType),
    BlockStmt_If(TokenType, TokenType, Box<Exp>, TokenType, TokenType, Box<StmtList>, TokenType),
    BlockStmt_IfElse(TokenType, TokenType, Box<Exp>, TokenType, TokenType, Box<StmtList>, TokenType, TokenType, TokenType, Box<StmtList>, TokenType),
}
//AST stmt
#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Stmt_VarDecl(Box<VarDecl>),
    Stmt_Assign(Box<Loc>, TokenType, Box<Exp>),
    Stmt_PostDec(Box<Loc>, TokenType),
    Stmt_PostInc(Box<Loc>, TokenType),
    Stmt_Give(TokenType, Box<Exp>),
    Stmt_Take(TokenType, Box<Loc>),
    Stmt_ReturnExp(TokenType, Box<Exp>),
    Stmt_Return(TokenType),
    Stmt_Exit(TokenType),
    Stmt_CallExp(Box<CallExp>),
}

//AST exp
#[derive(Clone, Debug, PartialEq)]
pub enum Exp {
    Exp_Dash(Box<Term>, TokenType, Box<Exp>),
    Exp_Cross(Box<Term>, TokenType, Box<Exp>),
    Exp_Star(Box<Term>, TokenType, Box<Exp>),
    Exp_Slash(Box<Term>, TokenType, Box<Exp>),
    Exp_And(Box<Term>, TokenType, Box<Exp>),
    Exp_Or(Box<Term>, TokenType, Box<Exp>),
    Exp_Equals(Box<Term>, TokenType, Box<Exp>),
    Exp_NotEquals(Box<Term>, TokenType, Box<Exp>),
    Exp_Greater(Box<Term>, TokenType, Box<Exp>),
    Exp_GreaterEq(Box<Term>, TokenType, Box<Exp>),
    Exp_Less(Box<Term>, TokenType, Box<Exp>),
    Exp_LessEq(Box<Term>, TokenType, Box<Exp>),
    Exp_Not(TokenType, Box<Exp>),
    Exp_DashTerm(TokenType, Box<Term>),
    Exp_Term(Box<Term>),
}

//AST callExp
#[derive(Clone, Debug, PartialEq)]
pub enum CallExp {
    CallExp_Fn(Box<Id>, TokenType, TokenType),
    CallExp_FnArgs(Box<Id>, TokenType, Box<ActualsList>, TokenType),
}

//AST actualsList
#[derive(Clone, Debug, PartialEq)]
pub enum ActualsList {
    ActualsList_Exp(Box<Exp>),
    ActualsList_ActualsList(Box<ActualsList>, TokenType, Box<Exp>),
}

//AST term
#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Term_Loc(Box<Loc>),
    Term_IntLiteral(TokenType),
    Term_StringLiteral(TokenType),
    Term_True(TokenType),
    Term_False(TokenType),
    Term_Magic(TokenType),
    Term_Paren(TokenType, Box<Exp>, TokenType),
    Term_CallExp(Box<CallExp>),
}

//AST loc
#[derive(Clone, Debug, PartialEq)]
pub enum Loc {
    Loc_Id(Box<Id>),
    Loc_PostDec(Box<Loc>, TokenType, Box<Id>),
}


//AST id
#[derive(Clone, Debug, PartialEq)]
pub enum Id {
    Id(TokenType),
}
