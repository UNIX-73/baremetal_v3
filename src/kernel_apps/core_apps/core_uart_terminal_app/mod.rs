pub mod commands;

use commands::{Command, CommandResult};

use crate::{
    kernel_apps::kernel_apps_manager::KERNEL_APPS_MANAGER, utils::string::ascii::AsciiChar,
};

pub const MAX_TERMINAL_WORDS: usize = 32;
pub const MAX_TERMINAL_WORD_CHARS: usize = 64;

use super::{
    CoreKernelApp,
    core_uart_app::{
        rx::MINI_UART_RX_BUFFER_SIZE,
        tx::global_send::{send_char, send_string},
    },
};

pub struct CoreUartTerminalApp {
    command_start: usize, // Índice para el buffer de comandos
    run_command: bool,    // Indica si el comando está en ejecución
}

impl CoreKernelApp for CoreUartTerminalApp {
    fn event_system_loop(&mut self) {
        if self.run_command {
            send_string("\n\r");
            self.run_command();
            self.run_command = false;
            self.command_start = 0;
        }
    }
}

impl CoreUartTerminalApp {
    pub const fn new() -> Self {
        CoreUartTerminalApp {
            command_start: 0,   // Iniciar el índice en 0
            run_command: false, // No ejecutar ningún comando al principio
        }
    }

    pub fn handle_mini_uart_rx_irq(&mut self, new_data: u8) {
        // Si el dato recibido es un salto de línea (Enter)
        if new_data == b'\r' || new_data == b'\n' {
            self.run_command = true;
            return; // Salir de la función una vez que se detecte un comando
        }

        // Si no se ha iniciado un comando, enviamos el dato recibido
        if !self.run_command {
            send_char(new_data);

            // Incrementamos el índice del buffer de comandos
            self.command_start += 1;

            // Si el buffer se ha llenado, comenzamos a borrar el contenido
            if self.command_start >= MINI_UART_RX_BUFFER_SIZE {
                self.command_start = 0;

                // Enviar secuencias para borrar el contenido del buffer
                for _ in 0..MINI_UART_RX_BUFFER_SIZE {
                    send_char(AsciiChar::Backspace.to_byte()); // Mover el cursor hacia atrás
                    send_char(AsciiChar::Space.to_byte()); // Sobrescribir con un espacio
                    send_char(AsciiChar::Backspace.to_byte()); // Volver a mover el cursor hacia atrás
                }

                // Enviar mensaje indicando que el buffer está lleno
                send_string("[Command Terminal] Buffer filled!\n\r");

                return; // Salir después de manejar el buffer lleno
            }
        }
    }

    /**
     * LLamado desde el system loop si se ha establecido en las interrupciones que se ha intentado mandar un comando
     */
    fn run_command(&mut self) {
        let mut command_result: CommandResult = CommandResult::UnknownCommand;

        let buffer: [u8; 1024] = KERNEL_APPS_MANAGER.lock(|m| {
            let rx = m.core().uart().rx();
            return rx.get_buffer().normalized(); //Normalizamos el buffer circular
        });

        let command_bytes =
            &buffer[MINI_UART_RX_BUFFER_SIZE - self.command_start - 1..MINI_UART_RX_BUFFER_SIZE];

        let command_len = command_bytes.len();

        for idx in 0..command_len {
            if command_bytes[idx] == AsciiChar::Space.to_byte() || idx + 1 == command_len {
                // Se suma 1 por el espacio que se sabe que hay entre el comando y los params
                command_result =
                    Command::run_command(&command_bytes[..idx], &command_bytes[idx + 1..]);

                break;
            }
        }

        // Result
        match command_result {
            CommandResult::Ok => {}
            CommandResult::Error(e) => {
                send_string("[CommandError] ");
                send_string(e);
            }
            CommandResult::UnknownCommand => {
                send_string("Unknown command\n\r");
            }
            CommandResult::InvalidBytes => {
                send_string("Invalid bytes\n\r");
            }
            CommandResult::CommandHandledResult => {}
        };
    }

    fn throw_terminal_error(&self, error: u32) {
        send_string("\n\rdefault terminal error\n\r");
    }
}
