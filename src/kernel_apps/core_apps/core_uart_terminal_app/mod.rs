pub mod commands;
use core::ascii;

use commands::TerminalCommand;

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
            self.run_command();
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
                send_string("\n[Command Terminal] Buffer filled!");

                return; // Salir después de manejar el buffer lleno
            }
        }
    }

    fn run_command(&mut self) {
        let mut valid_command: bool = true;
        let mut command: TerminalCommand;

        KERNEL_APPS_MANAGER.lock(|m| {
            let rx = m.core().uart().rx().get_buffer();
            let data = rx.normalized();
            let command_data = &data[self.command_start..MINI_UART_RX_BUFFER_SIZE];

            let mut command_chars: [u8; MAX_TERMINAL_WORD_CHARS] =
                [AsciiChar::Null.to_byte(); MAX_TERMINAL_WORD_CHARS];

            for i in 0..MAX_TERMINAL_WORD_CHARS {
                if command_data[i] != AsciiChar::Space.to_byte() {
                    command_chars[i] = command_data[i];
                }

                ascii::Char::

                match TerminalCommand::get_command(&command_chars[..i + 1]) {
                    TerminalCommand::Null => continue,
                    TerminalCommand::Test => {}
                    TerminalCommand::Test2 => {}
                }
            }
        });

        if valid_command {}

        self.command_start = 0;
        self.run_command = false;
    }

    fn handle_command(command: [[u8; MAX_TERMINAL_WORD_CHARS]; MAX_TERMINAL_WORDS]) {}

    fn throw_terminal_error(&self, error: u32) {
        send_string("\n\rdefault terminal error\n\r");
    }
}
