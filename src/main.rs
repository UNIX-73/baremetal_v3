#![no_std]
#![no_main]

use core::panic::PanicInfo;

use irq::init::enable_mini_uart_irq;
use peripherals::{
    gpio::{self, FunctionSelect, set_pin},
    mini_uart,
};

use utils::delay::loop_delay;
mod irq;
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
    mini_uart::mini_uart_init();
    let mut pin_state: bool = true;

    mini_uart::mini_uart_send(b'\n');
    mini_uart::mini_uart_send(b'\r');

    enable_mini_uart_irq();

    loop {
        mini_uart::mini_uart_send(b'H');
        mini_uart::mini_uart_send(b'E');
        mini_uart::mini_uart_send(b'L');
        mini_uart::mini_uart_send(b'L');
        mini_uart::mini_uart_send(b'O');
        mini_uart::mini_uart_send(b'\n');
        mini_uart::mini_uart_send(b'\r');
        unsafe {
            let el = get_el(); // Supongamos que devuelve 1, 2, 3, etc.
            let el_ascii = (el as u8) + b'0'; // Convierte el número a su dígito ASCII correspondiente
            mini_uart::mini_uart_send(el_ascii);
        }
        mini_uart::mini_uart_send(b'-');

        set_pin(21, pin_state);
        loop_delay(20000000);
        pin_state = !pin_state;
    }
}
