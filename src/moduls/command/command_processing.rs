include!("../meta.rs");

use std::fs;
use std::process;

fn process_new_cmd(dir_name: &String) -> Result<(), std::io::Error> {
//  make a directory
    fs::create_dir(dir_name)?;
    process::Command::new("cd").arg(dir_name).output()?;
    
//  make a support files and directories
    fs::File::create_new(".gitignore")?;
    fs::File::create_new("ShineMT.json")?;
    process::Command::new("git init").output()?;

    fs::create_dir("src")?;
    fs::File::create_new("src/main.shi")?;
    
    process::Command::new("cd").arg("..").output()?;
    return Result::Ok(());
}

fn process_ver_cmd() -> () {
    println!("Shine version {}\nReleased on {}\nby {}", VERSION, CURR_VER_RELEASE, CREATOR);
}