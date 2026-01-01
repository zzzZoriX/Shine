mod tokens {
    use crate::Lexeme as TokenLexeme;

    pub struct Token {
        value: Option<String>,
        tok_type: TokenLexeme
    }

    impl Token {
        pub fn new() -> Self {
            return Self {
                value: None,
                tok_type: TokenLexeme::LexUndef
            };
        }

        pub fn create(value: Option<&String>, tok_type: TokenLexeme) -> Self {
            match value {
                Some(v) => return Self {
                    value: Some(v.clone()), tok_type
                },
                None => return Self::new()
            }
        }
        
        pub fn get_value(&self) -> Option<&String> {
            match &self.value {
                Some(value) => return Some(value),
                None => return None
            }
        }

        pub fn get_type(&self) -> &TokenLexeme {
            return &self.tok_type;
        }
    }
}