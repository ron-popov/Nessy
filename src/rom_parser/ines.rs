use super::ParserError;

// Parsing is done with the help of this webpage : 
// http://wiki.nesdev.com/w/index.php/INES#:~:text=The%20.,for%20an%20emulator%20called%20iNES.

use std::fmt;

use crate::core::consts;
use crate::mapper::{Mapper, NROMMapper};

#[derive(Debug)]
enum MirroringMode {
    Horizonal,
    Vertical,
    Invalid
}

#[derive(Debug)]
enum TVSystem {
    Ntsc,
    Pal,
    DualCompatible,
    Invalid
}

pub struct InesRom {
    rom_content: Vec<u8>,
    prg_rom_content: Vec<u8>,
    chr_rom_content: Vec<u8>,
    trainer_content: Vec<u8>,
    prg_rom_size: usize,
    prg_ram_size: usize,
    chr_rom_size: usize,
    use_chr_ram: bool,
    mirroring_mode: MirroringMode,
    contains_prg_ram: bool,
    contains_trainer: bool,
    ignore_mirroring_control: bool,
    vs_unisystem: bool,
    playchoice_10: bool,
    is_nes2_format: bool,
    tv_system: TVSystem,
    mapper: u8,
}

impl fmt::Debug for InesRom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("INES Rom")
            .field("TV System", &self.tv_system)
            .field("Mirroring Mode", &self.mirroring_mode)
            .field("Rom Size", &self.rom_content.len())
            .field("PRG Rom Size", &(self.prg_rom_size))
            .field("PRG Ram Size", &(self.prg_ram_size))
            .field("CHR Rom Size", &(self.chr_rom_size))
            .field("Contains PRG Ram", &(self.contains_prg_ram))
            .field("Contains Trainer", &(self.contains_trainer))
            .field("Is Nes2", &(self.is_nes2_format))
            .field("Mapper ID", &(self.mapper))
            .finish()
    }
}

impl InesRom {
    pub fn new(content: Vec<u8>) -> Result<InesRom, ParserError> {
        let mut rom = InesRom{rom_content: content, prg_rom_size: 0, chr_rom_size: 0, 
            mirroring_mode: MirroringMode::Invalid, contains_prg_ram: false, contains_trainer: false, 
            ignore_mirroring_control: false, vs_unisystem: false, playchoice_10: false, is_nes2_format: false,
            prg_ram_size: 0, tv_system: TVSystem::Invalid, mapper: 0, use_chr_ram: false, prg_rom_content: Vec::new(),
            chr_rom_content: Vec::new(), trainer_content: Vec::new()};

            
        let header: Vec<u8> = rom.rom_content[0..0x10].to_vec();
        if header[0] != ('N' as u8) || header[1] != ('E' as u8) || header[2] != ('S' as u8) {
            log::error!("Invalid rom header");
            return Err(ParserError::InvalidRom);
        } else {
            log::debug!("Valid INES header found");
        }
            
        rom.prg_rom_size = header[4] as usize * 0x4000;
        rom.chr_rom_size = header[5] as usize * 0x2000;

        if rom.chr_rom_size == 0 {
            rom.use_chr_ram = true;
        }

        let mapper_lower_nibble: u8;
        let mapper_upper_nibble: u8;

        { // Flags 6 parsing
            let mut flags_6_byte = header[6];
            
            // Mirroring mode
            if flags_6_byte % 2 == 0 {
                rom.mirroring_mode = MirroringMode::Horizonal;
            } else {
                rom.mirroring_mode = MirroringMode::Vertical;
            }

            flags_6_byte /= 2;
            rom.contains_prg_ram = flags_6_byte % 2 == 1;

            flags_6_byte /= 2;
            rom.contains_trainer = flags_6_byte % 2 == 1;

            flags_6_byte /= 2;
            rom.ignore_mirroring_control = flags_6_byte % 2 == 1;

            flags_6_byte /= 2;

            mapper_lower_nibble = flags_6_byte;
        }

        { // Flags 7 parsing
            let mut flags_7_byte = header[7];

            rom.vs_unisystem = flags_7_byte % 2 == 1;

            flags_7_byte /= 2;
            rom.playchoice_10 = flags_7_byte % 2 == 1;

            flags_7_byte /= 2;
            rom.is_nes2_format = flags_7_byte % 2 == 1;

            flags_7_byte /= 2;
            mapper_upper_nibble = flags_7_byte;
        }

        rom.mapper = mapper_lower_nibble + mapper_upper_nibble * 0x10;

        { // Flags 8 parsing
            rom.prg_ram_size = header[8] as usize * 0x2000;
            if rom.prg_ram_size == 0 { // Due to compatability, 0 means 8KB of ram
                rom.prg_ram_size = 0x2000;
            }
        }

        { // Flags 9 parsing
            if header[9] % 2 == 0 {
                rom.tv_system = TVSystem::Ntsc;
            } else {
                rom.tv_system = TVSystem::Pal;
            }
        }

        // NOTE : By the docs usually byte 10 is not implemented

        // Parse content
        log::debug!("Parsing INES Rom content");
        let mut rom_index = 0x10;

        if rom.contains_trainer {
            rom.trainer_content = rom.rom_content[rom_index..rom_index + 0x200].to_vec();
            rom_index += 0x200;
        }

        rom.prg_rom_content = rom.rom_content[rom_index..rom_index + rom.prg_rom_size as usize].to_vec();
        log::trace!("First bytes of prg rom : {:X?}", rom.prg_rom_content[0..3].to_vec());

        log::debug!("INES Parser : {:?}", rom);

        Ok(rom)
    }

    pub fn get_mapper(&self) -> Result<Box<dyn Mapper>, ParserError> {
        match self.mapper {
            consts::NROM_MAPPER_ID => {
                let mapper_struct = Box::new(NROMMapper::new(&self.prg_rom_content, self.prg_ram_size));
                Ok(mapper_struct)
            },
            _ => {
                Err(ParserError::UnknownMapperID(self.mapper))
            }
        }
    }
}
