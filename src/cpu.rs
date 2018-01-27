use ram::Ram;
use instruction::Instruction;

const START: u16 = 0x200;

pub struct Cpu {
    pc: u16,
    i: u16,
    reg_vx: [u8; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: START,
            i: 0,
            reg_vx: [0; 16],
        }
    }

    fn write_on_vx(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        self.reg_vx[x] = instruction.nn();
        self.pc += 2;
    }

    fn skip_on_vx_equal_vy(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        let y = instruction.y() as usize;
        if self.reg_vx[x] == self.reg_vx[y] {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn skip_on_vx_not_equal_vy(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        let y = instruction.y() as usize;
        if self.reg_vx[x] != self.reg_vx[y] {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn skip_on_vx_not_equal_nn(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        if self.reg_vx[x] != instruction.nn() {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn skip_on_vx_equal_nn(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        if self.reg_vx[x] == instruction.nn() {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn add_on_vx(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        self.reg_vx[x] += instruction.nn();
        self.pc += 2;
    }

    fn assign_vx_to_vy(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        let y = instruction.y() as usize;
        self.reg_vx[x] = self.reg_vx[y];
        self.pc += 2;
    }

    fn bitwise_or(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        let y = instruction.y() as usize;
        self.reg_vx[x] |= self.reg_vx[y];
        self.pc += 2;
    }

    fn bitwise_and(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        let y = instruction.y() as usize;
        self.reg_vx[x] &= self.reg_vx[y];
        self.pc += 2;
    }

    fn bitwise_xor(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        let y = instruction.y() as usize;
        self.reg_vx[x] ^= self.reg_vx[y];
        self.pc += 2;
    }

    fn jump_to_address_nnn_plus_v0(&mut self, instruction: &Instruction) {
        self.pc = START + instruction.nnn() + self.reg_vx[0] as u16;
    }

    pub fn read_vx(&mut self, x: usize) -> u8 {
        return self.reg_vx[x];
    }

    pub fn read_i(&mut self) -> u16 {
        return self.i;
    }

    pub fn write_i(&mut self, instruction: &Instruction) {
        self.i = instruction.nnn();
    }

    pub fn execute(&mut self, ram: &mut Ram) {
        let lo = ram.read_bytes(self.pc) as u16;
        let hi = ram.read_bytes(self.pc + 1) as u16;
        let raw: u16 = (hi << 8) | lo;
        let instruction = &mut Instruction::new(raw);
        let op = instruction.op();

        println!("op: {}", op);
        println!("raw: {}", raw);
        println!("pc: {}", self.pc);

        match (op, instruction.n()) {
            (0x3, _) => self.skip_on_vx_equal_nn(instruction),
            (0x4, _) => self.skip_on_vx_not_equal_nn(instruction),
            (0x5, _) => self.skip_on_vx_equal_vy(instruction),
            (0x6, _) => self.write_on_vx(instruction),
            (0x7, _) => self.add_on_vx(instruction),
            (0x8, 0x0) => self.assign_vx_to_vy(instruction),
            (0x8, 0x1) => self.bitwise_or(instruction),
            (0x8, 0x2) => self.bitwise_and(instruction),
            (0x8, 0x3) => self.bitwise_xor(instruction),
            (0x9, 0x0) => self.skip_on_vx_not_equal_vy(instruction),
            (0xA, _) => self.write_i(instruction),
            (0xB, _) => self.jump_to_address_nnn_plus_v0(instruction),
            _ => panic!("Unknown instruction {}", raw)
        }

    }
}


#[cfg(test)]
mod executor_test {
    use super::Ram;
    use super::Cpu;

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

        write_operation_on_ram(ram, super::START, 0x6511);

        cpu.execute(ram);

        assert_eq!(cpu.read_vx(5), 0x11);
    }

    #[test]
    fn op_7xnn_adds_nn_to_vx() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, super::START, 0x6511);
        write_operation_on_ram(ram, super::START + 2, 0x7511);

        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(5), 0x22);
    }

    #[test]
    fn op_5xy0_skip_instruction_if_vx_equals_vy() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, super::START, 0x6511);

        write_operation_on_ram(ram, super::START + 2, 0x6611);

        write_operation_on_ram(ram, super::START + 4, 0x5560);

        write_operation_on_ram(ram, super::START + 8, 0x6766);

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

        write_operation_on_ram(ram, super::START, 0x6511);

        write_operation_on_ram(ram, super::START + 2, 0x4560);

        write_operation_on_ram(ram, super::START + 4, 0x6512);

        write_operation_on_ram(ram, super::START + 6, 0x6766);

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

        write_operation_on_ram(ram, super::START, 0x6511);

        write_operation_on_ram(ram, super::START + 2, 0x3511);

        write_operation_on_ram(ram, super::START + 6, 0x6766);

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(7), 0x66);
    }

    #[test]
    fn op_9xy0_skip_instruction_if_vx_not_equals_vy() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, super::START, 0x6111);

        write_operation_on_ram(ram, super::START, 0x6222);

        write_operation_on_ram(ram, super::START + 2, 0x9120);

        write_operation_on_ram(ram, super::START + 6, 0x6766);

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(7), 0x66);
    }

    #[test]
    fn op_annn_sets_i_to_nnn() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, super::START, 0xA123);

        cpu.execute(ram);

        assert_eq!(cpu.read_i(), 0x123);
    }

    #[test]
    fn op_bnnn_jumps_to_nnn_plus_v0() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, super::START, 0x6005); //v0 = 0x05
        write_operation_on_ram(ram, super::START + 2, 0xB001); //pc = v0 + 0x01 = 0x06
        write_operation_on_ram(ram, super::START + 6, 0x6110); //v1=0x010

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_vx(1), 0x010);
    }

    #[test]
    fn op_8xy0_assigns_vx_to_vy() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, super::START, 0x6105); //v1 = 0x05

        write_operation_on_ram(ram, super::START + 2, 0x8010); //v0 = v1

        cpu.execute(ram);

        assert_eq!(cpu.read_vx(0), 0x0);

        cpu.execute(ram);

        assert_eq!(cpu.read_vx(0), 0x05);
    }

    #[test]
    fn op_8xy1_bitwise_or_operation() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, super::START, 0x60E5); //v0 = 0xe5
        write_operation_on_ram(ram, super::START + 2, 0x6116); //v1 = 0x16

        write_operation_on_ram(ram, super::START + 4, 0x8011);
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

        write_operation_on_ram(ram, super::START, 0x60E5); //v0 = 0xe5
        write_operation_on_ram(ram, super::START + 2, 0x6116); //v1 = 0x16

        write_operation_on_ram(ram, super::START + 4, 0x8012);
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

        write_operation_on_ram(ram, super::START, 0x60E5); //v0 = 0xe5
        write_operation_on_ram(ram, super::START + 2, 0x6116); //v1 = 0x16

        write_operation_on_ram(ram, super::START + 4, 0x8013);
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
    #[should_panic]
    fn unknown_operation_should_fail() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, super::START, 0x8AAA);
        cpu.execute(ram);
    }
}

