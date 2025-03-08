pub mod clear;
pub mod test;
pub struct CommandList;

type CommandFunction = fn(&str) -> CommandResult;

pub struct Command(&'static str, CommandFunction);

pub enum CommandResult {
    Ok,
    Error(&'static str),
    UnknownCommand,
    InvalidBytes,
    CommandHandledResult,
}

impl Command {
    pub fn run_command(command: &[u8], args: &[u8]) -> CommandResult {
        let str_command = str::from_utf8(command);
        let str_command = match str_command {
            Ok(res) => res,
            Err(_) => return CommandResult::InvalidBytes, //TODO: Implementar el return de error
        };

        for terminal_command in TERMINAL_COMMAND_LIST.iter() {
            if str_command == terminal_command.0 {
                let str_args = str::from_utf8(args);
                let str_args = match str_args {
                    Ok(res) => res,
                    Err(_) => return CommandResult::InvalidBytes,
                };

                return terminal_command.1(str_args); // Ejecutamos el comando
            }
        }

        return CommandResult::UnknownCommand;
    }
}

pub static TERMINAL_COMMAND_LIST: &[Command] = &[
    Command("test", CommandList::test),
    Command("test2", CommandList::test2),
    Command("clear", CommandList::clear),
    Command("cls", CommandList::clear),
];
