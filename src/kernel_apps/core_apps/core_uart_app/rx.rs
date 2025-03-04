use crate::{
    kernel_apps::kernel_apps_manager::KERNEL_APPS_MANAGER, utils::circular_array::CircularArray,
};

pub const MINI_UART_RX_BUFFER_SIZE: usize = 1024;
pub struct CoreUartRx {
    buffer: CircularArray<u8, MINI_UART_RX_BUFFER_SIZE>,
}
impl CoreUartRx {
    pub const fn new() -> Self {
        CoreUartRx {
            buffer: CircularArray {
                buffer: [0; MINI_UART_RX_BUFFER_SIZE],
                current_write_idx: 0,
                overfilled: false,
            },
        }
    }

    pub fn notify_core_apps(new_data: u8) {
        KERNEL_APPS_MANAGER.lock(|m| {
            m.core().uart_terminal.handle_mini_uart_rx_irq(new_data);
        })
    }

    pub fn get_buffer(&self) -> &CircularArray<u8, MINI_UART_RX_BUFFER_SIZE> {
        &self.buffer
    }
}

pub mod rx_irq {
    use super::CoreUartRx;

    pub fn handle_irq_mini_uart_irq_rx(rx: &mut CoreUartRx, data: u8) {
        rx.buffer.push(data);

        CoreUartRx::notify_core_apps(data);
    }
}
