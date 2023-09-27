use logos::{Logos, SpannedIter};
use crate::scanner::tokens::TokenType;

#[derive(Debug)]
pub enum LexicalError {
    InvalidToken,
}

pub struct Lexer<'input> {
    // instead of an iterator over characters, we have a token iterator
    token_stream: SpannedIter<'input, TokenType>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        // the Token::lexer() method is provided by the Logos trait
        Self { token_stream: TokenType::lexer(input).spanned() }
    }
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<TokenType, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(token, span)| {
            let token = token.unwrap_or_else(|_| TokenType::Illegal("".to_string()) );
            match token {
                // found an invalid token
                TokenType::Illegal(_) => Err(LexicalError::InvalidToken),
                _ => Ok((span.start, token, span.end))
            }
        })
    }
}