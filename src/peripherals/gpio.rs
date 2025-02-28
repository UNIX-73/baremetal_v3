const GPIO_FSEL_OFFSET: usize = 0x00; // 4GPIO fsel de {00}[000][000][000][000][000][000][000][000][000][000] 10 pines
const GPIO_SET_OFFSET: usize = 0x1C;
const GPIO_CLR_OFFSET: usize = 0x28;
const GPIO_LEV_OFFSET: usize = 0x34;

const GPIO_PUD_OFFSET: usize = 0x94;
const GPIO_PUDCLK_OFFSET: usize = 0x98;

const GPIO_BASE: usize = 0x3F200000;
const GPIO_FSEL_BASE: usize = GPIO_BASE + GPIO_FSEL_OFFSET;
const P_GPIO_SET_BASE: usize = GPIO_BASE + GPIO_SET_OFFSET;
const P_GPIO_CLR_BASE: usize = GPIO_BASE + GPIO_CLR_OFFSET;
const P_GPIO_LEV_BASE: usize = GPIO_BASE + GPIO_LEV_OFFSET;

pub const PIN_COUNT: u32 = 40;

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum FunctionSelect {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010,
}

#[derive(Clone, Copy)]
pub enum ReadPinResult {
    low = 0,
    high = 1,
    error = 2,
}

pub fn set_function_select(pin: u32, function_select: FunctionSelect) {
    if pin >= PIN_COUNT {
        return;
    }

    let fsel_register_offset: usize = ((pin / 10) * 4) as usize;
    let shift_amount: u32 = (pin % 10) * 3;

    let mut current_value: u32;
    unsafe {
        current_value =
            core::ptr::read_volatile((GPIO_FSEL_BASE + fsel_register_offset) as *mut u32);
    }

    current_value &= !(0b111 << shift_amount);
    current_value |= (function_select as u32) << shift_amount;

    unsafe {
        core::ptr::write_volatile(
            (GPIO_FSEL_BASE as usize + fsel_register_offset) as *mut u32,
            current_value,
        );
    }
}

pub fn set_high(pin: u32) {
    if pin >= PIN_COUNT {
        return;
    }

    let set_register: usize = (pin as usize / 32) * 4;
    let bit_position: u32 = pin % 32;

    unsafe {
        core::ptr::write_volatile(
            (P_GPIO_SET_BASE + set_register) as *mut u32,
            0b1 << bit_position,
        );
    }
}

pub fn set_low(pin: u32) {
    if pin >= PIN_COUNT {
        return;
    }

    let clr_register: usize = (pin as usize / 32) * 4;
    let bit_position: u32 = pin % 32;

    unsafe {
        core::ptr::write_volatile(
            (P_GPIO_CLR_BASE + clr_register) as *mut u32,
            0b1 << bit_position,
        );
    }
}

pub fn set_pin(pin: u32, state: bool) {
    if state == true {
        set_high(pin);
    } else {
        set_low(pin);
    }
}

pub fn read_pin(pin: u32) -> ReadPinResult {
    if pin >= PIN_COUNT {
        return ReadPinResult::error; // Algo hay que hacer aquí
    }

    let lev_register: usize = (pin as usize / 32) * 4;
    let bit_position: u32 = pin % 32;

    let value: u32;
    unsafe {
        value = (core::ptr::read_volatile((P_GPIO_LEV_BASE + lev_register) as *mut u32)
            >> bit_position)
            & 0b1;
    }

    if value == 1 {
        return ReadPinResult::high;
    } else if value == 0 {
        return ReadPinResult::low;
    }

    return ReadPinResult::error;
}

// TODO: Escrita por chatgpt sin testear...
pub fn set_pull_up_down(pin: u32, pud: u32) {
    // Obtener los punteros a los registros GPPUD y GPPUDCLK
    let gppud = (GPIO_BASE + GPIO_PUD_OFFSET) as *mut u32;
    let gppudclk = (GPIO_BASE + GPIO_PUDCLK_OFFSET) as *mut u32;

    unsafe {
        // Escribimos la configuración de pull-up/down en GPPUD (0 para desactivar, 1 para pull-down, 2 para pull-up)
        core::ptr::write_volatile(gppud, pud);

        // Espera corta para que la señal se estabilice
        for _ in 0..150 {
            core::arch::asm!("nop", options(nomem, nostack));
        }

        // Aplicamos la configuración solo al pin indicado
        core::ptr::write_volatile(gppudclk, 1 << pin);

        // Otra espera corta
        for _ in 0..150 {
            core::arch::asm!("nop", options(nomem, nostack));
        }

        // Limpiamos los registros para finalizar la configuración
        core::ptr::write_volatile(gppud, 0);
        core::ptr::write_volatile(gppudclk, 0);
    }
}
