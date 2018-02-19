use ram::Ram;
use instruction::Instruction;

pub const START: u16 = 0x200;
pub const CARRY_FLAG: usize = 0xF;

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

    fn adds_vy_to_vx(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        let y = instruction.y() as usize;
        let sum : u16 = self.reg_vx[x] as u16 + self.reg_vx[y] as u16;
        self.reg_vx[CARRY_FLAG] = if sum > 0xFF { 0x1 } else { 0x0 };
        self.reg_vx[x] = sum as u8;
        self.pc += 2;
    }

    fn subtracts_vy_to_vx(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        let y = instruction.y() as usize;

        let subtract = self.reg_vx[x] as i8 - self.reg_vx[y] as i8;
        self.reg_vx[x] = subtract as u8;
        self.reg_vx[CARRY_FLAG] = if subtract < 0 { 0x1 } else { 0x0 };
        self.pc += 2;
    }

    fn subtracts_vx_to_vy(&mut self, instruction: &Instruction) {
        let x = instruction.x() as usize;
        let y = instruction.y() as usize;

        let subtract = self.reg_vx[y] as i8 - self.reg_vx[x] as i8;
        self.reg_vx[x] = subtract as u8;
        self.reg_vx[CARRY_FLAG] = if subtract < 0 { 0x0 } else { 0x1 };
        self.pc += 2;
    }

    fn jump_to_address_nnn_plus_v0(&mut self, instruction: &Instruction) {
        self.pc = START + instruction.nnn() + self.reg_vx[0] as u16;
    }

    fn jump_to_address_nnn(&mut self, instruction: &Instruction) {
        self.pc = START + instruction.nnn();
        println!("{}", self.pc);
    }

    // Adds VX to I. 
    // I +=Vx
    fn adds_vx_to_i(&mut self, instruction: &Instruction) {
        self.i = self.i + self.reg_vx[instruction.x() as usize] as u16;
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
            (0x1, _) => self.jump_to_address_nnn(instruction),
            (0x3, _) => self.skip_on_vx_equal_nn(instruction),
            (0x4, _) => self.skip_on_vx_not_equal_nn(instruction),
            (0x5, _) => self.skip_on_vx_equal_vy(instruction),
            (0x6, _) => self.write_on_vx(instruction),
            (0x7, _) => self.add_on_vx(instruction),
            (0x8, 0x0) => self.assign_vx_to_vy(instruction),
            (0x8, 0x1) => self.bitwise_or(instruction),
            (0x8, 0x2) => self.bitwise_and(instruction),
            (0x8, 0x3) => self.bitwise_xor(instruction),
            (0x8, 0x4) => self.adds_vy_to_vx(instruction),
            (0x8, 0x5) => self.subtracts_vy_to_vx(instruction),
            (0x8, 0x7) => self.subtracts_vx_to_vy(instruction),
            (0x9, 0x0) => self.skip_on_vx_not_equal_vy(instruction),
            (0xA, _) => self.write_i(instruction),
            (0xB, _) => self.jump_to_address_nnn_plus_v0(instruction),
            (0xF, _) => self.adds_vx_to_i(instruction),
            _ => panic!("Unknown instruction {}", raw)
        }

    }
}