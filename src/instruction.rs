
pub struct Instruction {
    raw: u16
}

impl Instruction {
    pub fn new(raw: u16) -> Instruction {
        Instruction {
            raw
        }
    }

    pub fn raw(self) -> u16{
        self.raw as u16
    }

    pub fn op(&self) -> u8{
        ((self.raw & 0xF000) >> 12) as u8
    }

    pub fn x(&self) -> u8{
        ((self.raw & 0x0F00) >> 8) as u8
    }

    pub fn y(&self) -> u8{
        ((self.raw & 0x00F0) >> 4) as u8
    }

    pub fn nnn(&self) -> u16{
        (self.raw & 0x0FFF) as u16
    }

    pub fn nn(&self) -> u8{
        (self.raw & 0x00FF) as u8
    }

    pub fn n(&self) -> u16{
        (self.raw & 0x000F) as u16
    }
}