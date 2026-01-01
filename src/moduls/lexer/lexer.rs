include!("./tokens.rs");
include!("./common.rs");

use std::io::Read;
use crate::tokens::Token;


fn tokenize(input_file_path: &String) -> Result<Vec<Token>, error::Error> {
    let mut row: u16 = 0;
    let mut column: u16 = 0;

    let mut ifp = File::open(input_file_path).unwrap_or_else(|_|{
        error::process_error(error::Error::LexerError(
            format!("Can't open input file: {}", input_file_path),
            row,
            column
        ));
    });
    let mut buffer: String = String::new();
    let mut word_buffer: String = String::new();
    let mut tokens_vec: Vec<Token> = Vec::new();
    
    File::read_to_string(&mut ifp, &mut buffer).unwrap_or_else(|_|{
        error::process_error(error::Error::LexerError(
            format!("Can't open input file: {}", input_file_path),
            row, 
            column
        ));
    });
    
    if buffer.len() < 1 {
        return Result::Ok(tokens_vec);
    }
    
    let mut chars = buffer.chars().peekable();

    row = 1;
    column = 1;

    while let Some(ch) = chars.next() {
        if ch == '\n' {
            row += 1;
            column = 1;
        }

        if is_spec_symbol(&ch) {
            if ch == ' ' {
                let lexeme: Lexeme = define_lexeme_by_word(&word_buffer);
                if let Lexeme::LexUndef = lexeme {
                    return Result::Err(error::Error::LexerError(
                        format!("Unknown word: {}", word_buffer),
                        row,
                        column
                    ));
                }

                tokens_vec.push(Token::create(
                    Some(&word_buffer),
                    lexeme
                ));

                word_buffer.clear();
            }

            else {
                let mut lexeme: Lexeme = define_lexeme_by_word(&word_buffer);
                if let Lexeme::LexUndef = lexeme {
                    return Result::Err(error::Error::LexerError(
                        format!("Unknown word: {}", word_buffer),
                        row,
                        column
                    ));
                }

                tokens_vec.push(Token::create(
                    Some(&word_buffer),
                    lexeme
                ));

                word_buffer.clear();
                word_buffer.push(ch);
                 
                if ch == ':' {
                    if let Some(&next) = chars.peek() {
                        if next == ':' {
                            chars.next();
                            word_buffer.push(next);
                        }
                    }
                }

                if ch == '-' {
                    if let Some(&next) = chars.peek() {
                        if next.is_digit(10) {
                            column += 1;
                            continue;
                        }
                    }
                }

                lexeme = define_lexeme_by_word(&word_buffer);
                if let Lexeme::LexUndef = lexeme {
                    return Result::Err(error::Error::LexerError(
                        format!("Unknown word: {}", word_buffer),
                        row,
                        column
                    ));
                }

                tokens_vec.push(Token::create(
                    Some(&word_buffer),
                    lexeme
                ));

                word_buffer.clear();
            }
        }

        else {
            word_buffer.push(ch);
        }

        column += 1;
    }
    
    return Result::Ok(tokens_vec);
}