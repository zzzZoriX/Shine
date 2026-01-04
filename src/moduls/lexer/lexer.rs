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
            if !word_buffer.is_empty() {
                tokens_vec.push(Token::create(
                    Some(&word_buffer),
                    define_lexeme_by_word(&word_buffer)
                ));

                word_buffer.clear();
            }

            if ch == ' ' {
                column += 1;
                continue;
            }

            match ch {
                ' ' => {
                    column += 1;
                    continue;
                }
                ':' => {
                    if let Some(&n) = chars.peek() {
                        if n == ':' {
                            chars.next();

                            tokens_vec.push(Token::create(
                                Some(&String::from("::")),
                                Lexeme::LexDblTwoDots
                            ));

                            column += 2;
                            continue;
                        }
                    }

                    tokens_vec.push(Token::create(
                        Some(&String::from(':')),
                        Lexeme::LexTwoDots
                    ));
                }
                '-' | '+' | '|' | '&' => {
                    word_buffer.push(ch);

                    if let Some(&n) = chars.peek() {
                        if n == word_buffer.chars().nth(0).unwrap() {
                            chars.next();
                            word_buffer.push(n);

                            tokens_vec.push(Token::create(
                                Some(&word_buffer),
                                define_lexeme_by_word(&word_buffer)
                            ));
    
                            column += 2;
                            continue;
                        }
                        if n == '=' {
                            chars.next();

                            word_buffer.push(n);

                            tokens_vec.push(Token::create(
                                Some(&word_buffer),
                                define_lexeme_by_word(&word_buffer)
                            ));

                            column += 2;
                            continue;
                        }
                    }

                    tokens_vec.push(Token::create(
                        Some(&word_buffer),
                        define_lexeme_by_word(&word_buffer)
                    ));
                }
                '*' | '/' | '%' | '^' => {
                    word_buffer.push(ch);

                    if let Some(&n) = chars.peek() {
                        if n == '=' {
                            chars.next();
                            word_buffer.push(n);

                            tokens_vec.push(Token::create(
                                Some(&word_buffer),
                                define_lexeme_by_word(&word_buffer)
                            ));

                            word_buffer.clear();

                            column += 2;
                            continue;
                        }
                    }

                    tokens_vec.push(Token::create(
                        Some(&word_buffer),
                        define_lexeme_by_word(&word_buffer)
                    ));

                    word_buffer.clear();
                }
                _ => {
                    word_buffer.push(ch);

                    tokens_vec.push(Token::create(
                        Some(&word_buffer),
                        define_lexeme_by_word(&word_buffer)
                    ));

                    word_buffer.clear();
                }
            }
        }
        else {
            word_buffer.push(ch);
        }

        column += 1;
    }
    
    return Result::Ok(tokens_vec);
}