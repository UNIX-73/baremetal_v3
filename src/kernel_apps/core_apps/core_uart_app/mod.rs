use rx::CoreUartRx;
use tx::CoreUartTx;

use crate::{
    irq::init::{IrqEnableComponent, irq_enable_component},
    peripherals::{
        auxiliar::{
            AUX_BASE, AUX_ENABLES_OFFSET, AUX_MU_BAUD_REG_OFFSET, AUX_MU_CNTL_REG_OFFSET,
            AUX_MU_IER_REG_OFFSET, AUX_MU_IIR_REG_OFFSET, AUX_MU_LCR_REG_OFFSET,
            AUX_MU_MCR_REG_OFFSET,
        },
        gpio::{self, FunctionSelect},
    },
};

use super::CoreKernelApp;

pub mod rx;
pub mod tx;

pub struct CoreUartApp {
    tx: CoreUartTx,
    rx: CoreUartRx,
}
impl CoreKernelApp for CoreUartApp {
    fn event_system_start(&mut self) {
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

        irq_enable_component(IrqEnableComponent::MiniUart);
    }
}
impl CoreUartApp {
    pub const fn new() -> Self {
        CoreUartApp {
            tx: CoreUartTx::new(),
            rx: CoreUartRx::new(),
        }
    }
    pub fn tx(&self) -> &CoreUartTx {
        &self.tx
    }

    pub fn rx(&self) -> &CoreUartRx {
        &self.rx
    }

    pub fn mut_rx(&mut self) -> &mut CoreUartRx {
        &mut self.rx
    }
}
