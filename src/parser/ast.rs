pub mod display;
pub mod span;
use std::{rc::Rc, cell::RefCell};

use span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub globals: Vec<Rc<Decl>>,
    // pub position: Position
}

#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    VarDecl(Rc<VarDecl>),
    ClassDecl(Rc<ClassDecl>),
    FnDecl(Rc<FnDecl>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl {
    pub var_type: Rc<Type>, 
    pub id: Rc<Id>, 
    pub init_val: Option<Rc<Exp>>,
    // pub position: Position
}

#[derive(Debug, Clone)]
pub struct Type {
    pub perfect: bool,
    pub kind: Rc<TypeKind>
}

impl Type {
    pub fn new(kind: TypeKind, perfect: bool) -> Rc<Type> {
        Rc::new(Type {
            perfect: perfect,
            kind: Rc::new(kind)
        })
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        use TypeKind::*;
        let self_type = match *self.kind {
            Prim(x) => x,
            _ => PrimType::Void
        };

        let other_type = match *other.kind {
            Prim(x) => x,
            _ => PrimType::Void
        };
        self_type == other_type
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Prim(PrimType),
    Class(Rc<Id>),
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PrimType {
    Bool,
    Int,
    Void,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    pub id: Rc<Id>, 
    pub member_f: Rc<Vec<Rc<Decl>>>,
    // pub position: Position
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub id: Rc<Id>, 
    pub args: Vec<Rc<FormalDecl>>, 
    pub ret: Rc<Type>, 
    pub body: Vec<Rc<Stmt>>,
    // pub position: Position
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormalDecl {
    VarDecl(VarDecl),
    FormalDecl{
        id: Rc<Id>, 
        formal_type: Rc<Type>,
        // position: Position
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Block(Rc<BlockStmt>),
    Line(Rc<LineStmt>),
    VarDecl(Rc<VarDecl>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockStmt {
    While  {cond: Rc<Exp>, body: Vec<Rc<Stmt>> },
    If     {cond: Rc<Exp>, body: Vec<Rc<Stmt>> } ,
    IfElse {cond: Rc<Exp>, true_branch: Vec<Rc<Stmt>>, false_branch: Vec<Rc<Stmt>> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineStmt {
    pub kind: Rc<LineStmtKind>,
    pub span: RefCell<Span>,
}

impl LineStmt {
    pub fn new(kind: LineStmtKind, l: usize, r: usize) -> Rc<LineStmt> {
        Rc::new(LineStmt { 
            kind: Rc::new(kind), 
            span: RefCell::new(Span::new(l, r))
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineStmtKind {
    Assign { dest: Rc<Loc>, src: Rc<Exp> },
    PostDec{ loc: Rc<Loc>},
    PostInc{ loc: Rc<Loc>},
    Give   { output: Rc<Exp>},
    Take   { recipient: Rc<Loc>},
    Return { result: Option<Rc<Exp>>},
    Exit,
    Call(Rc<CallExp>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Exp {
    pub expr_type: Option<Rc<Type>>,
    pub kind: Rc<ExpKind>,
    pub span: RefCell<Span>
}

impl Exp {
    pub fn new(kind: Rc<ExpKind>, left_span: usize, right_span: usize) -> Rc<Exp> {
        Rc::new(Exp {
            expr_type: None,
            kind,
            span: RefCell::new(Span::new(left_span, right_span))
        })
    }
}

impl Exp {
    pub fn new_with_type(kind: Rc<ExpKind>, expr_type: PrimType, left_span: usize, right_span: usize) -> Rc<Exp> {
        Rc::new(Exp {
            expr_type: Some(Rc::new(Type{ 
                perfect: false, 
                kind: Rc::new(TypeKind::Prim(expr_type))
            })),
            kind,
            span: RefCell::new(Span::new(left_span, right_span))
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpKind {
    True,
    False,
    Magic,
    UnaryExp(Rc<UnaryExp>),
    BinaryExp(Rc<BinaryExp>),
    CallExp(Rc<CallExp>),
    IntLit(i32),
    StrLit(String),
    Loc(Rc<Loc>),
}

impl ExpKind {
    pub fn new(kind: ExpKind) -> Rc<ExpKind> {
        Rc::new(kind)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExp {
    pub exp: Rc<Exp>,
    pub kind: Rc<UnaryExpKind>,
    pub expr_type: Option<Rc<Type>>,
    pub span: RefCell<Span>
}

impl UnaryExp {
    pub fn new(exp: Rc<Exp>, kind: UnaryExpKind, left_span: usize, right_span: usize) -> Rc<UnaryExp> {
        Rc::new(UnaryExp {
            exp: exp,
            kind: Rc::new(kind),
            expr_type: None,
            span: RefCell::new(Span::new(left_span, right_span))
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryExpKind {
    Neg,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExp {
    pub lhs: Rc<Exp>, 
    pub rhs: Rc<Exp>,
    pub kind: Rc<BinaryExpKind>,
    pub expr_type: Option<Rc<Type>>,
    pub span: RefCell<Span>,
}

impl BinaryExp {
    pub fn new(lhs: Rc<Exp>, rhs: Rc<Exp>, kind: BinaryExpKind, left_span: usize, right_span: usize) -> Rc<BinaryExp> {
        Rc::new(BinaryExp {
            lhs: lhs,
            rhs: rhs,
            kind: Rc::new(kind),
            expr_type: None,
            span: RefCell::new(Span::new(left_span, right_span))
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryExpKind {
    And,
    Or,
    Equals,
    NotEquals,
    Greater,
    Less,
    GreaterEq,
    LessEq,
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExp {
    pub name: Rc<Loc>, 
    pub args: Vec<Rc<Exp>>,
    pub fn_type: Option<Rc<Type>>,
    pub span: RefCell<Span>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Loc {
    pub span: RefCell<Span>,
    pub loc_type: Option<Rc<Type>>,
    pub kind: Rc<LocKind>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LocKind {
    Id(Rc<Id>),
    Loc { base_class: Rc<Loc>, field_name: Rc<Id> }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Id {
    pub name: String,
    pub span: RefCell<Span>,
    pub id_type: RefCell<Option<Type>>,
}