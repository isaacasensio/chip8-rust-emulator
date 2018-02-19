#[cfg(test)]
mod ram_test {

    use ram::{ Ram, MEMORY_SIZE };

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