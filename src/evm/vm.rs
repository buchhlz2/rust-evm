use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::num::ParseIntError;
use ethereum_types::U256;
use crate::evm::opcode::Opcode;
use crate::evm::memory::Memory;

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
    pub stack: Vec<U256>,
    pub mem: Memory
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
        return Ok(Vm { code: code, pc: 0, stack: Vec::new(), mem: Memory::new() });
    }

    // decode list of bytes into opcodes where `next` returns Opcode at current pc
    // get current byte, match to opcode, optionally extract more data from stack, move pc, return Opcode
    pub fn next(&mut self) -> Opcode {
        // TODO remove once all opcodes implemented and `END` removed
        if self.pc >= self.code.len() {
            return Opcode::END;
        }

        let _addr = self.pc;
        match self.code[_addr] {
            0x00 => {
                self.pc += 1;
                Opcode::STOP(_addr)
            },
            0x01 => {
                self.pc += 1;
                Opcode::ADD(_addr)
            },
            0x02 => {
                self.pc += 1;
                Opcode::MUL(_addr)
            },
            0x51 => {
                self.pc += 1;
                Opcode::MLOAD(_addr)
            },
            0x52 => {
                self.pc += 1;
                Opcode::MSTORE(_addr)
            },
            0x53 => {
                self.pc += 1;
                Opcode::MSTORE8(_addr)
            },
            0x60 => {
                let value = self.extract_u256(1);
                println!("{}", value);
                self.pc += 2;
                Opcode::PUSH1(_addr, value)
            },
            0x61 => {
                let value = self.extract_u256(2);
                self.pc += 3;
                Opcode::PUSH2(_addr, value)
            },
            0x73 => {
                let value = self.extract_u256(32);
                self.pc += 33;
                Opcode::PUSH32(_addr, value)
            },
            // 0x7f => {
            //     let value0 = self.code[pc+1];  
            //     let value1 = self.code[pc+1];  
            //     self.pc += 1;
            //     Some(Opcode::PUSH32(value0, value1))
            // },
            _ => { self.pc += 1; Opcode::UNKNOWN(_addr)}
        }
    }

    pub fn interpret(&mut self) {

        let maybe_op = self.next();
        println!("interpret: {:?}",maybe_op);
        // debugging
        match &maybe_op {
            x => x.describe(),
        }

        match self.get_new_size(&maybe_op) {
            Some(n) => self.mem.resize(n),
            _ => {}
        }

        match &maybe_op {
            x => {
                match x {
                    Opcode::PUSH1(_addr, value) => {
                        self.stack.push(U256::from(*value));
                    },
                    Opcode::ADD(_addr) => {
                        let v1 = self.stack.pop().unwrap();
                        let v2 = self.stack.pop().unwrap();
                        self.stack.push(v1 + v2);
                    },
                    Opcode::MLOAD(_addr) => {
                        let offset = self.stack.pop().unwrap();
                        let loaded_value = self.mem.get_word(offset.as_u64() as usize);
                        self.stack.push(loaded_value);
                    },
                    Opcode::MSTORE(_addr) => {
                        let offset = self.stack.pop().unwrap();
                        let w = self.stack.pop().unwrap();
                        self.mem.set_word(offset.as_u64() as usize, w);
                    },
                    Opcode::MSTORE8(_addr) => {
                        // stored as big endian so we get the last byte
                        let offset = self.stack.pop().unwrap();
                        let b = self.stack.pop().unwrap().byte(31);
                        self.mem.set_byte(offset.as_u64() as usize, b);
                    },
                    _ => {

                    }
                }
            },

        }
    }

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

    pub fn extract_u256(&mut self, to_extract: usize) -> U256 {
        let mut bytes = vec![0;32];
        for i in  0..to_extract {
            let value = self.code[self.pc+1];
            bytes[32-to_extract+i] = value;
        }

        U256::from_big_endian(&bytes)
    }

    pub fn get_new_size(&self, code: &Opcode) -> Option<usize> {
        match code {
            Opcode::MLOAD(_) | Opcode::MSTORE(_) => {
                Some(self.stack.last().unwrap().as_u64() as usize + 32)
            },
            Opcode::MSTORE8(_) => {
                Some(self.stack.last().unwrap().as_u64() as usize + 1)
            },
            _ => None  
        }
    }
}
