use crate::peripherals::PERIPHERAL_BASE;

/// Dirección del registro de habilitación de IRQs (IRQ Enable 1)
const IRQ_ENABLE1: *mut u32 = (PERIPHERAL_BASE + 0xB210) as *mut u32;

/// Enum que representa las interrupciones que se pueden habilitar.
/// Cada variante tiene el valor correspondiente (bitmask) en el registro de IRQ.
#[repr(u32)]
pub enum IrqEnableComponent {
    MiniUart = 1 << 29,
    // Puedes agregar más interrupciones, por ejemplo:
    // Timer1  = 1 << 1,
    // Timer2  = 1 << 2,
    // ...
}

/// Habilita la interrupción indicada en el controlador global.
/// Lee el registro IRQ_ENABLE1, aplica un OR con el bit correspondiente y lo vuelve a escribir.
pub fn irq_enable_component(component: IrqEnableComponent) {
    unsafe {
        let current = core::ptr::read_volatile(IRQ_ENABLE1);
        core::ptr::write_volatile(IRQ_ENABLE1, current | (component as u32));
    }
}
