#[cfg(test)]
mod instruction_test {

    use instruction::Instruction;

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