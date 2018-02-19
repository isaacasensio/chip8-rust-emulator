#[cfg(test)]
mod cpu_test {
    use ram::Ram;
    use cpu::{Cpu, START};

    fn write_operation_on_ram(ram: &mut Ram, address: u16, value: u16){
        let hi = (value >> 8) as u8;
        let lo = (value & 0x00FF) as u8;

        ram.write_bytes(address, lo);
        ram.write_bytes(address + 1, hi);
    }

    #[test]
    fn op_6xnn_assigns_nn_to_vx() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x6511);

        cpu.execute(ram);

        assert_eq!(cpu.read_vx(5), 0x11);
    }

    #[test]
    fn op_7xnn_adds_nn_to_vx() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x6511);
        write_operation_on_ram(ram, START + 2, 0x7511);

        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(5), 0x22);
    }

    #[test]
    fn op_5xy0_skip_instruction_if_vx_equals_vy() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x6511);
        write_operation_on_ram(ram, START + 2, 0x6611);
        write_operation_on_ram(ram, START + 4, 0x5560);
        write_operation_on_ram(ram, START + 8, 0x6766);

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(7), 0x66);
    }

    #[test]
    fn op_4xnn_skip_instruction_if_vx_not_equals_nn() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x6511);
        write_operation_on_ram(ram, START + 2, 0x4560);
        write_operation_on_ram(ram, START + 4, 0x6512);
        write_operation_on_ram(ram, START + 6, 0x6766);

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(5), 0x11);
        assert_eq!(cpu.read_vx(7), 0x66 );
    }

    #[test]
    fn op_3xnn_skip_instruction_if_vx_equals_nn() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x6511);
        write_operation_on_ram(ram, START + 2, 0x3511);
        write_operation_on_ram(ram, START + 6, 0x6766);

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(7), 0x66);
    }

    #[test]
    fn op_9xy0_skip_instruction_if_vx_not_equals_vy() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x6111);
        write_operation_on_ram(ram, START, 0x6222);
        write_operation_on_ram(ram, START + 2, 0x9120);
        write_operation_on_ram(ram, START + 6, 0x6766);

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(7), 0x66);
    }

    #[test]
    fn op_annn_sets_i_to_nnn() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0xA123);

        cpu.execute(ram);

        assert_eq!(cpu.read_i(), 0x123);
    }

    #[test]
    fn op_bnnn_jumps_to_nnn_plus_v0() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x6005); //v0 = 0x05
        write_operation_on_ram(ram, START + 2, 0xB001); //pc = v0 + 0x01 = 0x06
        write_operation_on_ram(ram, START + 6, 0x6110); //v1=0x010

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(1), 0x010);
    }

    #[test]
    fn op_8xy0_assigns_vx_to_vy() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x6105); //v1 = 0x05
        write_operation_on_ram(ram, START + 2, 0x8010); //v0 = v1

        cpu.execute(ram);

        assert_eq!(cpu.read_vx(0), 0x0);

        cpu.execute(ram);

        assert_eq!(cpu.read_vx(0), 0x05);
    }

    #[test]
    fn op_8xy1_bitwise_or_operation() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x60E5); //v0 = 0xe5
        write_operation_on_ram(ram, START + 2, 0x6116); //v1 = 0x16
        write_operation_on_ram(ram, START + 4, 0x8011);
        // V0=V0|V1
        // 11100101 E5
        // 00010110 16
        // --------
        // 11110111 F7

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(0), 0xF7);
    }

    #[test]
    fn op_8xy2_bitwise_and_operation() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x60E5); //v0 = 0xe5
        write_operation_on_ram(ram, START + 2, 0x6116); //v1 = 0x16
        write_operation_on_ram(ram, START + 4, 0x8012);
        // V0=V0|V1
        // 11100101 E5
        // 00010110 16
        // --------
        // 00000100 F7

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(0), 0x04);
    }

    #[test]
    fn op_8xy3_bitwise_xor_operation() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x60E5); //v0 = 0xe5
        write_operation_on_ram(ram, START + 2, 0x6116); //v1 = 0x16

        write_operation_on_ram(ram, START + 4, 0x8013);
        // V0=V0|V1
        // 11100101 E5
        // 00010110 16
        // --------
        // 11110011 F7

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(0), 0xF3);
    }

    #[test]
    fn op_8xy4_adds_vy_to_vx_without_carry() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x6001); //v0 = 0x01
        write_operation_on_ram(ram, START + 2, 0x6102); //v1 = 0x02
        write_operation_on_ram(ram, START + 4, 0x8014);

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(0), 0x03);
        assert_eq!(cpu.read_vx(0xF), 0x0);
    }

    #[test]
    fn op_8xy4_adds_vy_to_vx_with_carry() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x60FF); //v0 = 0xFF
        write_operation_on_ram(ram, START + 2, 0x61FF); //v1 = 0xFF
        write_operation_on_ram(ram, START + 4, 0x8014);

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(0), 0xFE);
        assert_eq!(cpu.read_vx(0xF), 0x1);
    }

    #[test]
    fn op_8xy5_subtracts_vy_from_vx_without_borrow() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x6002); //v0 = 0x02
        write_operation_on_ram(ram, START + 2, 0x6101); //v1 = 0x01
        write_operation_on_ram(ram, START + 4, 0x8015);

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(0), 0x01);
        assert_eq!(cpu.read_vx(0xF), 0x0);
    }

    #[test]
    fn op_8xy5_subtracts_vy_from_vx_with_borrow() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x6002); //v0 = 0x02
        write_operation_on_ram(ram, START + 2, 0x6103); //v1 = 0x03
        write_operation_on_ram(ram, START + 4, 0x8015);

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(0), 0xFF);
        assert_eq!(cpu.read_vx(0xF), 0x1);
    }

    #[test]
    fn op_8xy7_without_borrow() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x6002); //v0 = 0x02
        write_operation_on_ram(ram, START + 2, 0x6103); //v1 = 0x03
        write_operation_on_ram(ram, START + 4, 0x8017);

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(0), 0x1);
        assert_eq!(cpu.read_vx(0xF), 0x1);
    }

    #[test]
    fn op_8xy7_with_borrow() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x6005); //v0 = 0x05
        write_operation_on_ram(ram, START + 2, 0x6101); //v1 = 0x01
        write_operation_on_ram(ram, START + 4, 0x8017);

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(0), 0xFC);
        assert_eq!(cpu.read_vx(0xF), 0x0);
    }

    #[test]
    fn op_1nnn_go_to_nnn() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x6005); //v0 = 0x05
        write_operation_on_ram(ram, START + 2, 0x1022);
        write_operation_on_ram(ram, START + 0x22, 0x6015);

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(0x0), 0x15);
    }


    #[test]
    #[should_panic]
    fn unknown_operation_should_fail() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, START, 0x8AAA);
        cpu.execute(ram);
    }
}

