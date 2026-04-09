pub const L_VERSION: &str = "?version";
pub const S_VERSION: &str = "?v";
pub const L_HELP: &str = "?help";
pub const S_HELP: &str = "?h";
pub const L_BUILD: &str = "?build";
pub const S_BUILD: &str = "?b";


pub fn is_command(word: &String) -> bool {
    match word.as_str() {
        L_VERSION | S_VERSION | L_BUILD | S_BUILD | L_HELP | S_HELP => true,
        _ => false,
    }
}