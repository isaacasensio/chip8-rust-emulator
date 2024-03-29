use cpu::Cpu;
use ram::Ram;

mod ram;
mod cpu;
mod instruction;

pub struct Chip8 {
    ram: Ram,
    cpu: Cpu
}

impl Chip8 {
    fn new () -> Chip8{
        Chip8{
            ram: Ram::new(),
            cpu: Cpu::new()
        }
    }
}