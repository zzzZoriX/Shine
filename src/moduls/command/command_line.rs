include!("./commands.rs");

mod command_line {
use crate::{
    COMP_CMD, 
    INPUT_FLAG, 
    INPUT_FLAG_S, 
    NEW_CMD, 
    OUTPUT_FLAG, 
    OUTPUT_FLAG_S, 
    VERSION_CMD
};


    pub struct Input {
        has: bool,
        pub args: Vec<String>
    }

    pub struct Output {
        has: bool,
        pub arg: String
    }

    struct Flags {
        inp: Input,
        otp: Output
    }

    pub enum CmdType {
        ShowVersion,
        CreateNewEnv,
        Compile,
        Unknown
    }

    enum ParseStatus {
        Input,
        Output
    }

    pub struct Command {
        flags:      Flags,
        cmd_type:   CmdType
    }


    impl Input {
        pub fn new() -> Self {
            return Self {
                has: false,
                args: Vec::new()
            };
        }
    }

    impl Output {
        pub fn new() -> Self {
            return Self {
                has: false,
                arg: String::new()
            };
        }
    }

    impl Flags {
        pub fn new() -> Self {
            return Self {
                inp: Input::new(),
                otp: Output::new()
            };
        }
    }

    impl Command {
        pub fn new() -> Self {
            return Self {
                flags: Flags::new(),
                cmd_type: CmdType::Unknown
            };
        }

        pub fn parse(&mut self, args: &Vec<String>) -> Result<(), String> {
            let mut read_status: Option<ParseStatus> = None;

            for arg in args {
                if arg == INPUT_FLAG_S || arg == INPUT_FLAG {
                    if self.flags.inp.has {
                        return Result::Err("Multiply input flag entry".to_string());
                    }
                    
                    self.flags.inp.has = true;
                    read_status = Some(ParseStatus::Input);
                }

                else if arg == OUTPUT_FLAG_S || arg == OUTPUT_FLAG {
                    if self.flags.otp.has {
                        return Result::Err("Multiply output flag entry".to_string());
                    }

                    self.flags.otp.has = true;
                    read_status = Some(ParseStatus::Output);
                }

                else if arg == VERSION_CMD {
                    self.cmd_type = CmdType::ShowVersion;

                    return Result::Ok(());
                }

                else if arg == NEW_CMD {
                    self.cmd_type = CmdType::CreateNewEnv;

                    return Result::Ok(());
                }

                else if arg == COMP_CMD {
                    if let CmdType::Compile = self.cmd_type{
                        return Result::Err("Multiply compile command".to_string());
                    }

                    self.cmd_type = CmdType::Compile;
                }

                else {
                    if let Some(ParseStatus::Input) = read_status {
                        self.flags.inp.args.push(String::clone(arg));
                    }
                    else if let Some(ParseStatus::Output) = read_status {
                        if self.flags.otp.arg.len() > 0 {
                            return Result::Err("Already have the output file name".to_string());
                        }

                        self.flags.otp.arg = String::clone(arg);
                    }
                    else {
                        return Result::Err(format!("Unexpected file: {}", arg));
                    }
                }
            }

            return Result::Ok(());
        }

        // pub fn dbg_print(&self) -> () {
        //     print!("inp '{}': ", self.flags.inp.has);
        //     if self.flags.inp.has {
        //         for arg in &self.flags.inp.args {
        //             print!("{} ", arg);
        //         }
        //         println!();
        //     }

        //     print!("otp '{}': ", self.flags.otp.has);
        //     if self.flags.otp.has {
        //         println!("{}", self.flags.otp.arg);
        //     }
        // }
    }
}