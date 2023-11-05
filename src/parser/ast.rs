pub mod display;
pub mod span;
use span::Span;

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

#[derive(Debug, Clone)]
pub struct Type {
    pub perfect: bool,
    pub kind: Box<TypeKind>
}

impl Type {
    pub fn new(kind: TypeKind, perfect: bool) -> Box<Type> {
        Box::new(Type {
            perfect: perfect,
            kind: Box::new(kind)
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
    Class(Box<Id>),
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
pub struct LineStmt {
    pub kind: Box<LineStmtKind>,
    pub span: Span,
}

impl LineStmt {
    pub fn new(kind: LineStmtKind, l: usize, r: usize) -> Box<LineStmt> {
        Box::new(LineStmt { 
            kind: Box::new(kind), 
            span: Span::new(l, r)
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LineStmtKind {
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
pub struct Exp {
    pub expr_type: Option<Box<Type>>,
    pub kind: Box<ExpKind>,
    pub span: Span
}

impl Exp {
    pub fn new(kind: Box<ExpKind>, left_span: usize, right_span: usize) -> Box<Exp> {
        Box::new(Exp {
            expr_type: None,
            kind,
            span: Span::new(left_span, right_span)
        })
    }
}

impl Exp {
    pub fn new_with_type(kind: Box<ExpKind>, expr_type: PrimType, left_span: usize, right_span: usize) -> Box<Exp> {
        Box::new(Exp {
            expr_type: Some(Box::new(Type{ 
                perfect: false, 
                kind: Box::new(TypeKind::Prim(expr_type))
            })),
            kind,
            span: Span::new(left_span, right_span)
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpKind {
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

impl ExpKind {
    pub fn new(kind: ExpKind) -> Box<ExpKind> {
        Box::new(kind)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExp {
    pub exp: Box<Exp>,
    pub kind: Box<UnaryExpKind>,
    pub expr_type: Option<Box<Type>>,
    pub span: Span
}

impl UnaryExp {
    pub fn new(exp: Box<Exp>, kind: UnaryExpKind, left_span: usize, right_span: usize) -> Box<UnaryExp> {
        Box::new(UnaryExp {
            exp: exp,
            kind: Box::new(kind),
            expr_type: None,
            span: Span::new(left_span, right_span)
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
    pub lhs: Box<Exp>, 
    pub rhs: Box<Exp>,
    pub kind: Box<BinaryExpKind>,
    pub expr_type: Option<Box<Type>>,
    pub span: Span,
}

impl BinaryExp {
    pub fn new(lhs: Box<Exp>, rhs: Box<Exp>, kind: BinaryExpKind, left_span: usize, right_span: usize) -> Box<BinaryExp> {
        Box::new(BinaryExp {
            lhs: lhs,
            rhs: rhs,
            kind: Box::new(kind),
            expr_type: None,
            span: Span::new(left_span, right_span)
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
    pub name: Box<Loc>, 
    pub args: Vec<Box<Exp>>,
    pub fn_type: Option<Type>,
    pub span: Span
}

#[derive(Debug, Clone, PartialEq)]
pub struct Loc {
    pub span: Span,
    pub loc_type: Option<Type>,
    pub kind: Box<LocKind>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LocKind {
    Id(Box<Id>),
    Loc { base_class: Box<Loc>, field_name: Box<Id> }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Id {
    pub name: String,
    pub span: Span,
    pub id_type: Option<Type>,
}