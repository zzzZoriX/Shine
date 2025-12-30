mod tokens {
    use crate::Lexeme;

    pub struct Token<D> {
        value: Option<D>,
        tok_type: Lexeme
    }

    impl<D> Token<D> {
        pub fn new() -> Self {
            return Self {
                value: None,
                tok_type: Lexeme::LexUndef
            };
        }

        pub fn create(value: Option<D>, tok_type: Lexeme) -> Self {
            return Self {
                value, tok_type
            };
        }
        
        pub fn get_value(&self) -> Option<&D> {
            match &self.value {
                Some(value) => return Some(value),
                None => return None
            }
        }

        pub fn get_type(&self) -> &Lexeme {
            return &self.tok_type;
        }
    }
}