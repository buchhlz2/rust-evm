
/* memory layout
0x00 - 0x3f: scratch space for hashing methods
0x40 - 0x5f: currently allocated memory size (aka. free memory pointer)
0x60 - 0x7f: zero slot
*/

use ethereum_types::U256;

#[derive(Debug, Default)]
pub struct Memory {
    data: Vec<u8>
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: Vec::new() }
    }

    pub fn resize(&mut self, new_size: usize) {
        if self.data.len() < new_size {
            self.data.resize(new_size, 0)
        }
    }

    pub fn get_word(&self, addr: usize) -> U256 {
        U256::from_big_endian(&self.data[addr..addr+32])
    }

    pub fn set_byte(&mut self, addr: usize, b: u8) {
        self.data[addr] = b;
    }

    pub fn set_word(&mut self, addr: usize, w: U256) {
        let mut bytes = vec![0; 32];
        w.to_big_endian(&mut bytes);

        for i in 0..bytes.len() {
            self.data[i+addr] = bytes[i];
        }
    }
}