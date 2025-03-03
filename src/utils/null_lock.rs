use core::cell::UnsafeCell;

/// Un pseudo-lock que garantiza acceso exclusivo mientras se ejecuta el closure.
pub struct NullLock<T: ?Sized> {
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for NullLock<T> where T: Send {}

impl<T> NullLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }

    /// Toma un closure que recibe una referencia mutable exclusiva al dato.
    pub fn lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // Como estamos en un sistema bare-metal de un solo núcleo con interrupciones deshabilitadas,
        // no es necesario bloquear contra otros hilos.
        let data = unsafe { &mut *self.data.get() };
        f(data)
    }
}
