use instruction::Instruction;

mod instruction;

pub struct Executor {
}

impl Executor {

    fn clear_screen(&self){

    }

    pub fn new() -> Executor {
        Executor {

        }
    }

    pub fn execute(&self, instruction: Instruction){
        let val = instruction.raw();
        match val {
            0x00E0 => self.clear_screen(),
            0x00EE => println!("{}", val),
            _ => println!("???"),
        }
    }
}


#[cfg(test)]
mod executor_test {

    use super::Executor;
    use instruction::Instruction;

    #[test]
    fn op_0x00e0_executes() {
        let instruction = Instruction::new(0x00E0);
        Executor::new().execute(instruction);
    }
}

