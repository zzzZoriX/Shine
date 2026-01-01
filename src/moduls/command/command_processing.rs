include!("../meta.rs");

use std::fs;
use std::process;

fn process_new_cmd(dir_name: &String) -> () {
//  make a directory
    fs::create_dir(dir_name).unwrap_or_else(|_|{
        error::process_error(error::Error::ShineError(
            error::ShineErrorType::CommandLineError,
            String::from("Can't create new directory for new-command")
        ));
    });
    process::Command::new("cd").arg(dir_name).output().unwrap_or_else(|err|{
        error::process_error(error::Error::ShineError(
            error::ShineErrorType::CommandLineError,
            format!("Command writing failed with error code: {}", err)
        ));
    });
    
//  make a support files and directories
    fs::File::create_new(".gitignore").unwrap_or_else(|_|{
        error::process_error(error::Error::ShineError(
            error::ShineErrorType::CommandLineError,
            String::from("Can't create new file for new-command")
        ));
    });
    process::Command::new("git init").output().unwrap_or_else(|err|{
        error::process_error(error::Error::ShineError(
            error::ShineErrorType::CommandLineError,
            format!("Command writing failed with error code: {}", err)
        ));
    });

    fs::create_dir("src").unwrap_or_else(|_|{
        error::process_error(error::Error::ShineError(
            error::ShineErrorType::CommandLineError,
            String::from("Can't create new directory for new-command")
        ));
    });
    fs::File::create_new("src/main.shi").unwrap_or_else(|_|{
        error::process_error(error::Error::ShineError(
            error::ShineErrorType::CommandLineError,
            String::from("Can't create new file for new-command")
        ));
    });
    
    process::Command::new("cd").arg("..").output().unwrap_or_else(|err|{
        error::process_error(error::Error::ShineError(
            error::ShineErrorType::CommandLineError,
            format!("Command writing failed with error code: {}", err)
        ));
    });
}

fn process_ver_cmd() -> () {
    println!("Shine version {}\nReleased on {}\nby {}", VERSION, CURR_VER_RELEASE, CREATOR);
}