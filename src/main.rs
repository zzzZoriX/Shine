include!("./moduls/command/command_line.rs");
include!("./moduls/command/command_processing.rs");
include!("./moduls/lexer/lexemes.rs");
include!("./moduls/lexer/lexer.rs");

use std::env;
use std::fs::File;
use crate::command_line::{CmdType, Command, Input};

fn main() -> Result<(), std::io::Error> {
    let mut args: Vec<String> = env::args()
                                .skip(1) // skip call command
                                .collect();

    if args.len() < 1 {
        panic!("Too few arguments...\nExample of correct use: shine compile -i <input_file> -o <output_use>");
    }

    let mut command_line: Command = Command::new(); 

    command_line.parse(&args).unwrap_or_else(|err|{
        args.clear();
        
        panic!("{}", err);
    });

    if let CmdType::CreateNewEnv = command_line.get_type() {
        process_new_cmd(&command_line.get_input().args[0])?;
        return Result::Ok(());
    }
    else if let CmdType::ShowVersion = command_line.get_type() {
        process_ver_cmd();
        return Result::Ok(());
    }

    let input_files: Input = command_line.get_input();
    for input_file in &input_files.args {
        let tokens: Vec<Token> = tokenize(input_file).unwrap_or_else(|err|{
            match err {
                LexerResult::ShineError(serr) => panic!("{}", serr),
                LexerResult::StdIoError(ioerr) => panic!("{}", ioerr)
            }
        });
    }

    return Result::Ok(());
}