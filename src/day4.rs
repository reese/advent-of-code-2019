extern crate itertools;

use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day4)]
fn day_four_generator(input: &str) -> Vec<i64> {
    input.split("-").map(|i| i.parse::<i64>().unwrap()).collect()
}

#[aoc(day4, part1)]
fn day_four_part_one(input: &Vec<i64>) -> i64 {
    let mut count = 0;
    for number in *input.get(0).unwrap()..*input.get(1).unwrap() {
        let number_as_string = number.to_string();


        if has_six_digits(&number_as_string) && // This _should_ be true given the input, but checking for portability
            all_digits_are_ascending(&number_as_string) &&
            has_double_digit(&number_as_string){
            count += 1;
        }
    }
    count
}
#[aoc(day4, part2)]
fn day_four_part_two(input: &Vec<i64>) -> i64 {
    let mut count = 0;
    for number in *input.get(0).unwrap()..*input.get(1).unwrap() {
        let number_as_string = number.to_string();


        if has_six_digits(&number_as_string) && // This _should_ be true given the input, but checking for portability
            all_digits_are_ascending(&number_as_string) &&
            has_exact_double_digit(&number_as_string){
            count += 1;
        }
    }
    count
}


fn has_six_digits(input: &String) -> bool {
    input.len() == 6
}

fn all_digits_are_ascending(input: &String) -> bool {
    let digits = input.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
    digits.into_iter().tuple_windows().all(|(prev, next)| prev <= next)
}

fn has_double_digit(input: &String) -> bool {
    let digits = input.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
    digits.into_iter().tuple_windows().any(|(prev, next)| prev == next)
}

fn has_exact_double_digit(input: &String) -> bool {
    let digits = input.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
    let mut double_digit_count: HashMap<u32, u32> = HashMap::new();
    digits.into_iter().tuple_windows().for_each(|(prev, next)| {
        if prev == next {
            let counter = double_digit_count.entry(prev).or_insert(0);
            *counter += 1;
        }
    });
    double_digit_count.values().any(|value| *value == 1)
}

