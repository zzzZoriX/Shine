use crate::error::Error;

pub fn check_syntax(tokens: &Vec<Token>) -> Option<Error> {
    for tok in tokens {
        if let Lexeme::LexUndef = tok.get_type() {
            let word: Option<&String> = tok.get_value();
            match word {
                None => return Some(Error::SyntaxError(String::from("Unknown word: *word not entried*"))),
                Some(v) => return Some(Error::SyntaxError(format!("Unknown word: {}", v)))
            }
        }
    }

    return None;
}