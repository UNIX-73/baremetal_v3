use crate::kernel_apps::core_apps::{
    core_uart_app::tx::global_send::send_string,
    core_uart_terminal_app::commands::{send_string_terminal_formatted, send_terminal_user_string},
};

use super::{CommandList, CommandResult};

impl CommandList {
    pub fn test(args: &str) -> CommandResult {
        send_string("\n\rran test");

        if args.len() > 0 {
            send_string("\n\rparams=");
            send_string(args);
        }

        send_terminal_user_string(true);
        CommandResult::CommandHandledResult
    }

    pub fn test2(args: &str) -> CommandResult {
        if args.len() == 0 {
            send_string("ran test2");

            CommandResult::Ok
        } else {
            return CommandResult::Error("The command does not support params");
        }
    }
}
