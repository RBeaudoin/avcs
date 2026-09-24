/// An error encountered when creating a cartridge.
#[derive(Debug, PartialEq, Eq)]
pub enum CartridgeError {
    /// The supplied ROM contains no bytes.
    EmptyRom,
}

/// An Atari 2600 cartridge.
pub struct Cartridge {
    rom: Vec<u8>,
}

impl Cartridge {
    /// Creates a cartridge from ROM bytes.
    ///
    /// Takes ownership of the bytes.
    ///
    /// # Errors
    ///
    /// Returns `CartridgeError::EmptyRom` if the supplied rom is zero bytes.
    pub fn new(rom: Vec<u8>) -> Result<Self, CartridgeError> {
        if rom.is_empty() {
            return Err(CartridgeError::EmptyRom);
        }
        Ok(Self { rom })
    }

    /// Returns the size of the ROM image in bytes.
    pub fn size_bytes(&self) -> usize {
        self.rom.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_rom_bytes_and_reports_size() {
        let bytes = vec![0xA9, 0x00, 0x85, 0x80];

        let cartridge = Cartridge::new(bytes).expect("nonempty rom should be accepted");

        assert_eq!(cartridge.rom, vec![0xA9, 0x00, 0x85, 0x80]);
        assert_eq!(cartridge.size_bytes(), 4);
    }

    #[test]
    fn returns_error_for_empty_rom() {
        let result = Cartridge::new(Vec::new());

        assert!(matches!(result, Err(CartridgeError::EmptyRom)));
    }
}
