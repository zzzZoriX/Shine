use colored::Colorize;

pub enum Color {
    Green,
    Yellow,
    Red,
    White,
}

pub struct Output;
pub trait Out {
    fn output(msg: String, color: Color);
}

impl Out for Output {
    fn output(msg: String, color: Color) {
        match color {
            Color::Red => println!("{}", msg.red()),
            Color::Green => println!("{}", msg.green()),
            Color::Yellow => println!("{}", msg.yellow()),
            Color::White => println!("{}", msg)
        }
    }
}