include!("../meta.rs");

use std::fs;
use std::process;

fn process_new_cmd(dir_name: &String) -> () {
//  make a directory
    fs::create_dir(dir_name).unwrap_or_else(|_|{
        error::process_error(error::Error::CommandLineError(
            String::from("Can't create a directory")
        ));
    });
    process::Command::new("cd").arg(dir_name).output().unwrap_or_else(|_|{
        error::process_error(error::Error::CommandLineError(
            format!("Can't use command here")
        ));
    });
    
//  make a support files and directories
    fs::File::create_new(".gitignore").unwrap_or_else(|_|{
        error::process_error(error::Error::CommandLineError(
            String::from("Can't create new file for new-command")
        ));
    });
    process::Command::new("git init").output().unwrap_or_else(|_|{
        error::process_error(error::Error::CommandLineError(
            format!("Can't use command here")
        ));
    });

    fs::create_dir("src").unwrap_or_else(|_|{
        error::process_error(error::Error::CommandLineError(
            String::from("Can't create new directory for new-command")
        ));
    });
    fs::File::create_new("src/main.shi").unwrap_or_else(|_|{
        error::process_error(error::Error::CommandLineError(
            String::from("Can't create new file for new-command")
        ));
    });
    
    process::Command::new("cd").arg("..").output().unwrap_or_else(|_|{
        error::process_error(error::Error::CommandLineError(
            format!("Can't use command here")
        ));
    });
}

fn process_ver_cmd() -> () {
    println!("Shine version {}\nReleased on {}\nby {}", VERSION, CURR_VER_RELEASE, CREATOR);
}