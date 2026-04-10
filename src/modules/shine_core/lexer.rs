use crate::modules::shine_core::{token::*, lexemes::*};

pub struct Lexer {
    pub tokens_list: TokensList,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens_list: TokensList::new()
        }
    }

    pub fn parse(&mut self) {

    }
}