pub enum Color {
    Green,
    Yellow,
    Red,
    White,
}

pub trait Output {
    fn output(msg: String, color: Color);
}