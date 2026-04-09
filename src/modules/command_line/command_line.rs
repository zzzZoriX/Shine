pub mod command_line {
    use crate::modules::shine_output::error_output::error_output::{ErrorOutput, Error};

    pub const S_OTP_FLAG: &str = "-o";
    pub const L_OTP_FLAG: &str = "-output";


    pub struct Flags {
        output_f: bool
    }
    pub struct Input {
        inputs: Vec<String>,
        output: String,
        flags: Flags
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

            for word in cmd {
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


            Input{inputs, output, flags}
        }
    }
}