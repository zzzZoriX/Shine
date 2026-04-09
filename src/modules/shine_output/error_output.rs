use crate::modules::shine_output::output::*;

pub struct Error;
pub trait ErrorOutput {
    fn abort(msg: String, code: i8);
}

impl ErrorOutput for Error {
    fn abort(msg: String, code: i8) {
        Output::output(msg, Color::Red);

        std::process::exit(code as i32);
    }
}