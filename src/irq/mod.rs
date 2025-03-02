use crate::peripherals::mini_uart::mini_uart_send;

#[unsafe(no_mangle)]
pub extern "C" fn show_invalid_entry_message() {
    mini_uart_send(b'I');
    mini_uart_send(b'R');
    mini_uart_send(b'Q');
    mini_uart_send(b'E');
    mini_uart_send(b'\r');
    mini_uart_send(b'\n');
}

#[unsafe(no_mangle)]
pub extern "C" fn handle_irq() {
    mini_uart_send(b'I');
    mini_uart_send(b'R');
    mini_uart_send(b'Q');
    mini_uart_send(b'\r');
    mini_uart_send(b'\n');
}
