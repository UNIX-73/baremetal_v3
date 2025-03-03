use crate::{
    kernel_apps::{
        core_apps::{CoreKernelApp, core_uart_app::rx::rx_irq::handle_irq_mini_uart_irq_rx},
        kernel_apps_manager::KERNEL_APPS_MANAGER,
    },
    peripherals::aux::{
        AUX_BASE, AUX_MU_IER_REG_OFFSET, AUX_MU_IIR_REG_OFFSET, AUX_MU_IO_REG_OFFSET,
    },
};

/// Máscaras de bits para el registro IIR
const IIR_RX_READY: u32 = 0b10 << 1; // Interrupción por datos en RX (bits 3:1)
const IER_RX_DISABLE: u32 = 0xFFFFFFFE; // Deshabilitar RX (bits 0 a 0)

#[unsafe(no_mangle)]
pub extern "C" fn show_invalid_entry_message() {
    KERNEL_APPS_MANAGER.lock(|m| {
        m.core()
            .uart()
            .tx()
            .b_send_string("\n\rInvalid interrupt\n\r");
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn handle_irq() {
    unsafe {
        // Leemos el registro de identificación de interrupciones (IIR)
        let irq_status: u32 =
            core::ptr::read_volatile((AUX_BASE + AUX_MU_IIR_REG_OFFSET) as *const u32);

        // Verificamos si la interrupción es de RX, comprobando los bits 3:1
        if irq_status & 0b110 == IIR_RX_READY {
            // Leemos el byte recibido del registro MU_IO.
            // Si el registro es de 8 bits, es más adecuado tratarlo como *const u8.
            let data: u8 = core::ptr::read_volatile((AUX_BASE + AUX_MU_IO_REG_OFFSET) as *const u8);

            KERNEL_APPS_MANAGER.lock(|m| {
                handle_irq_mini_uart_irq_rx(m.core().uart().mut_rx(), data);
            });
        }
    }
}
