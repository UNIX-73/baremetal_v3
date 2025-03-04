use core_uart_app::CoreUartApp;
use core_uart_terminal_app::CoreUartTerminalApp;

pub mod core_uart_app;
pub mod core_uart_terminal_app;

pub trait CoreKernelApp {
    fn event_system_start(&mut self) {}
    fn event_system_loop(&mut self) {}
}

pub struct CoreApps {
    uart: CoreUartApp,
    uart_terminal: CoreUartTerminalApp,
}
impl CoreApps {
    pub const fn new() -> Self {
        CoreApps {
            uart: CoreUartApp::new(),
            uart_terminal: CoreUartTerminalApp::new(),
        }
    }

    pub fn handle_event_system_start(&mut self) {
        self.uart.event_system_start();
        self.uart_terminal.event_system_start();
    }

    pub fn handle_event_system_loop(&mut self) {
        self.uart.event_system_loop();
        self.uart_terminal.event_system_loop();
    }

    pub fn uart(&mut self) -> &mut CoreUartApp {
        &mut self.uart
    }
}
