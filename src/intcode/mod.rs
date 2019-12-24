const ADD: i32 = 1;
const MULTIPLY: i32 = 2;
const HALT: i32 = 99;

pub struct Program {
    instructions: Vec<i32>
}

impl Program {
    pub fn create(input: Vec<i32>) -> Program {
        Program { instructions: input }
    }

    pub fn run(&mut self) {
        let mut current_index = 0;
        let mut current_value = self.instructions.get(current_index).expect("Could not get instruction at given index.");
        while *current_value != HALT {
            match *current_value {
                ADD => self.add(current_index),
                MULTIPLY => self.multiply(current_index),
                _ => panic!("Unknown operator."),
            }

            current_index += 4;
            current_value = self.instructions.get(current_index).expect("Could not get instruction at given index.");
        }
    }

    pub fn get_return_value(self) -> i32 {
        self.get_instruction(0)
    }

    pub fn get_instruction(&self, index: usize) -> i32 {
        self.instructions.get(index).expect("Could not get instruction at index.").clone()
    }

    fn add(&mut self, index: usize) {
        let storage_index = self.get_instruction(index + 3);
        let (instruction_one, instruction_two) = self.get_next_two_instructions(index);
        self.instructions[storage_index as usize] = self.get_instruction(instruction_one as usize) + self.get_instruction(instruction_two as usize);
    }

    fn multiply(&mut self, index: usize) {
        let storage_index = self.get_instruction(index + 3);
        let (instruction_one, instruction_two) = self.get_next_two_instructions(index);
        self.instructions[storage_index as usize] = self.get_instruction(instruction_one as usize) * self.get_instruction(instruction_two as usize);
    }

    fn get_next_two_instructions(&self, index: usize) -> (i32, i32) {
        let instruction_one = self.get_instruction(index + 1);
        let instruction_two = self.get_instruction(index + 2);
        (instruction_one, instruction_two)
    }
}

