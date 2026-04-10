use crate::modules::shine_core::lexemes::*;

pub struct Token<'a> {
    pub value: String,
    pub lexeme: Lexeme,
    pub next: Option<&'a mut Box<Token<'a>>>,
}

pub struct TokensList<'a> {
    pub head: Option<&'a mut Box<Token<'a>>>,
    pub tail: Option<&'a mut Box<Token<'a>>>,
    pub size: usize
}


impl<'a> Token<'a> {
    pub fn new(word: &String, next_token: &Option<Box<Token>>) -> Token<'a> {
        Token {
            value: word.clone(),
            lexeme: define_lexeme(word),
            next: Some(&mut next_token.unwrap())
        }
    }
}

impl<'a> TokensList<'a> {
    pub fn add(&mut self, word: &String) {
        let mut new_token = Box::new(Token::new(
            word, &None
        ));

        if self.head.is_none() {
            self.head = Some(&mut new_token);
        }

        self.tail.unwrap().next = Some(&mut new_token);
        self.tail = Some(&mut new_token);

        self.size += 1;
    }

    pub fn take(&self) -> &Option<&'a mut Box<Token<'a>>> {
        &self.tail
    }

    pub fn take_mut(&mut self) -> &mut Option<&'a mut Box<Token<'a>>> {
        &mut self.tail
    }
}