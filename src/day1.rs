#[aoc_generator(day1)]
pub fn day1_generator(input: &str) -> Vec<i32> {
  input
    .lines()
    .map(|line| line.to_string().parse::<i32>().unwrap())
    .collect()
}

#[aoc(day1, part1)]
pub fn part1(module_mass: &Vec<i32>) -> i32 {
  module_mass.iter().map(|mass| (mass / 3) - 2).sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works_for_base_case() {
    assert_eq!(part1(&vec!(12)), 2)
  }

  #[test]
  fn it_rounds_down() {
    assert_eq!(part1(&vec!(14)), 2)
  }

  #[test]
  fn it_works_for_larger_input() {
    assert_eq!(part1(&vec!(1969)), 654)
  }

  #[test]
  fn it_works_for_even_larget_input() {
    assert_eq!(part1(&vec!(100756)), 33583)
  }
}
