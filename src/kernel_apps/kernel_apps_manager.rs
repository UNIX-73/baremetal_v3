use crate::utils::null_lock::NullLock;

use super::{core_apps::CoreApps, kernel_app::KernelApp};
pub const KERNEL_APPS_SIZE: usize = 128;

pub struct KernelAppManager<'a> {
    apps: [Option<&'a mut dyn KernelApp>; KERNEL_APPS_SIZE],
    started_apps: [bool; KERNEL_APPS_SIZE], // true si ha iniciado, false si no

    core: CoreApps,
}
impl<'a> KernelAppManager<'a> {
    pub fn subscribe_app(&mut self, new_app: &'a mut dyn KernelApp) {
        for slot in self.apps.iter_mut() {
            if slot.is_none() {
                *slot = Some(new_app);
                return; // Salimos después de agregar el primer `None`
            }
        }
    }

    pub fn core(&mut self) -> &mut CoreApps {
        &mut self.core
    }

    pub fn handle_event_system_start(&mut self) {
        self.core.handle_event_system_start();
    }

    pub fn handle_event_system_loop(&mut self) {
        self.core.handle_event_system_loop();
    }

    pub fn handle_event_start(&mut self) {
        for i in 0..KERNEL_APPS_SIZE {
            match self.apps[i].as_mut() {
                None => {}
                Some(app) => {
                    if !self.started_apps[i] {
                        app.event_start();
                        self.started_apps[i] = true;
                    }
                }
            }
        }
    }

    pub fn handle_event_loop(&mut self) {
        for i in 0..KERNEL_APPS_SIZE {
            match self.apps[i].as_mut() {
                None => {}
                Some(app) => {
                    app.event_loop();
                }
            }
        }
    }
}

pub static KERNEL_APPS_MANAGER: NullLock<KernelAppManager> = NullLock::new(KernelAppManager {
    apps: [const { None }; KERNEL_APPS_SIZE],
    started_apps: [false; KERNEL_APPS_SIZE],
    core: CoreApps::new(),
});
