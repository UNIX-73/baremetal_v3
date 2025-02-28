use core::arch::asm;

pub fn loop_delay(loops: u64) {
    for _ in 1..loops {
        unsafe {
            asm!("nop");
        }
    }
}
