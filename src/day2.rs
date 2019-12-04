const HALT: i32 = 99;
const ADD: i32 = 1;
const MULTIPLY: i32 = 2;
const PART_TWO_EXPECTED_RESULT: i32 = 19690720;

#[aoc_generator(day2)]
pub fn day2_generator(input: &str) -> Vec<i32> {
  input
    .split(",")
    .map(|number| number.to_string().parse::<i32>().unwrap())
    .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<i32>) -> String {
  let mut program = input.clone();
  run_program(&mut program);

  program
    .iter()
    .map(|num| num.to_string())
    .collect::<Vec<_>>()
    .join(",")
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<i32>) -> i32 {
  for position_one in 0..99 {
    for position_two in 0..99 {
      let mut initial_program = input.clone();
      initial_program[1] = position_one;
      initial_program[2] = position_two;
      if run_program(&mut initial_program) == PART_TWO_EXPECTED_RESULT {
        return position_one * 100 + position_two
      }
    }
  }

  0
}

fn run_program(program: &mut Vec<i32>) -> i32 {
  let mut current_index = 0;
  let mut current_value = program.get(current_index).unwrap();
  while *current_value != HALT as i32 {
    match *current_value {
      ADD => add(program, current_index),
      MULTIPLY => multiply(program, current_index),
      _ => panic!("How did you get here?"),
    }

    current_index += 4;
    current_value = &program[current_index];
  }

  program[0]
}

fn add(input_list: &mut Vec<i32>, index: usize) {
  let storage_index = input_list[index + 3] as usize;
  input_list[storage_index] =
    input_list[input_list[index + 1] as usize] + input_list[input_list[index + 2] as usize];
}

fn multiply(input_list: &mut Vec<i32>, index: usize) {
  let storage_index = input_list[index + 3] as usize;
  input_list[storage_index] =
    input_list[input_list[index + 1] as usize] * input_list[input_list[index + 2] as usize];
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one_works_for_example_cases() {
    let input = "1,9,10,3,2,3,11,0,99,30,40,50";
    let expected_output = "3500,9,10,70,2,3,11,0,99,30,40,50";

    assert_eq!(part1(&day2_generator(input)), expected_output);
  }
}
