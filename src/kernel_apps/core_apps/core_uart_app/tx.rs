use crate::peripherals::aux::{AUX_BASE, AUX_MU_IO_REG_OFFSET, AUX_MU_LSR_REG_OFFSET};

pub struct CoreUartTx {}

impl CoreUartTx {
    pub const fn new() -> Self {
        CoreUartTx {}
    }

    pub fn b_send_char(&self, c: u8) {
        unsafe {
            // Esperar hasta que el bit 5 (transmisor listo) esté activo en AUX_MU_LSR_REG
            while (core::ptr::read_volatile((AUX_BASE + AUX_MU_LSR_REG_OFFSET) as *mut u32)
                & (1 << 5))
                == 0
            {}

            // Escribir el byte en el registro de I/O del mini UART
            core::ptr::write_volatile((AUX_BASE + AUX_MU_IO_REG_OFFSET) as *mut u32, c as u32);
        }
    }

    pub fn b_send_string(&self, msg: &str) {
        for c in msg.chars() {
            self.b_send_char(c as u8);
        }
    }
}
