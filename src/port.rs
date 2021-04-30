pub struct Port {
   pub input: u8,
   pub output: u8,
}

impl Port {
    pub fn new() -> Self {
        Self {
            input: 0b0000,
            output: 0b0000,
        }
    }

    pub fn reset(&mut self) {
        self.input = 0b0000;
        self.output = 0b0000;
    }
}
