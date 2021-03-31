use super::Mapper;
use super::MapperError;

use crate::core::Byte;
use crate::core::Double;
use crate::core::consts;

pub struct NROMMapper {
    prg_ram_size: usize,
    prg_rom_size_8_kb: usize,
    prg_rom_content: Vec<Byte>,
    general_purpose_memory: Vec<Byte>
}

impl NROMMapper {
    pub fn new(prg_rom_content: &Vec<u8>, prg_ram_size: usize) -> NROMMapper {
        let mut prg_rom_size_8_kb: usize = 0;
        match prg_rom_content.len() {
            0x2000 => {
                prg_rom_size_8_kb = 1;
            },
            0x4000 => {
                prg_rom_size_8_kb = 2;
            },
            _ => {
                panic!("Invalid prg rom size");
            }
        }

        let mut prg_rom_content_byte: Vec<Byte> = Vec::<Byte>::new();
        for b in prg_rom_content {
            prg_rom_content_byte.push(Byte::new(*b));
        }

        let mut general_purpose_memory = Vec::<Byte>::new();
        for x in 0..consts::MEMORY_SIZE {
            general_purpose_memory.push(Byte::new(0x00));
        }

        NROMMapper{prg_ram_size: prg_ram_size, prg_rom_size_8_kb:prg_rom_size_8_kb, 
            prg_rom_content:prg_rom_content_byte, general_purpose_memory}
    }
}

impl Mapper for NROMMapper {
    fn get_memory_addr(&self, addr: Double) -> Result<Byte, MapperError> {
        match addr.get_value() {
            consts::NROM_PRG_RAM_RANGE_START..=consts::NROM_PRG_RAM_RANGE_END => {
                Ok(self.prg_rom_content[addr.get_value() as usize % self.prg_ram_size])
            },
            consts::NROM_FIRST_PRG_ROM_RANGE_START..=consts::NROM_FIRST_PRG_ROM_RANGE_END => {
                Ok(self.prg_rom_content[addr.get_value() as usize - consts::NROM_FIRST_PRG_ROM_RANGE_START as usize])
            },
            consts::NROM_SECOND_PRG_ROM_RANGE_START..=consts::NROM_SECOND_PRG_ROM_RANGE_END => {
                if self.prg_rom_size_8_kb == 2 {
                    Ok(self.prg_rom_content[addr.get_value() as usize - consts::NROM_SECOND_PRG_ROM_RANGE_START as usize])
                } else {
                    Ok(self.prg_rom_content[addr.get_value() as usize - consts::NROM_FIRST_PRG_ROM_RANGE_START as usize])
                }
            }
            _ => {
                Ok(self.general_purpose_memory[addr.get_value() as usize])
            }
        }
    }

    fn set_memory_addr(&mut self, addr: Double, value: Byte) -> Result<(), MapperError> {
        match addr.get_value() {
            consts::NROM_PRG_RAM_RANGE_START..=consts::NROM_PRG_RAM_RANGE_END => {
                self.prg_rom_content[addr.get_value() as usize % self.prg_ram_size] = value;
                Ok(())
            },
            consts::NROM_FIRST_PRG_ROM_RANGE_START..=consts::NROM_FIRST_PRG_ROM_RANGE_END => {
                self.prg_rom_content[addr.get_value() as usize - consts::NROM_FIRST_PRG_ROM_RANGE_START as usize] = value;
                Ok(())
            },
            consts::NROM_SECOND_PRG_ROM_RANGE_START..=consts::NROM_SECOND_PRG_ROM_RANGE_END => {
                if self.prg_rom_size_8_kb == 1 {
                    self.prg_rom_content[addr.get_value() as usize - consts::NROM_SECOND_PRG_ROM_RANGE_START as usize] = value;
                    Ok(())
                } else {
                    self.prg_rom_content[addr.get_value() as usize - consts::NROM_FIRST_PRG_ROM_RANGE_START as usize] = value;
                    Ok(())
                }
            }
            _ => {
                self.general_purpose_memory[addr.get_value() as usize] = value;
                Ok(())
            }
        }
    }
}