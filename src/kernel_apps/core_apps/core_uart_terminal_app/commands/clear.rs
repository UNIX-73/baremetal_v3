use crate::kernel_apps::core_apps::core_uart_app::tx::global_send::send_string;

use super::{CommandList, CommandResult};

impl CommandList {
    pub fn clear(args: &str) -> CommandResult {
        if args.len() != 0 {
            return CommandResult::Error("This command does not support args\n\r");
        }

        let clear_screen = str::from_utf8(b"\x1B[2J\x1B[H");
        let clear_screen = match clear_screen {
            Ok(str) => str,
            Err(_) => return CommandResult::Error("Clear screen bytes are incorrect\n\r"),
        };

        send_string(clear_screen);

        return CommandResult::Ok;
    }
}
