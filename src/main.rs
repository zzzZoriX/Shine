include!("./moduls/command/command_line.rs");
include!("./moduls/command/command_processing.rs");
include!("./moduls/lexer/lexemes.rs");
include!("./moduls/lexer/lexer.rs");
include!("./moduls/error/herror.rs");
include!("./moduls/lexer/syntax_checker.rs");

use std::env;
use std::fs::File;
use crate::{command_line::{CmdType, Command, Input}, error::process_error};

fn main() {
    let args: Vec<String> = env::args()
                                .skip(1) // skip call command
                                .collect();

    if args.len() < 1 {
        error::process_error(error::Error::CommandLineError(
            String::from(
"Too few arguments...
Example of correct use: shine compile -i <input_file> -o <output_use>"
            )
        ));
    }

    let mut command_line: Command = Command::new(); 

    command_line.parse(&args).unwrap_or_else(|err|{
        error::process_error(error::Error::CommandLineError(
            err
        ));
    });

    if let CmdType::CreateNewEnv = command_line.get_type() {
        process_new_cmd(&command_line.get_output().arg);
        return;
    }
    else if let CmdType::ShowVersion = command_line.get_type() {
        process_ver_cmd();
        return;
    }

    let input_files: Input = command_line.get_input();
    for input_file in &input_files.args {
        let tokens: Vec<Token> = tokenize(input_file).unwrap_or_else(|err|{
            error::process_error(err);
        });

        match check_syntax(&tokens) {
            None => {},
            Some(e) => process_error(e)
        }
    }
}