use super::errors::ParserError;

// Parsing is done with the help of this webpage : 
// http://wiki.nesdev.com/w/index.php/INES#:~:text=The%20.,for%20an%20emulator%20called%20iNES.

enum MirroringMode {
    Horizonal,
    Vertical,
    Invalid
}

pub struct InesRom {
    rom_content: Vec<u8>,
    prg_rom_size_16k: u8,
    chr_rom_size_8k: u8,
    mirroring_mode: MirroringMode,
    contains_prg_ram: bool,
    contains_trainer: bool,
    ignore_mirroring_control: bool,
    vs_unisystem: bool,
    playchoice_10: bool,
    is_nes2_format: bool,
}

impl InesRom {
    pub fn new(content: Vec<u8>) -> Result<InesRom, ParserError> {
        let mut parser = InesRom{rom_content: content, prg_rom_size_16k: 0, chr_rom_size_8k: 0, 
            mirroring_mode: MirroringMode::Invalid, contains_prg_ram: false, contains_trainer: false, 
            ignore_mirroring_control: false, vs_unisystem: false, playchoice_10: false, is_nes2_format: false};

        let header: Vec<u8> = parser.rom_content[0..0x10].to_vec();
        if header[0] != ('N' as u8) || header[1] != ('E' as u8) || header[2] != ('S' as u8) {
            log::error!("Invalid rom header");
            return Err(ParserError::InvalidRom);
        } else {
            log::trace!("Valid header found");
        }

        parser.prg_rom_size_16k = header[4];
        parser.chr_rom_size_8k = header[5];

        let mapper_lower_nibble: u8;
        let mapper_upper_nibble: u8;

        { // Flags 6 parsing
            let mut flags_6_byte = header[6];
            
            // Mirroring mode
            if flags_6_byte % 2 == 0 {
                parser.mirroring_mode = MirroringMode::Horizonal;
            } else {
                parser.mirroring_mode = MirroringMode::Vertical;
            }

            flags_6_byte /= 2;
            parser.contains_prg_ram = flags_6_byte % 2 == 1;

            flags_6_byte /= 2;
            parser.contains_trainer = flags_6_byte % 2 == 1;

            flags_6_byte /= 2;
            parser.ignore_mirroring_control = flags_6_byte % 2 == 1;

            flags_6_byte /= 2;

            mapper_lower_nibble = flags_6_byte;
        }

        { // Flags 7 parsing
            let mut flags_7_byte = header[7];

            parser.vs_unisystem = flags_7_byte % 2 == 1;

            flags_7_byte /= 2;
            parser.playchoice_10 = flags_7_byte % 2 == 1;

            flags_7_byte /= 2;
            parser.is_nes2_format = flags_7_byte % 2 == 1;

            flags_7_byte /= 2;
            mapper_upper_nibble = flags_7_byte;
        }

        // TODO : Parse the rest of the flags in the header

        Ok(parser)
    }
}