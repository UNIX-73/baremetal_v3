use crate::kernel_apps::core_apps::core_uart_app::tx::global_send::send_string;

use super::{CommandList, CommandResult};

impl CommandList {
    pub fn test(args: &str) -> CommandResult {
        send_string("ran test\n\r");

        if args.len() > 0 {
            send_string("Params=");
            send_string(args);
        }

        CommandResult::CommandHandledResult
    }

    pub fn test2(args: &str) -> CommandResult {
        if args.len() != 0 {
            send_string("ran test2\n\r");

            CommandResult::Ok
        } else {
            return CommandResult::Error("The command does not support params");
        }
    }
}
