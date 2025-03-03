pub trait KernelApp: Send + Sync {
    fn event_start(&mut self) {}
    fn event_loop(&mut self) {}
}
