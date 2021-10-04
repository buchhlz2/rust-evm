// subset of opcodes -- note: #[derive(Debug)] allows enum to use std::fmt
#[derive(Debug)]
pub enum Opcode {
    STOP(usize), // 0x00
    ADD(usize), // 0x01
    MUL(usize), // 0x02
    // note: MSTORE operation will store as up to 256-bit words using U256 (must update Cargo to use)
    // MSTORE(U256), // 0x52
    // MSTORE8(u8), // 0x53
    // note: PUSH operation will push N 1 byte values to the stack (aka only 8 bytes at at time)
    PUSH1(usize, u8), // 0x60
    PUSH2(usize, u8, u8), // 0x61
    // PUSH32(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8), // 0x7f
    
    // fake opcode `UNKOWN` -- due to short opcode list
    UNKNOWN(usize),
    // fake opcode `END` used to terminate exeuction insetad of `STOP` -- due to shortened opcode list; to remove
    END,
}

// function to describe the outputs upon printing
impl Opcode {
    pub fn describe(&self) {
        match self {
            Opcode::STOP(line) => println!("0x{:x}\tSTOP\tHalts execution", line),
            Opcode::ADD(line) => println!("0x{:x}\tADD\tAddition operation", line),
            Opcode::MUL(line) => println!("0x{:x}\tMUL\tMultiplication operation", line),
            Opcode::PUSH1(line, x) => println!("0x{:x}\tPUSH1\tPlace 1-byte item on the stack 0x{:x}", line, x),
            Opcode::PUSH2(line, x0, x1) => println!("0x{:x}\tPUSH2\tPlace 2-bytes item on the stack 0x{:x} 0x{:x}", line, x0, x1),
            Opcode::UNKNOWN(line) => println!("0x{:x}\tUNKNOWN\tOpcode not found", line),
            _ => println!("Error")
        }
    }
}
