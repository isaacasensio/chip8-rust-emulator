
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

#[cfg(test)]
mod instruction_test {

    use super::Instruction;

    #[test]
    fn op_returns_first_4_bits() {

        for multiplier in 0..16u8 {
            let next = 0x0000u16 + (0x1000u16 * multiplier as u16);
            let instruction = Instruction::new(next);
            assert_eq!(instruction.op(), multiplier);
        }
    }

    #[test]
    fn x_registry_is_always_the_second_group_of_4_bits() {
        for (_, value) in (0x0000u16..0xFFFFu16).enumerate() {
            let instruction = Instruction::new(value);
            let hex = format!("{:04X}", value).chars().nth(1).unwrap().to_string();
            let byte = u8::from_str_radix(&hex, 16).unwrap();
            assert_eq!(instruction.x(), byte);
        }
    }

    #[test]
    fn y_registry_is_always_the_third_group_of_4_bits() {
        for (_, value) in (0x0000u16..0xFFFFu16).enumerate() {
            let instruction = Instruction::new(value);
            let hex = format!("{:04X}", value).chars().nth(2).unwrap().to_string();
            let byte = u8::from_str_radix(&hex, 16).unwrap();
            assert_eq!(instruction.y(), byte);
        }
    }

    #[test]
    fn nnn_registry_is_always_the_last_12_bits() {
        for (_, value) in (0x0000u16..0xFFFFu16).enumerate() {
            let instruction = Instruction::new(value);
            let hex : String = format!("{:04X}", value).chars().skip(1).collect();
            let bytes = u16::from_str_radix(&hex, 16).unwrap();
            assert_eq!(instruction.nnn(), bytes);
        }
    }

    #[test]
    fn nn_registry_is_always_the_last_byte() {
        for (_, value) in (0x0000u16..0xFFFFu16).enumerate() {
            let instruction = Instruction::new(value);
            let hex : String = format!("{:04X}", value).chars().skip(2).collect();
            let bytes = u8::from_str_radix(&hex, 16).unwrap();
            assert_eq!(instruction.nn(), bytes);
        }
    }

    #[test]
    fn n_registry_is_always_the_last_4_bits() {
        for (_, value) in (0x0000u16..0xFFFFu16).enumerate() {
            let instruction = Instruction::new(value);
            let hex : String = format!("{:04X}", value).chars().skip(3).collect();
            let bytes = u16::from_str_radix(&hex, 16).unwrap();
            assert_eq!(instruction.n(), bytes);
        }
    }
}
