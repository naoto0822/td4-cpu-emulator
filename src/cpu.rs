use crate::op::OpCode;
use crate::rom::Rom;
use crate::register::Register;
use crate::port::Port;

pub struct Cpu {
    pub rom: Rom,
    pub register: Register,
    pub port: Port,
}

impl Cpu {
    pub fn new(rom: Rom) -> Self {
        let mut register = Register::new();
        register.reset();

        let mut port = Port::new();
        port.reset();

        Self {
            rom,
            register,
            port,
        }
    }
    // loop
    // 1 fetch op
    // 2 decode op
    // 3 execute
    // 4 count up
    pub fn run(&self) {
        loop {
            // 1
            let address = self.register.pc;
            if address > (self.rom.size() - 1) {
                // finish
                break;
            }

            // 2
            let op = self.rom.fetch(address);
            let (opcode, im) = self.decode(op);

            // 3
            self.execute(opcode, im);

            // 4
            if &opcode == OpCode::Jmp || &opcode == OpCode::Jnc {
                continue;
            }
            self.register.pc += 1;
        }
    }

    fn decode(&self, op: u8) -> (OpCode, u8) {
        // TODO: comment
        let shifted_op = op >> 4;
        // TODO: comment
        let shifted_im = op & 0x0f;

        match shifted_op {
            // TODO: using Enum OpCode
            0b0011 => (OpCode::MovA, shifted_im),
            0b0111 => (OpCode::MovB, shifted_im),
            0b0001 => (OpCode::MovAB, 0b0000),
            0b0100 => (OpCode::MovBA, 0b0000),
            0b0000 => (OpCode::AddA, shifted_im),
            0b0101 => (OpCode::AddB, shifted_im),
            0b0010 => (OpCode::InA, 0b0000),
            0b0110 => (OpCode::InB, 0b0000),
            0b1011 => (OpCode::Out, shifted_im),
            0b1001 => (OpCode::OutB, 0b0000),
            0b1111 => (OpCode::Jmp, shifted_im),
            0b1110 => (OpCode::Jnc, shifted_im),
            _ => panic!("not support op code!")
        }
    }

    fn execute(&self, opcode: OpCode, im: u8) {
        match opcode {
            OpCode::MovA => self.mov_a(im),
            OpCode::MovB => self.mov_b(im),
            OpCode::MovAB => self.mov_ab(),
            OpCode::MovBA => self.mov_ba(),
            OpCode::AddA => self.add_a(im),
            OpCode::AddB => self.add_b(im),
            OpCode::InA => self.in_a(),
            OpCode::InB => self.in_b(),
            OpCode::Out => self.out(im),
            OpCode::OutB => self.out_b(),
            OpCode::Jmp => self.jmp(im),
            OpCode::Jnc => self.jnc(im),
        }
    }

    fn mov_a(&self, im: u8) {
        self.register.a = im;
        self.register.c = 0;
    }

    fn mov_b(&self, im: u8) {
        self.register.b = im;
        self.register.c = 0;
    }

    fn mov_ab(&self) {
        let b = self.register.b;
        self.register.a = b;
        self.register.c = 0;
    }

    fn mov_ba(&self) {
        let a = self.register.a;
        self.register.b = a;
        self.register.c = 0;
    }

    fn add_a(&self, im: u8) {
        let a = self.register.a;
        let added = a + im;
        if added > 0x0f {
            self.register.c = 1;
        }

        let shifted = added & 0x0f;
        self.register.a = shifted;
    }

    fn add_b(&self, im: u8) {
        let b = self.register.a;
        let added = b + im;
        if added > 0x0f {
            self.register.c = 1;
        }

        let shifted = added & 0x0f;
        self.register.b = shifted;

    }

    fn in_a(&self) {
        let input = self.port.input;
        self.register.a = input;
        self.register.c = 0;
    }

    fn in_b(&self) {
        let input = self.port.input;
        self.register.b = input;
        self.register.c = 0;
    }

    fn out(&self, im: u8) {
        self.port.output = im;
        self.register.c = 0;
    }

    fn out_b(&self) {
        let b = self.register.b;
        self.port.output = b;
        self.register.c = 0;
    }

    fn jmp(&self, im: u8) {
        self.register.pc = im;
        self.register.c = 0;
    }

    fn jnc(&self, im: u8) {
        if self.register.c == 0 {
            self.register.pc = im;
        } else {
            self.register.pc += 1;
        }
        self.register.c = 0;
    }
}
