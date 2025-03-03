use core_uart_app::CoreUartApp;

pub mod core_uart_app;

pub trait CoreKernelApp {
    fn event_system_start(&mut self) {}
}

pub struct CoreApps {
    uart: CoreUartApp,
}
impl CoreApps {
    pub const fn new() -> Self {
        CoreApps {
            uart: CoreUartApp::new(),
        }
    }

    pub fn handle_event_system_start(&mut self) {
        self.uart.event_system_start();
    }

    pub fn uart(&mut self) -> &mut CoreUartApp {
        &mut self.uart
    }
}
