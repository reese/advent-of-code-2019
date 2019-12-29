use std::io;

#[derive(Copy, Clone, Debug)]
enum Mode {
    PositionMode = 0,
    ImmediateMode = 1
}

impl Mode {
    pub fn from(num: i32) -> Mode {
        match num {
            0 => Mode::PositionMode,
            1 => Mode::ImmediateMode,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Debug)]
enum Operation {
    None = -1,
    Add = 1,
    Multiply = 2,
    GetInput = 3,
    Print = 4,
    Halt = 99,
}

impl Operation {
    pub fn from(num: i32) -> Operation {
        match num {
            1 => Operation::Add,
            2 => Operation::Multiply,
            3 => Operation::GetInput,
            4 => Operation::Print,
            99 => Operation::Halt,
            _ => unreachable!(),
        }
    }
}

pub struct Program {
    instructions: Vec<i32>,
    operation: Operation,
    modes: [Mode; 3],
    location: i32,
}

impl Program {
    pub fn create(input: Vec<i32>) -> Program {
        Program {
            instructions: input,
            location: 0,
            operation: Operation::None,
            modes: [Mode::PositionMode, Mode::PositionMode, Mode::PositionMode]
        }
    }

    pub fn run(&mut self) {
        self.parse_current_operation();
        while self.operation != Operation::Halt {
            match self.operation {
                Operation::Add => self.add(),
                Operation::Multiply => self.multiply(),
                Operation::GetInput => self.get_input(),
                Operation::Print => self.print(),
                _ => panic!("Unknown operator."),
            }
            self.parse_current_operation();
        }
    }

    pub fn get_return_value(self) -> i32 {
        self.get_instruction(0)
    }

    pub fn get_instruction(&self, index: i32) -> i32 {
        self.instructions.get(index as usize).expect("Could not get instruction at index.").clone()
    }

    fn get_positional_value(&self, offset: i32) -> i32 {
        self.get_instruction(self.get_instruction(self.location + offset))
    }

    fn get_immediate_value(&self, offset: i32) -> i32 {
        self.get_instruction(self.location + offset)
    }

    fn add(&mut self) {
        let storage_index = self.get_immediate_value(3);
        self.instructions[storage_index as usize] = self.get_first_argument() + self.get_second_argument();
        self.location += 4;
    }

    fn multiply(&mut self) {
        let storage_index = self.get_immediate_value(3);
        self.instructions[storage_index as usize] = self.get_first_argument() * self.get_second_argument();
        self.location += 4;
    }

    fn parse_current_operation(&mut self) {
        let padded_operation_string = format!("{:05}", self.get_instruction(self.location));
        self.operation = Operation::from(padded_operation_string.chars().skip(3).take(2).collect::<String>().parse::<i32>().unwrap());
        let modes_vec = padded_operation_string
            .chars()
            .take(3)
            .collect::<Vec<_>>()
            .iter()
            .map(|char| Mode::from(char.to_digit(10).unwrap() as i32))
            .collect::<Vec<_>>();
        self.modes = [*modes_vec.get(0).unwrap(), *modes_vec.get(1).unwrap(), *modes_vec.get(2).unwrap()];
    }

    fn get_input(&mut self) {
        let mut stdin_input = String::new();
        println!("Waiting for input: ");
        io::stdin().read_line(&mut stdin_input).unwrap();
        let index = self.get_immediate_value(1);
        self.instructions[index as usize] = stdin_input.trim().parse::<i32>().unwrap();
        self.location += 2;
    }

    fn print(&mut self) {
        println!("{}", self.get_first_argument());
        self.location += 2;
    }

    fn get_first_argument(&self) -> i32 {
        self.get_argument_by_mode(1, 2)
    }

    fn get_second_argument(&self) -> i32 {
        self.get_argument_by_mode(2, 1)
    }

    fn get_third_argument(&self) -> i32 {
        self.get_argument_by_mode(3, 0)
    }

    fn get_argument_by_mode(&self, offset: i32, mode_index: usize) -> i32 {
        match self.modes[mode_index] {
            Mode::PositionMode => self.get_positional_value(offset),
            Mode::ImmediateMode => self.get_immediate_value(offset),
        }
    }
}

