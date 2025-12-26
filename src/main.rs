use std::env;
include!("./moduls/common/messages.rs");

fn main() {
    let args: Vec<String> = env::args()
                                .skip(1) // skip call command
                                .collect();

    if args.len() < 1 {
        panic!("{}", TOO_FEW_ARGS);
    }
}
