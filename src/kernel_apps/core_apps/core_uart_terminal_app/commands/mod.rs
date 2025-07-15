use crate::kernel_apps::kernel_apps_manager::KERNEL_APPS_MANAGER;

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

#[inline]
pub fn send_string_terminal_formatted(msg: &str) {
    KERNEL_APPS_MANAGER.lock(|m| {
        m.core().uart.tx().b_send_string(&"\n\r");
        m.core().uart.tx().b_send_string(msg);
        send_terminal_user_string(true);
    })
}

#[inline]
pub fn send_terminal_user_string(new_line: bool) {
    KERNEL_APPS_MANAGER.lock(|m| {
        if new_line {
            m.core().uart.tx().b_send_string(&"\n\r");
        }
        m.core().uart.tx().b_send_string(&"[root] ");
    })
}
