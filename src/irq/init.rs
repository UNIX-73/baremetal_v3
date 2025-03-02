use crate::peripherals::PERIPHERAL_BASE;

const IRQ_ENABLE1: *mut u32 = (PERIPHERAL_BASE + 0xB210) as *mut u32;
/// El mini UART usa la IRQ 29 (bit 29 en Enable IRQs 1).
const MINI_UART_IRQ_BIT: u32 = 1 << 29;

pub fn enable_mini_uart_irq() {
    unsafe {
        let current = core::ptr::read_volatile(IRQ_ENABLE1);
        core::ptr::write_volatile(IRQ_ENABLE1, current | MINI_UART_IRQ_BIT);
    }
}
