use crate::modules::shine_core::lexemes::*;

pub struct Token {
    pub value: String,
    pub lexeme: &'static Lexeme,
    pub next: Option<Box<Token>>,
}

pub struct TokensList {
    pub head: Option<Box<Token>>,
    pub size: usize
}


impl Token {
    pub fn new(word: &String, next_token: Option<Box<Token>>) -> Token {
        Token {
            value: word.clone(),
            lexeme: define_lexeme(word),
            next: next_token
        }
    }
}

impl TokensList {
    pub fn add(&mut self, word: &String) {
        let new_token = Box::new(Token::new(
            word, None
        ));

        match self.head.as_mut() {
            None => self.head = Some(new_token),
            Some(mut current) => {
                while let Some(ref mut next) = current.next {
                    current = next;
                }

                current.next = Some(new_token);
            }
        }

        self.size += 1;
    }

    pub fn take(&mut self) -> Option<&mut Box<Token>> {
        match self.head.as_mut() {
            None => None,
            Some(mut current) => {
                while let Some(ref mut next) = current.next {
                    current = next;
                }

                Some(current)
            }
        }
    }
}