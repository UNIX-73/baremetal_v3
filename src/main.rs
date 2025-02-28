#![no_std]
#![no_main]

use core::panic::PanicInfo;

use peripherals::{
    gpio::{self, FunctionSelect, set_pin},
    mini_uart,
};
use utils::delay::loop_delay;
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

#[unsafe(no_mangle)]
extern "C" fn _start_rust() -> ! {
    gpio::set_function_select(21, FunctionSelect::Output);
    mini_uart::mini_uart_init();
    let mut pin_state: bool = true;
    loop {
        set_pin(21, pin_state);
        loop_delay(10000000);

        pin_state = !pin_state;
        mini_uart::mini_uart_send(b'H');
        mini_uart::mini_uart_send(b'E');
        mini_uart::mini_uart_send(b'L');
        mini_uart::mini_uart_send(b'L');
        mini_uart::mini_uart_send(b'O');
        mini_uart::mini_uart_send(b'\n');
        mini_uart::mini_uart_send(b'\r');
    }
}
