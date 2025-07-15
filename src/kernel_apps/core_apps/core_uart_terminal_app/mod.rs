pub mod commands;
use crate::{
    kernel_apps::{
        core_apps::core_uart_terminal_app::commands::{
            send_string_terminal_formatted, send_terminal_user_string,
        },
        kernel_apps_manager::KERNEL_APPS_MANAGER,
    },
    utils::string::ascii::AsciiChar,
};
use commands::{Command, CommandResult};
pub const TERMINAL_BUFFER_SIZE: usize = MINI_UART_RX_BUFFER_SIZE;

use super::{
    CoreKernelApp,
    core_uart_app::{
        rx::MINI_UART_RX_BUFFER_SIZE,
        tx::global_send::{send_char, send_string},
    },
};

pub struct CoreUartTerminalApp {
    run_command: bool,                   // Indica si se debe ejecutar el comando
    terminal_buffer_arrived_data: usize, // Cantidad de bytes nuevos sin procesar
    terminal_buffer_idx: usize,          // Cantidad total de bytes válidos en el buffer
    terminal_buffer: [u8; TERMINAL_BUFFER_SIZE], // Buffer de datos recibidos
}

impl CoreKernelApp for CoreUartTerminalApp {
    fn event_system_loop(&mut self) {
        // Si se llenó el buffer o se solicita limpieza, se limpia y se retorna
        if self.handle_clean_buffer(false) {
            return;
        } else if self.run_command {
            self.run_command();
            self.clear_buffer();
            return;
        }

        // Si hay datos nuevos por procesar, se envían (se imprimen)
        if self.terminal_buffer_arrived_data > 0 {
            self.handle_send_chars();
        }
    }
}

impl CoreUartTerminalApp {
    pub const fn new() -> Self {
        CoreUartTerminalApp {
            run_command: false,
            terminal_buffer_arrived_data: 0,
            terminal_buffer_idx: 0,
            terminal_buffer: [0u8; TERMINAL_BUFFER_SIZE],
        }
    }

    /// Se invoca desde la interrupción del UART RX.
    /// Se procesa la tecla de borrado de inmediato y se actualizan los índices.
    pub fn handle_mini_uart_rx_irq(&mut self, new_data: u8) {
        // Si se recibe Enter, se marca para ejecutar el comando
        if new_data == b'\r' || new_data == b'\n' {
            self.run_command = true;
            return;
        } else {
            if self.handle_clean_buffer(false) {
                return;
            }

            // Si se presiona Backspace o Delete, se actualizan los índices y se envía la secuencia visual
            if new_data == AsciiChar::Delete.to_byte() || new_data == AsciiChar::Backspace.to_byte()
            {
                if self.terminal_buffer_idx > 0 {
                    self.terminal_buffer_idx -= 1;
                    // Se asume que si se borró un byte, también se debe restar de los datos nuevos pendientes
                    if self.terminal_buffer_arrived_data > 0 {
                        self.terminal_buffer_arrived_data -= 1;
                    }
                    send_char(AsciiChar::Backspace.to_byte());
                    send_char(AsciiChar::Space.to_byte());
                    send_char(AsciiChar::Backspace.to_byte());
                }
                return;
            }

            // Inserta el byte recibido en la posición actual y actualiza los contadores
            self.terminal_buffer[self.terminal_buffer_idx] = new_data;
            self.terminal_buffer_idx += 1;
            self.terminal_buffer_arrived_data += 1;
        }
    }

    /// Limpia el buffer si se llena o se solicita forzosamente.
    /// Devuelve true si se limpió el buffer.
    fn handle_clean_buffer(&mut self, force_clean: bool) -> bool {
        if self.terminal_buffer_idx >= TERMINAL_BUFFER_SIZE || force_clean {
            // Envía secuencia para borrar el contenido visual del buffer
            for _ in 0..self.terminal_buffer_idx {
                send_char(AsciiChar::Backspace.to_byte());
                send_char(AsciiChar::Space.to_byte());
                send_char(AsciiChar::Backspace.to_byte());
            }

            if !force_clean {
                send_string("[Command Terminal] Buffer filled!\n\r");
            } else {
                send_string("Buffer was cleaned!!\n\r");
            }

            self.clear_buffer();
            return true;
        }
        false
    }

    /// Reinicia los contadores y el buffer
    fn clear_buffer(&mut self) {
        self.run_command = false;
        self.terminal_buffer = [0u8; TERMINAL_BUFFER_SIZE];
        self.terminal_buffer_idx = 0;
        self.terminal_buffer_arrived_data = 0;
    }

    /// Ejecuta el comando recibido.
    /// Se utiliza el slice correcto, de 0 hasta terminal_buffer_idx (sin sumar 1).
    fn run_command(&mut self) {
        let command_bytes = &self.terminal_buffer[0..self.terminal_buffer_idx];
        let mut command_result: CommandResult = CommandResult::UnknownCommand;

        // Se intenta encontrar un espacio para separar comando y parámetros.
        if let Some(space_idx) = command_bytes
            .iter()
            .position(|&b| b == AsciiChar::Space.to_byte())
        {
            // Se encontró un espacio: el comando es lo que está antes, y los parámetros lo que está después.
            command_result =
                Command::run_command(&command_bytes[..space_idx], &command_bytes[space_idx + 1..]);
        } else {
            // No se encontró espacio: se usa todo el buffer como comando.
            command_result = Command::run_command(&command_bytes[..], &[]);
        }

        // Se procesa el resultado del comando.
        match command_result {
            CommandResult::Ok => {
                send_terminal_user_string(true);
            }
            CommandResult::Error(e) => {
                send_string("[CommandError] ");
                send_string(e);
                send_string("\n\r");
            }
            CommandResult::UnknownCommand => {
                send_string("\n\rUnknown command");
                send_terminal_user_string(true);
            }
            CommandResult::InvalidBytes => {
                send_string_terminal_formatted("\n\rInvalid bytes\n\r");
            }
            CommandResult::CommandHandledResult => {}
        };
    }

    /// Función auxiliar para enviar un u8 en formato decimal.
    fn send_u8_decimal(mut value: u8) {
        let mut buf = [0u8; 3];
        let mut i = 0;

        if value == 0 {
            send_char(b'0');
        } else {
            while value > 0 && i < buf.len() {
                buf[i] = (value % 10) + b'0';
                value /= 10;
                i += 1;
            }
            for j in (0..i).rev() {
                send_char(buf[j]);
            }
        }
        send_string("\n\r");
    }

    /// Procesa y envía los caracteres recién recibidos (el slice de nuevos datos).
    fn handle_send_chars(&mut self) {
        let start_index = self.terminal_buffer_idx - self.terminal_buffer_arrived_data;
        let arrived_bytes = &self.terminal_buffer[start_index..self.terminal_buffer_idx];

        for &byte in arrived_bytes.iter() {
            // Aquí se pueden agregar más casos si se desean otros caracteres especiales
            let is_letter: bool = matches!(byte, b'a'..=b'z' | b'A'..=b'Z');
            if is_letter {
                send_char(byte);
            } else {
                // Por ejemplo, se podría imprimir espacios u otros caracteres según se requiera
                send_char(byte);
            }
        }

        // Se reinicia el contador de bytes nuevos
        self.terminal_buffer_arrived_data = 0;
    }
}
