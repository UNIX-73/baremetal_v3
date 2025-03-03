#![no_std]
#![no_main]

use core::panic::PanicInfo;

use kernel_apps::kernel_apps_manager::KERNEL_APPS_MANAGER;
use peripherals::gpio::{self, FunctionSelect, set_pin};

use utils::delay::loop_delay;
mod irq;
mod kernel_apps;
mod peripherals;
mod utils;

#[unsafe(no_mangle)]
unsafe extern "C" fn rust_entry() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_i: &PanicInfo) -> ! {
    loop {}
}

unsafe extern "C" {
    pub fn get_el() -> u64;
}

#[unsafe(no_mangle)]
extern "C" fn _start_rust() -> ! {
    gpio::set_function_select(21, FunctionSelect::Output);

    set_pin(21, true);
    loop_delay(20000000);
    KERNEL_APPS_MANAGER.lock(|m| {
        m.handle_event_system_start();
        let el = unsafe { get_el() }; // Supongamos que devuelve 1, 2, 3, etc.
        let el_ascii = (el as u8) + b'0'; // Convierte el número a su dígito ASCII correspondiente
        m.core().uart().tx().b_send_string("EL=");
        m.core().uart().tx().b_send_char(el_ascii);
        m.core()
            .uart()
            .tx()
            .b_send_string("\n\rHello from rust!\n\r");

        loop {
            m.handle_event_start();
            m.handle_event_loop();
        }
    });

    loop {}
}
