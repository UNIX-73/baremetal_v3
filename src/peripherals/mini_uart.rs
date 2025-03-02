use super::gpio::{self, FunctionSelect};

const AUX_BASE: usize = 0x3F215000;

const IRQ_STATUS_OFFSET: usize = 0x0;
const AUX_ENABLES_OFFSET: usize = 0x04;
const AUX_MU_IO_REG_OFFSET: usize = 0x40;
const AUX_MU_IER_REG_OFFSET: usize = 0x44;
const AUX_MU_IIR_REG_OFFSET: usize = 0x48;
const AUX_MU_LCR_REG_OFFSET: usize = 0x4C;
const AUX_MU_MCR_REG_OFFSET: usize = 0x50;
const AUX_MU_LSR_REG_OFFSET: usize = 0x54;
const AUX_MU_MSR_REG_OFFSET: usize = 0x58;
const AUX_MU_SCRATCH_OFFSET: usize = 0x5C;
const AUX_MU_CNTL_REG_OFFSET: usize = 0x60;
const AUX_MU_STAT_REG_OFFSET: usize = 0x64;
const AUX_MU_BAUD_REG_OFFSET: usize = 0x68;

/// Inicializa el mini UART:
/// 1. Configura los pines GPIO 14 (TX) y 15 (RX) para la función alternativa Alt5.
/// 2. Desactiva los pull-up/down en dichos pines.
/// 3. Habilita el mini UART, desactiva transmisor y receptor para la configuración, limpia FIFOs,
///    configura el modo de 8 bits, establece la velocidad de baudios y finalmente activa el transmisor y receptor.
pub fn mini_uart_init() {
    // Configurar pines GPIO para mini UART (TX y RX)
    gpio::set_function_select(14, FunctionSelect::Alt5);
    gpio::set_function_select(15, FunctionSelect::Alt5);

    // Deshabilitar pull-up/down en TX y RX
    gpio::set_pull_up_down(14, 0);
    gpio::set_pull_up_down(15, 0);

    unsafe {
        //  Deshabilitar interrupciones en el mini UART (IER_REG = 0)
        core::ptr::write_volatile((AUX_BASE + AUX_MU_IER_REG_OFFSET) as *mut u32, 0);

        //  Habilitar el mini UART escribiendo 1 en AUX_ENABLES
        core::ptr::write_volatile((AUX_BASE + AUX_ENABLES_OFFSET) as *mut u32, 1);

        //  Deshabilitar transmisor y receptor mientras se configura (CNTL_REG = 0)
        core::ptr::write_volatile((AUX_BASE + AUX_MU_CNTL_REG_OFFSET) as *mut u32, 0);

        //  Limpiar las FIFOs y reiniciarlas (IIR_REG = 0xC6)
        core::ptr::write_volatile((AUX_BASE + AUX_MU_IIR_REG_OFFSET) as *mut u32, 0xC6);

        //  Configurar la transmisión a 8 bits (LCR_REG = 3)
        core::ptr::write_volatile((AUX_BASE + AUX_MU_LCR_REG_OFFSET) as *mut u32, 3);

        //  Deshabilitar el control de modem (MCR_REG = 0)
        core::ptr::write_volatile((AUX_BASE + AUX_MU_MCR_REG_OFFSET) as *mut u32, 0);

        //  Establecer la velocidad de baudios (BAUD_REG).
        //    Para 115200 baudios, el valor suele ser 270 (valor dependiente del reloj base).
        core::ptr::write_volatile((AUX_BASE + AUX_MU_BAUD_REG_OFFSET) as *mut u32, 270);

        //  Habilitar transmisor y receptor (CNTL_REG = 3)
        core::ptr::write_volatile((AUX_BASE + AUX_MU_CNTL_REG_OFFSET) as *mut u32, 3);

        //  Habilitar interrupciones en el mini UART (IER_REG = 1)
        core::ptr::write_volatile((AUX_BASE + AUX_MU_IER_REG_OFFSET) as *mut u32, 1);
    }
}

/// Envía un byte por el mini UART.
/// Espera a que el transmisor esté listo (verificando el bit 5 del LSR) antes de escribir el dato.
pub fn mini_uart_send(c: u8) {
    unsafe {
        // Esperar hasta que el bit 5 (transmisor listo) esté activo en AUX_MU_LSR_REG
        while (core::ptr::read_volatile((AUX_BASE + AUX_MU_LSR_REG_OFFSET) as *mut u32) & (1 << 5))
            == 0
        {}

        // Escribir el byte en el registro de I/O del mini UART
        core::ptr::write_volatile((AUX_BASE + AUX_MU_IO_REG_OFFSET) as *mut u32, c as u32);
    }
}
