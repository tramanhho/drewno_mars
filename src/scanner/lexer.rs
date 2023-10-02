use logos::{Logos, SpannedIter};
use super::tokens::{TokenType, LexingError};

pub struct Lexer<'input> {
    token_stream: SpannedIter<'input, TokenType>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self { token_stream: TokenType::lexer(input).spanned() }
    }
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<TokenType, usize, LexingError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(token, span)| {
            //let token = token.unwrap_or_else(|_| LexicalError::InvalidToken );
            match token {
                Ok(x) => Ok((span.start, x, span.end)),
                Err(x) => {
                    Err(x)
                }
            }
        })
    }
}