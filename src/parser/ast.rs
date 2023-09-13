use crate::scanner::tokens::TokenType;

//AST stmtList
#[derive(Clone, Debug, PartialEq)]
pub enum ASTstmtList {
    stmtList_stmtList(Box<ASTstmtList>, Box<ASTstmt>, TokenType),
    stmtList_blockStmt(Box<ASTstmtList>, Box<ASTblockStmt>),
    stmtList_epsilon(),
}


//AST blockStmt
#[derive(Clone, Debug, PartialEq)]
pub enum ASTblockStmt {
    blockStmt_while(TokenType, TokenType, Box<ASTexp>, TokenType, TokenType, Box<ASTstmtList>, TokenType),
    blockStmt_if(TokenType, TokenType, Box<ASTexp>, TokenType, TokenType, Box<ASTstmtList>, TokenType),
    blockStmt_ifelse(TokenType, TokenType, Box<ASTexp>, TokenType, TokenType, Box<ASTstmtList>, TokenType, TokenType, TokenType, Box<ASTstmtList>, TokenType),
}
//AST stmt
#[derive(Clone, Debug, PartialEq)]
pub enum ASTstmt {
    //stmt_varDecl(Box<ASTvarDecl>),
    stmt_assign(Box<ASTloc>, TokenType, Box<ASTexp>),
    stmt_postdec(Box<ASTloc>, TokenType),
    stmt_postinc(Box<ASTloc>, TokenType),
    stmt_give(TokenType, Box<ASTexp>),
    stmt_take(TokenType, Box<ASTloc>),
    stmt_returnexp(TokenType, Box<ASTexp>),
    stmt_return(TokenType),
    stmt_exit(TokenType),
    stmt_callExp(Box<ASTcallExp>),
}

//AST exp
#[derive(Clone, Debug, PartialEq)]
pub enum ASTexp {
    exp_dash(Box<ASTexp>, TokenType, Box<ASTterm>),
    exp_cross(Box<ASTexp>, TokenType, Box<ASTterm>),
    exp_star(Box<ASTexp>, TokenType, Box<ASTterm>),
    exp_slash(Box<ASTexp>, TokenType, Box<ASTterm>),
    exp_and(Box<ASTexp>, TokenType, Box<ASTterm>),
    exp_or(Box<ASTexp>, TokenType, Box<ASTterm>),
    exp_equals(Box<ASTexp>, TokenType, Box<ASTterm>),
    exp_notequals(Box<ASTexp>, TokenType, Box<ASTterm>),
    exp_greater(Box<ASTexp>, TokenType, Box<ASTterm>),
    exp_greatereq(Box<ASTexp>, TokenType, Box<ASTterm>),
    exp_less(Box<ASTexp>, TokenType, Box<ASTterm>),
    exp_lesseq(Box<ASTexp>, TokenType, Box<ASTterm>),
    exp_not(TokenType, Box<ASTterm>),
    exp_dashterm(TokenType, Box<ASTterm>),
    exp_term(Box<ASTterm>),
}

//AST callExp
#[derive(Clone, Debug, PartialEq)]
pub enum ASTcallExp {
    callExp_fn(Box<ASTid>, TokenType, TokenType),
    callExp_fnargs(Box<ASTid>, TokenType, Box<ASTactualsList>, TokenType),
}

//AST actualsList
#[derive(Clone, Debug, PartialEq)]
pub enum ASTactualsList {
    actualsList_exp(Box<ASTexp>),
    actualsList_actualsList(Box<ASTactualsList>, TokenType, Box<ASTexp>),
}

//AST term
#[derive(Clone, Debug, PartialEq)]
pub enum ASTterm {
    term_loc(Box<ASTloc>),
    term_intliteral(TokenType),
    term_stringliteral(TokenType),
    term_true(TokenType),
    term_false(TokenType),
    term_magic(TokenType),
    term_paren(TokenType, Box<ASTexp>, TokenType),
    term_callExp(Box<ASTcallExp>),
}

//AST loc
#[derive(Clone, Debug, PartialEq)]
pub enum ASTloc {
    loc_id(Box<ASTid>),
    loc_postdec(Box<ASTloc>, TokenType, Box<ASTid>),
}


//AST id
#[derive(Clone, Debug, PartialEq)]
pub enum ASTid {
    id(TokenType),
}
