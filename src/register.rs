pub struct Register {
    // A Register
    pub a: u8,
    // B Register
    pub b: u8,
    // Carry Flag
    pub c: u8,
    // Program Counter
    pub pc: u8,
}

impl Register {
    pub fn new() -> Self {
        Self{
            a: 0b0000,
            b: 0b0000,
            c: 0b0000,
            pc: 0b0000,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0b000;
        self.b = 0b0000;
        self.c = 0b0000;
        self.pc = 0b0000;
    }

    pub fn reset_pc(&mut self) {
        self.pc = 0b0000;
    }

    pub fn increment_pc(&mut self) {
        self.pc += 1;
    }
}
