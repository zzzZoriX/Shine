const L_VERSION: &str = "?version";
const S_VERSION: &str = "?v";
const L_HELP: &str = "?help";
const S_HELP: &str = "?h";
const L_BUILD: &str = "?build";
const S_BUILD: &str = "?b";


pub fn is_command(word: &String) -> bool {
    match word.as_str() {
        L_VERSION | S_VERSION | L_BUILD | S_BUILD | L_HELP | S_HELP => true,
        _ => false,
    }
}