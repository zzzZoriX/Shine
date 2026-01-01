include!("./tokens.rs");
include!("./common.rs");

use std::io::Read;
use crate::tokens::Token;


enum LexerResult {
    StdIoError(std::io::Error),
    ShineError(String)
}

fn tokenize(input_file_path: &String) -> Result<Vec<Token>, LexerResult> {
    let mut ifp = File::open(input_file_path).map_err(LexerResult::StdIoError)?;
    let mut buffer: String = String::new();
    let mut word_buffer: String = String::new();
    let mut tokens_vec: Vec<Token> = Vec::new();

    File::read_to_string(&mut ifp, &mut buffer).map_err(LexerResult::StdIoError)?;

    if buffer.len() < 1 {
        return Result::Ok(tokens_vec);
    }

    let mut chars = buffer.chars().peekable();

    while let Some(ch) = chars.next() {
        if is_spec_symbol(&ch) {
            if ch == ' ' {
                let lexeme: Lexeme = define_lexeme_by_word(&word_buffer);
                if let Lexeme::LexUndef = lexeme {
                    return Result::Err(LexerResult::ShineError(
                        format!("Unknown word: {}", word_buffer)
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
                    return Result::Err(LexerResult::ShineError(String::from(
                        format!("Unknown word: {}", word_buffer)
                    )));
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
                            continue;
                        }
                    }
                }

                lexeme = define_lexeme_by_word(&word_buffer);
                if let Lexeme::LexUndef = lexeme {
                    return Result::Err(LexerResult::ShineError(String::from(
                        format!("Unknown word: {}", word_buffer)
                    )));
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
    }
    
    return Result::Ok(tokens_vec);
}