use ram::Ram;
use instruction::Instruction;

const START: u16 = 0x200;

pub struct Cpu {
    pc: u16,
    reg_vx: [u8; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: START,
            reg_vx: [0; 16],
        }
    }

    fn write_on_vx(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        self.reg_vx[x] = instruction.nn();
    }

    fn skip_on_vx_equal_vy(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        let y = instruction.y() as usize;
        if self.reg_vx[x] == self.reg_vx[y] {
            self.pc += 2;
        }
    }

    fn skip_on_vx_not_equal_nn(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        if self.reg_vx[x] != instruction.nn() {
            self.pc += 2;
        }
    }

    fn add_on_vx(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        self.reg_vx[x] = self.reg_vx[x] + instruction.nn();
    }

    pub fn read_on_vx(&mut self, x: usize) -> u8 {
        return self.reg_vx[x];
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

        match op {
            4 => self.skip_on_vx_not_equal_nn(instruction),
            5 => self.skip_on_vx_equal_vy(instruction),
            6 => self.write_on_vx(instruction),
            7 => self.add_on_vx(instruction),
            _ => println!("woooo")
        }

        self.pc = self.pc + 2
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

        assert_eq!(cpu.read_on_vx(5), 0x11);
    }

    #[test]
    fn op_7xnn_adds_nn_to_vx() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, super::START, 0x6511);
        write_operation_on_ram(ram, super::START + 2, 0x7511);

        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_on_vx(5), 0x22);
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

        assert_eq!(cpu.read_on_vx(7), 0x66);
    }

    #[test]
    fn op_4xnn_skip_instruction_if_vx_not_equals_nn() {
        let mut cpu = Cpu::new();
        let ram = &mut Ram::new();

        write_operation_on_ram(ram, super::START, 0x6511);

        write_operation_on_ram(ram, super::START + 2, 0x4560);

        write_operation_on_ram(ram, super::START + 6, 0x6766);

        cpu.execute(ram);
        cpu.execute(ram);
        cpu.execute(ram);

        assert_eq!(cpu.read_on_vx(7), 0x66);
    }
}

