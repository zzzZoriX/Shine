include!("./moduls/common/messages.rs");
include!("./moduls/command/command_line.rs");

use std::env;
use std::fs::File;
use crate::command_line::Command;

fn main() {
    let mut args: Vec<String> = env::args()
                                .skip(1) // skip call command
                                .collect();

    if args.len() < 1 {
        panic!("{}", TOO_FEW_ARGS);
    }

    let mut command_line: Command = Command::new(); 

    command_line.parse(&args).unwrap_or_else(|err|{
        args.clear();
        
        panic!("{}", err);
    });

    
}