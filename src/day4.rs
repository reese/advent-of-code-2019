extern crate itertools;

use itertools::Itertools;

#[aoc_generator(day4)]
fn day_four_generator(input: &str) -> Vec<i64> {
    input.split("-").map(|i| i.parse::<i64>().unwrap()).collect()
}

#[aoc(day4, part1)]
fn day_four(input: &Vec<i64>) -> i64 {
    let mut count = 0;
    for number in *input.get(0).unwrap()..*input.get(1).unwrap() {
        let number_as_string = number.to_string();


        if has_six_digits(&number_as_string) && // This _should_ be true given the input, but checking for portability
            all_digits_are_ascending(&number_as_string) &&
            has_double_digit(&number_as_string){
            count += 1;
            println!("{:?}: (current_count: {:?})", number_as_string, count);
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

#[cfg(test)]
mod test {
}