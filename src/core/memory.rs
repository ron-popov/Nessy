use super::byte::Byte;
use super::double::Double;

#[cfg(test)]
use super::consts;

use std::ops::{Index, IndexMut, Range};

#[derive(Clone)]
pub struct Memory {
    memory_map: Vec<Byte>
}

impl Memory {
    pub fn new(memory_size: usize) -> Memory {
        let mut mem = Memory{memory_map: Vec::<Byte>::new()};
        for _ in 0..memory_size {
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

impl Index<Double> for Memory {
    type Output = Byte;
    fn index<'a>(&'a self, i: Double) -> &'a Byte {
        &self.memory_map[i.get_value() as usize]
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

impl IndexMut<Double> for Memory {
    fn index_mut<'a>(&'a mut self, i: Double) -> &'a mut Byte {
        &mut self.memory_map[i.get_value() as usize]
    }
}

#[test]
fn memory_init() {
    let memory_size = consts::MEMORY_SIZE;
    let memory = Memory::new(memory_size);
    for i in 0..memory_size {
        assert_eq!(memory[i].get_value(), 0);
    }
}

#[test]
fn memory_change() {
    let memory_size = consts::MEMORY_SIZE;
    let mut memory = Memory::new(memory_size);
    
    memory[1 as usize] = 0x14.into();
    memory[2 as usize] = 0x15.into();
    memory[3 as usize] = 0x16.into();

    assert_eq!(memory[1 as usize].get_value(), 0x14);
    assert_eq!(memory[2 as usize].get_value(), 0x15);
    assert_eq!(memory[3 as usize].get_value(), 0x16);
}