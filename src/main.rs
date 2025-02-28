#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[unsafe(no_mangle)]
unsafe extern "C" fn rust_entry() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn _start_rust() -> ! {
    unsafe {
        gpio::set_function_select(21, gpio::FunctionSelect::Output);
    }
    let mut test1 = VtableTest { loops: 1 };
    let mut test2 = VtableTest2 { loops: 2 };

    loop {
        let blink_count_1 = tester(&mut test1);
        blink_fast(blink_count_1);

        delay_long();6

        let blink_count_2 = tester(&mut test2);
        blink_fast(blink_count_2);

        delay_long();
    }
}

/// Estructura 1 de prueba
struct VtableTest {
    loops: u32,
}

/// Estructura 2 de prueba
struct VtableTest2 {
    loops: u32,
}

/// Trait para pruebas con vtables
pub trait TraitTest {
    fn call_test(&mut self) -> u32;
}

impl TraitTest for VtableTest {
    fn call_test(&mut self) -> u32 {
        self.loops
    }
}

impl TraitTest for VtableTest2 {
    fn call_test(&mut self) -> u32 {
        self.loops *= 2;
        self.loops
    }
}

/// Llama a `call_test` de cualquier struct que implemente `TraitTest`
fn tester(trait_input: &mut dyn TraitTest) -> u32 {
    trait_input.call_test()
}

/// Hace parpadear el LED rápidamente la cantidad de veces indicada6
fn blink_fast(times: u32) {
    for _ in 0..times {
        unsafe {
            gpio::set_pin(21);
        }
        delay_short();
        unsafe {
            gpio::clear_pin(21);
        }
        delay_short();
    }
}

/// Retardo corto para parpadeo rápido
fn delay_short() {
    for _ in 0..(10999999u64 / 3) {
        unsafe { asm!("nop") }
    }
}

/// Retardo largo después de cada test
fn delay_long() {
    for _ in 0..10999999u64 {
        unsafe { asm!("nop") }
    }
}

pub mod gpio {
    const GPIO_BASE: usize = 0x3F200000;
    const GPIO_FSEL_OFFSET: usize = 0x00;
    const GPIO_SET_OFFSET: usize = 0x1C;
    const GPIO_CLR_OFFSET: usize = 0x28;
    pub const PIN_COUNT: u32 = 40;

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

    pub unsafe fn set_function_select(pin: u32, function: FunctionSelect) {
        if pin >= PIN_COUNT {
            return;
        }
        let fsel_register = pin / 10;
        let shift = (pin % 10) * 3;
        let addr = (GPIO_BASE + GPIO_FSEL_OFFSET + (fsel_register as usize * 4)) as *mut u32;
        let current: u32;
        unsafe {
            current = core::ptr::read_volatile(addr);
        }
        let mask = !(0b111 << shift);
        let new_val = (current & mask) | ((function as u32) << shift);
        unsafe {
            core::ptr::write_volatile(addr, new_val);
        }
    }

    pub unsafe fn set_pin(pin: u32) {
        if pin >= PIN_COUNT {
            return;
        }
        let set_register = pin / 32;
        let shift = pin % 32;
        let addr = (GPIO_BASE + GPIO_SET_OFFSET + (set_register as usize * 4)) as *mut u32;
        unsafe {
            core::ptr::write_volatile(addr, 1 << shift);
        }
    }

    pub unsafe fn clear_pin(pin: u32) {
        if pin >= PIN_COUNT {
            return;
        }
        let clr_register = pin / 32;
        let shift = pin % 32;
        let addr = (GPIO_BASE + GPIO_CLR_OFFSET + (clr_register as usize * 4)) as *mut u32;
        unsafe {
            core::ptr::write_volatile(addr, 1 << shift);
        }
    }
}
