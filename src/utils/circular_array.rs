pub struct CircularArray<T, const S: usize> {
    pub buffer: [T; S],
    pub current_write_idx: usize,
    pub overfilled: bool,
}

impl<T: Default + Copy, const S: usize> CircularArray<T, S> {
    pub fn new() -> Self {
        CircularArray {
            buffer: [T::default(); S],
            current_write_idx: 0,
            overfilled: false,
        }
    }

    pub fn push(&mut self, value: T) {
        self.buffer[self.current_write_idx] = value;
        self.current_write_idx = (self.current_write_idx + 1) % S;

        if self.current_write_idx == 0 {
            self.overfilled = true;
        }
    }

    pub fn normalized(&self) -> [T; S] {
        let mut result = self.buffer;
        result.rotate_left(self.current_write_idx);

        result
    }

    pub fn get_size(&self) -> usize {
        S
    }
}
