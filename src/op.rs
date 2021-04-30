#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    // MOV A Im
    // - 0011 Im
    MovA = 0b0011,
    // MOV B Im
    // - 0111 Im
    MovB = 0b0111,
    // MOV A B
    // - 00010000
    MovAB = 0b0001,
    // MOV B A
    // - 01000000
    MovBA = 0b0100,
    // ADD A Im
    // - 0000 Im
    AddA = 0b0000,
    // ADD B Im
    // - 0101 Im
    AddB = 0b0101,
    // In A
    // - 00100000
    InA = 0b0010,
    // In B Im
    // - 01100000
    InB = 0b0110,
    // OUT Im
    // - 1011 Im
    Out = 0b1011,
    // OUT B
    // - 10010000
    OutB = 0b1001,
    // JMP Im
    // - 1111 Im
    Jmp = 0b1111,
    // JNC Im
    // - 1110 Im
    Jnc = 0b1110,
}
