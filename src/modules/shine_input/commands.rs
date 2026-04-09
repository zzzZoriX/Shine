pub const L_VERSION: &str = "?version";
pub const S_VERSION: &str = "?v";
pub const L_HELP: &str = "?help";
pub const S_HELP: &str = "?h";
pub const L_BUILD: &str = "?build";
pub const S_BUILD: &str = "?b";
pub const S_OTP_FLAG: &str = "-o";
pub const L_OTP_FLAG: &str = "-output";

pub const HELP_INFO: &str = "Available commands:\n\
  ?h | ?help    -- show this info message\n\
  ?v | ?version -- show current version of shine\n\
  ?b | ?build   -- compile all pointed .ne files\n\n\
  \
Available flags:\n\
  -o | -output  -- tell Shine that next file is an output\n\n\
  \
Shine source files extension: .ne\n\
Shine creator: zorix (github: https://github.com/zzzZoriX )";
pub const SHINE_VERSION: &str = "v0.01";


pub fn is_command(word: &String) -> bool {
    match word.as_str() {
        L_VERSION | S_VERSION | L_BUILD | S_BUILD | L_HELP | S_HELP => true,
        _ => false,
    }
}