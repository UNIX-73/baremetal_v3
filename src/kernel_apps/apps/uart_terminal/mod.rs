use crate::kernel_apps::kernel_app::KernelApp;

pub struct UartTerminal {

}
impl KernelApp for UartTerminal {
    fn event_loop(&mut self) {}
}
impl UartTerminal {
    pub const fn new() -> Self {
        UartTerminal {}
    }
}
