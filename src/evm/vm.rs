use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::num::ParseIntError;
use ethereum_types::U256;
use crate::evm::opcode::Opcode;

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

// rudimentary virtual machine
#[derive(Debug, Default)]
pub struct Vm {
    pub code: Vec<u8>, // smart contract
    pub pc: usize, // program counter with current instruction index of `code`
    pub stack: Vec<U256>
}

// initialize a new Vm
impl Vm {
    // factory function to instantiate new Vm from file
    pub fn new_from_file(filename: &str) -> Result<Vm, Box<dyn Error>> {
        // create buffer & read file to it
        let mut f = File::open(filename)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        // decode binaries from string into bytecode and init Vm
        let code = decode(&buffer)?;
        println!("{}", buffer);
        return Ok(Vm { code: code, pc: 0, stack: Vec::new() });
    }
}

// decode list of bytes into opcodes where `next` returns Opcode at current pc
impl Vm {
    // get current byte, match to opcode, optionally extract more data from stack, move pc, return Opcode
    pub fn next(&mut self) -> Opcode {
        // TODO remove once all opcodes implemented and `END` removed
        if self.pc >= self.code.len() {
            return Opcode::END;
        }

        let addr = self.pc;
        match self.code[addr] {
            0x00 => {
                self.pc += 1;
                Opcode::STOP(addr)
            },
            0x01 => {
                self.pc += 1;
                Opcode::ADD(addr)
            },
            0x02 => {
                self.pc += 1;
                Opcode::MUL(addr)
            },
            // 0x52 => {
            //     let offset = self.code[pc-2];
            //     let value = self.code[pc-1];
            //     self.pc += 1;
            //     Opcode::MSTORE(offset, value)
            // },
            // 0x53 => {
            //     let offset = self.code[pc-2];
            //     let value = self.code[pc-1];
            //     self.pc += 1;
            //     Opcode::MSTORE8(offset, value)
            // },
            0x60 => {
                let value = self.code[self.pc+1];
                println!("{}", value);
                self.pc += 2;
                Opcode::PUSH1(addr, value)
            },
            0x61 => {
                let value0 = self.code[self.pc+1];
                let value1 = self.code[self.pc+1];
                self.pc += 3;
                Opcode::PUSH2(addr, value0, value1)
            },
            // 0x7f => {
            //     let value0 = self.code[pc+1];  
            //     let value1 = self.code[pc+1];  
            //     self.pc += 1;
            //     Some(Opcode::PUSH32(value0, value1))
            // },
            _ => { self.pc += 1; Opcode::UNKNOWN(addr)}
        }
    }
}

impl Vm {
    pub fn interpret(&mut self) {

        let maybe_op = self.next();
        println!("interpret: {:?}",maybe_op);
        // debugging
        match &maybe_op {
            x => x.describe(),
        }

        match &maybe_op {
            x => {
                match x {
                    Opcode::PUSH1(addr, value) => {
                        self.stack.push(U256::from(*value));
                    },
                    Opcode::ADD(addr) => {
                        let v1 = self.stack.pop().unwrap();
                        let v2 = self.stack.pop().unwrap();
                        self.stack.push(v1 + v2);
                    },
                    _ => {

                    }
                }
            },

        }
    }
}

impl Vm {
    pub fn print_stack(&self) {
        self.stack
            .iter()
            .enumerate()
            .rev()
            .for_each(|(i,x)| {
                let mut bytes = vec![0;32];
                x.to_big_endian(&mut bytes);
                println!("|{}:\t{:?}|", i, bytes)
            });
    }
}
