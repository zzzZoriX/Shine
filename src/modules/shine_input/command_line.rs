use crate::modules::shine_input::commands;
use crate::modules::shine_input::commands::*;
use crate::modules::shine_output::error_output::error_output::{ErrorOutput, Error};


pub struct Flags {
    output_f: bool
}
pub struct Input {
    pub command: String,
    pub inputs: Vec<String>,
    pub output: String,
    pub flags: Flags
}
pub trait input {
    fn parse_command_line(cmd: Vec<String>) -> Input;
}
struct parse_stats {
    output_set: bool
}

impl input for Input {
    fn parse_command_line(cmd: Vec<String>) -> Input {
        let mut inputs: Vec<String> = Vec::new();
        let mut output: String = "shine_new".to_string();
        let mut flags: Flags = Flags{output_f: false};
        let mut stats: parse_stats = parse_stats{output_set: false};

        if !is_command(&cmd[0]) {
            Error::abort("Unknown command".to_string(), -2);
        }

        let command: String = cmd[0].clone();


        for word in cmd {
            if is_command(&word) {
                Error::abort(String::from("Unexpected command"), -3);
            }


            if word.contains(".ne") {
                if flags.output_f {
                    Error::abort(String::from("Input files defining after output flag!"), 1);
                }

                inputs.push(word.to_string());
            }
            else if !word.contains('-') && flags.output_f {
                if stats.output_set {
                    Error::abort(String::from("Multiply output file defining!"), 2);
                }

                output = word;

                stats.output_set = true;
            }

            else if word == L_OTP_FLAG || word == S_OTP_FLAG {
                if flags.output_f {
                    Error::abort(String::from("Multiply output flag defining"), 2);
                }

                flags.output_f = true;
            }
        }


        Input{command, inputs, output, flags}
    }
}