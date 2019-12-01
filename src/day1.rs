#[aoc_generator(day1)]
pub fn day1_generator(input: &str) -> Vec<i32> {
  input
    .lines()
    .map(|line| line.to_string().parse::<i32>().unwrap())
    .collect()
}

#[aoc(day1, part1)]
pub fn part1(module_mass: &Vec<i32>) -> i32 {
  module_mass.iter().map(|mass| fuel_for_mass(mass)).sum()
}

#[aoc(day1, part2)]
pub fn part2(module_mass: &Vec<i32>) -> i32 {
  module_mass
    .iter()
    .map(|mass| fuel_for_mass_recursive(mass))
    .sum()
}

fn fuel_for_mass(mass: &i32) -> i32 {
  (mass / 3) - 2
}

fn fuel_for_mass_recursive(mass: &i32) -> i32 {
  let required_fuel = fuel_for_mass(mass);
  if required_fuel > 0 {
    required_fuel + fuel_for_mass_recursive(&required_fuel)
  } else {
    0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_one_works_for_example_cases() {
    assert_eq!(part1(&vec!(12)), 2);
    assert_eq!(part1(&vec!(14)), 2);
    assert_eq!(part1(&vec!(1969)), 654);
    assert_eq!(part1(&vec!(100756)), 33583);
  }

  #[test]
  fn part_two_works_for_example_cases() {
    assert_eq!(part2(&vec!(14)), 2);
    assert_eq!(part2(&vec!(1969)), 966);
    assert_eq!(part2(&vec!(100756)), 50346);
  }
}
