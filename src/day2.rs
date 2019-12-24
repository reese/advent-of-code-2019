use crate::intcode::Program;

const PART_TWO_EXPECTED_RESULT: i32 = 19690720;

#[aoc_generator(day2)]
pub fn day2_generator(input: &str) -> Vec<i32> {
  input
    .split(",")
    .map(|number| number.to_string().parse::<i32>())
    .filter_map(Result::ok)
    .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<i32>) -> i32 {
  let mut program = Program::create(input.clone());
  program.run();
  program.get_return_value()
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<i32>) -> i32 {
  for position_one in 0..99 {
    for position_two in 0..99 {
      let mut initial_program = input.clone();
      initial_program[1] = position_one;
      initial_program[2] = position_two;
      let mut program = Program::create(initial_program);
      program.run();

      if  program.get_return_value() == PART_TWO_EXPECTED_RESULT {
        return position_one * 100 + position_two
      }
    }
  }

  panic!("Qualifying pair not found.")
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs::read_to_string;

  #[test]
  fn part_one_works_for_example_cases() {
    let input = "1,9,10,3,2,3,11,0,99,30,40,50";
    let expected_output = 3500;

    assert_eq!(part1(&day2_generator(input)), expected_output);
  }

  #[test]
  fn it_returns_the_correct_answer_for_part_one() {
    let input = get_input_file();

    assert_eq!(part1(&day2_generator(input.as_ref())), 7210630);
  }

  #[test]
  fn it_returns_the_correct_answer_for_part_two() {
    let input = get_input_file();

    assert_eq!(part2(&day2_generator(input.as_ref())), 3892);
  }

  fn get_input_file() -> String {
    read_to_string("input/2019/day2.txt").expect("Could not read file.")
  }
}
