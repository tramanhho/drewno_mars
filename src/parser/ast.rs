use crate::scanner::tokens::TokenType;

#[derive(Clone, Debug, PartialEq)]
pub enum ASTloc {
    // Terminal,
    Prod1(Box<ASTid>),
    Prod2(Box<ASTloc>, TokenType, Box<ASTid>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ASTid {
    TokenType,
}

// #[derive(Clone, Debug, PartialEq)]
// pub enum Terminal {
//     POSTDEC,
//     ID,
// }