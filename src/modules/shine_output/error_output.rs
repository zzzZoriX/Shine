mod error_output {
    use crate::modules::shine_output::output::{Color, Output};
    use colored::Colorize;

    pub struct Error;
    impl Output for Error {
        fn output(msg: String, color: Color) {
            match color {
                Color::Red => println!("{}", msg.red()),
                Color::Green => println!("{}", msg.green()),
                Color::Yellow => println!("{}", msg.yellow()),
                Color::White => println!("{}", msg)
            }
        }
    }
}