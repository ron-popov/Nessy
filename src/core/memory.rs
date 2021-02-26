use super::byte::Byte;
use super::consts;

use std::ops::{Index, IndexMut};

pub struct Memory {
    memory_map: [Byte; consts::MEMORY_SIZE]
}

impl Memory {
    pub fn new() -> Memory {
        Memory{memory_map: [Byte::new(0); consts::MEMORY_SIZE]}
    }
}


impl Index<usize> for Memory {
    type Output = Byte;
    fn index<'a>(&'a self, i: usize) -> &'a Byte {
        &self.memory_map[i]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Byte {
        &mut self.memory_map[i]
    }
}


#[test]
fn memory_init() {
    let memory = Memory::new();
    for i in 0..consts::MEMORY_SIZE {
        assert_eq!(memory[i].get_value(), 0);
    }
}

#[test]
fn memory_change() {
    let mut memory = Memory::new();
    
    memory[1] = 0x14.into();
    memory[2] = 0x15.into();
    memory[3] = 0x16.into();

    assert_eq!(memory[1].get_value(), 0x14);
    assert_eq!(memory[2].get_value(), 0x15);
    assert_eq!(memory[3].get_value(), 0x16);
}