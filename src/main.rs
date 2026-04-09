mod modules;

use modules::shine_output::error_output::error_output::{ErrorOutput, Error};
use std::env;

fn main(){
    let args: Vec<String> = env::args()
                                .skip(1)
                                .collect();

    if args.len() < 1 {
        Error::abort(String::from("Too few arguments!"), -1);
    }
}