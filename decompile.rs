use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::error::Error;
use std::num::ParseIntError;
// use primitive_types::U256;

// started via tutorial from: https://snoozetime.github.io/2018/11/09/ethereum-vm-1.html

// convert binary string (via compiled contract) to a list of u8 bytes
// info on Result return: https://doc.rust-lang.org/std/result/enum.Result.html#method.from_iter
fn decode(s: &str) -> Result<Vec<u8>, ParseIntError> {
    // create map from 0 to length s; create another iterator that increments by 2 & slice s by hex bytes
    // usage of collect due to lazy rust: https://doc.rust-lang.org/book/second-edition/ch13-02-iterators.html
    return (0..(s.len()-1))
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i+2], 16))
        .collect();
}

// subset of opcodes -- note: #[derive(Debug)] allows enum to use std::fmt
#[derive(Debug)]
enum Opcode {
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

    // fake opcode `END` used to terminate exeuction insetad of `STOP` -- due to shortened opcode list; to remove
    END,
}

// function to describe the outputs upon printing
impl Opcode {
    fn describe(&self) {
        match self {
            Opcode::STOP(line) => println!("0x{:x}\tSTOP\tHalts execution", line),
            Opcode::ADD(line) => println!("0x{:x}\tADD\tAddition operation", line),
            Opcode::MUL(line) => println!("0x{:x}\tMUL\tMultiplication operation", line),
            Opcode::PUSH1(line, x) => println!("0x{:x}\tPUSH1\tPlace 1-byte item on the stack 0x{:x}", line, x),
            Opcode::PUSH2(line, x0, x1) => println!("0x{:x}\tPUSH2\tPlace 2-bytes item on the stack 0x{:x} 0x{:x}", line, x0, x1),
            _ => println!("Unknown opcode")
        }
    }
}

// rudimentary virtual machine
#[derive(Debug, Default)]
struct Vm {
    code: Vec<u8>, // opcode
    pc: usize, // program counter with current instruction index of `code`
}

// initialize a new Vm
impl Vm {
    fn new_from_file(filename: &str) -> Result<Vm, Box<dyn Error>> {
        // create buffer & read file to it
        let mut f = File::open(filename)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        // decode binaries from string into bytecode
        let code = decode(&buffer)?;
        return Ok(Vm{code: code, pc: 0});
    }
}

// decode list of bytes into opcodes where `next` returns Opcode at current pc
impl Vm {
    // get current byte, match to opcode, optionally extract more data from stack, move pc, return Opcode
    fn next(&mut self) -> Option<Opcode> {
        // TODO remove once all opcodes implemented and `END` removed
        if self.pc >= self.code.len() {
            return Some(Opcode::END);
        }

        let addr = self.pc;
        match self.code[addr] {
            0x00 => {
                self.pc += 1;
                Some(Opcode::STOP(addr))
            },
            0x01 => {
                self.pc += 1;
                Some(Opcode::ADD(addr))
            },
            0x02 => {
                self.pc += 1;
                Some(Opcode::MUL(addr))
            },
            // 0x52 => {
            //     let offset = self.code[pc-2];
            //     let value = self.code[pc-1];
            //     self.pc += 1;
            //     Some(Opcode::MSTORE(offset, value))
            // },
            // 0x53 => {
            //     let offset = self.code[pc-2];
            //     let value = self.code[pc-1];
            //     self.pc += 1;
            //     Some(Opcode::MSTORE8(offset, value))
            // },
            0x60 => {
                let value = self.code[self.pc+1];
                self.pc += 2;
                Some(Opcode::PUSH1(addr, value))
            },
            0x61 => {
                let value0 = self.code[self.pc+1];
                let value1 = self.code[self.pc+1];
                self.pc += 3;
                Some(Opcode::PUSH2(addr, value0, value1))
            },
            // 0x7f => {
            //     let value0 = self.code[pc+1];  
            //     let value1 = self.code[pc+1];  
            //     self.pc += 1;
            //     Some(Opcode::PUSH32(value0, value1))
            // },
            _ => { self.pc += 1; None}
        }
    }
}

// read data from input of a compiled contract's binary representation file
fn run() -> Result<(), Box<dyn Error>> {
    // get command line inputs where a single compiled contract binary file is of args[1]
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "No file passed as argument for decompilation");

    // set file to decode
    let binary_filename = &args[1];
    println!("File to decompile: {:?}", binary_filename);

    let mut vm = Vm::new_from_file(&binary_filename).unwrap();

    // print out opcodes & relative meaning
    loop {
        match vm.next() {
            // TODO remove END once all opcodes implemented
            Some(Opcode::END) => {
                println!("END");
                break;
            },
            Some(x) => x.describe(),
            None => {}
        }
    }

    Ok(())
}

fn main() {
    run().unwrap();
}