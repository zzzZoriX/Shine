pub mod error {
    pub enum ShineErrorType {
        CommandLineError,
        LexerError,
        ParserError,
        SyntaxError,
        LogicError,
        CodeGeneratorError,
        StdIOError
    }
    pub enum Error {
        ShineError(ShineErrorType, String),
    }

//  NoReturn function
    pub fn process_error(err: Error) -> ! {
        match err {
            Error::ShineError(etype, msg) => {
                match etype {
                    ShineErrorType::CommandLineError => {
                        eprint!("Console command error:\n{}", msg);
                        std::process::exit(1);
                    },
                    ShineErrorType::LexerError => {
                        eprint!("Lexer error:\n{}", msg);
                        std::process::exit(1);
                    },
                    ShineErrorType::ParserError => {
                        eprint!("Tokens parser error:\n{}", msg);
                        std::process::exit(1);
                    },
                    ShineErrorType::SyntaxError => {
                        eprint!("Syntax error:\n{}", msg);
                        std::process::exit(1);
                    },
                    ShineErrorType::LogicError => {
                        eprint!("Logic error:\n{}", msg);
                        std::process::exit(1);
                    },
                    ShineErrorType::CodeGeneratorError => {
                        eprint!("Error occurred while code generating:\n{}", msg);
                        std::process::exit(1);
                    },
                    ShineErrorType::StdIOError => {
                        eprint!("std::io error:\n{}", msg);
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}