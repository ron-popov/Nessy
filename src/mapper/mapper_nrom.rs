use super::Mapper;
use super::MapperError;

use crate::core::Byte;
use crate::core::Double;
use crate::core::consts;

#[derive(PartialEq, Debug)]
pub enum NROMType {
    NROM128,
    NROM256,
}

#[derive(Debug)]
pub struct NROMMapper {
    prg_ram_size: usize,
    prg_rom_content: Vec<Byte>,
    general_purpose_memory: Vec<Byte>,
    nrom_type: NROMType,
}

impl NROMMapper {
    pub fn new(prg_rom_content: &Vec<u8>, prg_ram_size: usize) -> NROMMapper {
        let nrom_type: NROMType;
        match prg_rom_content.len() {
            0x4000 => {
                nrom_type = NROMType::NROM128;
                debug!("NROM Type is NROM-128");
            },
            0x8000 => {
                nrom_type = NROMType::NROM256;
                debug!("NROM Type is NROM-256");
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
        for _x in 0..consts::MEMORY_SIZE {
            general_purpose_memory.push(Byte::new(0x00));
        }

        NROMMapper{prg_ram_size: prg_ram_size, nrom_type: nrom_type, 
            prg_rom_content: prg_rom_content_byte, general_purpose_memory: general_purpose_memory}
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
                if self.nrom_type == NROMType::NROM128 {
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
                if self.nrom_type == NROMType::NROM128 {
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