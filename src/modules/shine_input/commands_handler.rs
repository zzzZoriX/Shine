use crate::modules::shine_input::*;
use crate::modules::shine_output::*;
use crate::modules::shine_output::output::Out;

pub fn handle_command(input: &command_line::Input) {
    match input.command.as_str() {
        commands::S_HELP | commands::L_HELP => help_command_handler(),
        commands::S_VERSION | commands::L_VERSION => version_command_handler(),
        commands::S_BUILD | commands::L_BUILD => build_command_handler(),
        _ => () // never execute
    }
}


fn help_command_handler() {
    output::Output::output(commands::HELP_INFO.to_string(), output::Color::White);

    std::process::exit(0);
}

fn version_command_handler() {
    output::Output::output(commands::SHINE_VERSION.to_string(), output::Color::Green);

    std::process::exit(0);
}

fn build_command_handler() {
    todo!()
}