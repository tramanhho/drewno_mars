#[derive(Clone, Debug, PartialEq)]
pub enum ASTloc {
    Terminal,
    // Prod1(Terminal),
    // Prod2(Box<ASTloc>, Terminal, Box<ASTid>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ASTid {
    Terminal,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Terminal {
    POSTDEC,
    ID,
}