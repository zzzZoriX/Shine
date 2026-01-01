pub mod error {
    pub enum Error {
        CommandLineError(String),
        LexerError(String, u16, u16),
        ParserError(String),
        SyntaxError(String),
        LogicError(String),
        CodeGeneratorError(String),
        StdIOError(String)
    }

//  NoReturn function
    pub fn process_error(err: Error) -> ! {
        match err {
            Error::CommandLineError(msg) => print!("{}", msg),
            Error::LexerError(msg, row, col) => print!("{}.{}: {}", row, col, msg),
            Error::ParserError(msg) => print!("{}", msg),
            Error::SyntaxError(msg) => print!("{}", msg),
            Error::LogicError(msg) => print!("{}", msg),
            Error::CodeGeneratorError(msg) => print!("{}", msg),
            Error::StdIOError(msg) => print!("{}", msg)
        }

        std::process::exit(1);
    }
}