struct Cpu {
    memory: [u8; 4096],

    vx: [u8; 16], //general purpose registries

    i_reg: u8, // Index registry
    pc: u16, // Program counter. Currently executing address.
    sp: u8, // Stack pointer: points to top of the stack

    stack: [u16; 16] // Stores address that interpreter returns when a routine finishes
    
}



fn main() {
    println!("Hello, world!");
}
