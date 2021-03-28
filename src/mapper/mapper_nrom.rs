use super::Mapper;
use super::MapperError;

use crate::core::Byte;
use crate::core::Double;
use crate::core::consts;

pub struct NROMMapper {
    prg_ram_size: usize,
    prg_rom_size_8KB: usize,
    prg_rom_content: Vec<Byte>,
}

impl NROMMapper {
    pub fn new(prg_rom_content: &Vec<u8>, prg_ram_size: usize) -> NROMMapper {
        let mut prg_rom_size_8KB: usize = 0;
        match prg_rom_content.len() {
            0x2000 => {
                prg_rom_size_8KB = 1;
            },
            0x4000 => {
                prg_rom_size_8KB = 2;
            },
            _ => {
                panic!("Invalid prg rom size");
            }
        }

        let mut prg_rom_content_byte: Vec<Byte> = Vec::<Byte>::new();
        for b in prg_rom_content {
            prg_rom_content_byte.push(Byte::new(*b));
        }

        NROMMapper{prg_ram_size: prg_ram_size, prg_rom_size_8KB:prg_rom_size_8KB, prg_rom_content:prg_rom_content_byte}
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
                if self.prg_rom_size_8KB == 1 {
                    Ok(self.prg_rom_content[addr.get_value() as usize - consts::NROM_SECOND_PRG_ROM_RANGE_START as usize])
                } else {
                    Ok(self.prg_rom_content[addr.get_value() as usize - consts::NROM_FIRST_PRG_ROM_RANGE_START as usize])
                }
            }
            _ => {
                Err(MapperError::InvalidMemoryAddrRequseted(addr))
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
                if self.prg_rom_size_8KB == 1 {
                    self.prg_rom_content[addr.get_value() as usize - consts::NROM_SECOND_PRG_ROM_RANGE_START as usize] = value;
                    Ok(())
                } else {
                    self.prg_rom_content[addr.get_value() as usize - consts::NROM_FIRST_PRG_ROM_RANGE_START as usize] = value;
                    Ok(())
                }
            }
            _ => {
                Err(MapperError::InvalidMemoryAddrRequseted(addr))
            }
        }
    }
}