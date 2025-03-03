const MINI_UART_RX_BUFFER_SIZE: usize = 1024;

pub struct CoreUartRx {
    buffer: [u8; MINI_UART_RX_BUFFER_SIZE],
    buffer_idx: usize,
}

impl CoreUartRx {
    pub const fn new() -> Self {
        CoreUartRx {
            buffer: [0u8; MINI_UART_RX_BUFFER_SIZE],
            buffer_idx: 0,
        }
    }

    pub fn get_last_data(&mut self) -> u8 {
        self.buffer[self.buffer_idx]
    }

    pub fn get_data(
        &mut self,
        data_buffer: &mut [u8; MINI_UART_RX_BUFFER_SIZE],
        mut length: usize,
    ) {
        if length > MINI_UART_RX_BUFFER_SIZE {
            length = MINI_UART_RX_BUFFER_SIZE;
        }

        let b_idx: usize = self.buffer_idx;

        todo!("implementar la función");
    }
}

pub mod rx_irq {
    use super::{CoreUartRx, MINI_UART_RX_BUFFER_SIZE};

    pub fn handle_irq_mini_uart_irq_rx(rx: &mut CoreUartRx, data: u8) {
        rx.buffer[rx.buffer_idx] = data;

        rx.buffer_idx += 1;

        if rx.buffer_idx >= MINI_UART_RX_BUFFER_SIZE {
            rx.buffer_idx = 0;
        }
    }
}
