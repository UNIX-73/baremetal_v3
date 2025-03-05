pub mod commands;

use commands::{Command, CommandResult};

use crate::{
    kernel_apps::kernel_apps_manager::KERNEL_APPS_MANAGER, utils::string::ascii::AsciiChar,
};

pub const IRQ_DATA_SIZE: usize = 32;
pub const TERMINAL_BUFFER_SIZE: usize = 64;

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

    irq_data_idx: usize,
    irq_data: [u8; IRQ_DATA_SIZE], //la data que llega desde el irq se gestiona en el loop, no en el irq
}

impl CoreKernelApp for CoreUartTerminalApp {
    fn event_system_loop(&mut self) {
        if self.hanfle_clean_buffer(false) {
            return; // Si es true se ha llenado el buffer, se elimina y gestiona sólo
        }

        if self.run_command {
            send_string("\n\r");
            self.run_command();
            self.run_command = false;
            self.command_start = 0;
            self.irq_data_idx = 0;

            return;
        }

        if self.irq_data_idx != 0 {
            if self.irq_data_idx >= IRQ_DATA_SIZE {
                self.hanfle_clean_buffer(true);
                send_string("IRQ buffer was filled between interrupts\n\r");

                return;
            }

            // Aquí se gestiona que se reenvíe al usuario la data que ha llegado
            let arrived_data = &self.irq_data[0..self.irq_data_idx];

            for i in arrived_data {
                send_char(*i);
            }

            self.irq_data_idx = 0;
        }
    }
}

impl CoreUartTerminalApp {
    pub const fn new() -> Self {
        CoreUartTerminalApp {
            command_start: 0,   // Iniciar el índice en 0
            run_command: false, // No ejecutar ningún comando al principio

            irq_data_idx: 0,
            irq_data: [0; IRQ_DATA_SIZE],
        }
    }

    pub fn handle_mini_uart_rx_irq(&mut self, new_data: u8) {
        // Si el dato recibido es un salto de línea (Enter)
        if new_data == b'\r' || new_data == b'\n' {
            self.run_command = true;

            return; // Salir de la función una vez que se detecte un comando
        } else {
            self.irq_data[self.irq_data_idx] = new_data; //añadimos la data para gestionarse en el loop
            self.irq_data_idx += 1;
        }

        // Si no se ha iniciado un comando, enviamos el dato recibido
        if !self.run_command {
            // Incrementamos el índice del buffer de comandos
            self.command_start += 1;
        }
    }

    fn hanfle_clean_buffer(&mut self, force_clean: bool) -> bool {
        if self.command_start >= MINI_UART_RX_BUFFER_SIZE || force_clean {
            // Enviar secuencias para borrar el contenido del buffer
            for _ in 0..self.command_start {
                send_char(AsciiChar::Backspace.to_byte()); // Mover el cursor hacia atrás
                send_char(AsciiChar::Space.to_byte()); // Sobrescribir con un espacio
                send_char(AsciiChar::Backspace.to_byte()); // Volver a mover el cursor hacia atrás
            }

            // Enviar mensaje indicando que el buffer está lleno
            if !force_clean {
                send_string("[Command Terminal] Buffer filled!\n\r");
            } else {
                send_string("Buffer was cleaned!!\n\r");
            }

            self.run_command = false;
            self.command_start = 0;
            self.irq_data_idx = 0;

            return true; // Salir después de manejar el buffer lleno
        }
        return false;
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

    fn handle_send_chars(&mut self, data: u8) {}
}
