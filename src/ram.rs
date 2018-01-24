pub struct Ram {
    main: [u8; 3584],
    sprites: [[u8; 5]; 16],
}

const MEMORY_SIZE : usize = 3584;

impl Ram {
    pub fn new() -> Ram {
        let mut memory = Ram {
            main: [0; MEMORY_SIZE],
            sprites: [[0; 5]; 16],
        };

        memory.load_sprites();
        return memory;
    }

    pub fn read_bytes(&self, address: u16) -> u8{
        return self.main[address as usize];
    }

    pub fn write_bytes(&mut self, address: u16, value: u8){
        self.main[address as usize] = value;
    }

    fn load_sprites(&mut self) {
        self.sprites = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ]
    }
}

#[cfg(test)]
mod ram_test {

    use ram::Ram;
    use ram::MEMORY_SIZE;

    #[test]
    fn starts_with_empty_memory() {
        let ram = Ram::new();
        for (_, address) in (0..MEMORY_SIZE).enumerate() {
            let bytes = ram.read_bytes(address as u16);
            assert_eq!(bytes, 0 as u8);
        }
    }

    #[test]
    fn writes_value_in_memory() {
        let mut ram = Ram::new();
        ram.write_bytes(0, 1);
        let bytes = ram.read_bytes(0);
        assert_eq!(bytes, 1);

        for (_, address) in (1..MEMORY_SIZE).enumerate() {
            let bytes = ram.read_bytes(address as u16);
            assert_eq!(bytes, 0 as u8);
        }
    }
}