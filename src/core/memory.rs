use super::byte::Byte;
use super::consts;

use std::ops::{Index, IndexMut, Range};

#[derive(Clone)]
pub struct Memory {
    memory_map: Vec<Byte>
}

impl Memory {
    pub fn new() -> Memory {
        let mut mem = Memory{memory_map: Vec::<Byte>::new()};
        for _ in 0..consts::MEMORY_SIZE {
            mem.memory_map.push(Byte::new(0x00));
        }
        
        mem
    }
}


impl Index<usize> for Memory {
    type Output = Byte;
    fn index<'a>(&'a self, i: usize) -> &'a Byte {
        &self.memory_map[i]
    }
}

impl Index<u16> for Memory {
    type Output = Byte;
    fn index<'a>(&'a self, i: u16) -> &'a Byte {
        &self.memory_map[i as usize]
    }
}

impl Index<Byte> for Memory {
    type Output = Byte;
    fn index<'a>(&'a self, i: Byte) -> &'a Byte {
        &self.memory_map[i.get_value() as usize]
    }
}

impl Index<Range<usize>> for Memory {
    type Output = [Byte];
    fn index<'a>(&'a self, r: Range<usize>) -> &'a [Byte] {
        &self.memory_map[r.start..r.end]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Byte {
        &mut self.memory_map[i]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut<'a>(&'a mut self, i: u16) -> &'a mut Byte {
        &mut self.memory_map[i as usize]
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
    
    memory[1 as usize] = 0x14.into();
    memory[2 as usize] = 0x15.into();
    memory[3 as usize] = 0x16.into();

    assert_eq!(memory[1 as usize].get_value(), 0x14);
    assert_eq!(memory[2 as usize].get_value(), 0x15);
    assert_eq!(memory[3 as usize].get_value(), 0x16);
}