#![allow(unused)]

use std::{
    fmt::{Display, Formatter},
    iter::repeat,
};

fn main() {
    println!("Part 1:");
    part1();

    println!("Part 2:");
    part2();
}

fn part1() {
    let input = read_input("input.txt");
    let mut sum_of_invalide_ranges: i64 = 0;
    input.iter().for_each(|range| {
        // println!("{}", range);
        let sum: i64 = range.check_for_invalid_part1();
        sum_of_invalide_ranges += sum;
    });

    println!("Sum of invalides: {}", sum_of_invalide_ranges);
}

fn part2() {
    let input = read_input("input.txt");
    let mut sum_of_invalide_ranges: i64 = 0;
    input.iter().for_each(|range| {
        // println!("{}", range);
        let sum: i64 = range.check_for_invalid_part2();
        sum_of_invalide_ranges += sum;
    });

    println!("Sum of invalides: {}", sum_of_invalide_ranges);
}

fn read_input(file_name: &str) -> Vec<IdRange> {
    let input = std::fs::read_to_string(file_name).expect("Failed to read input file");
    input
        .trim()
        .split(',')
        .map(|range| {
            let ranges: Vec<i64> = range
                .split('-')
                .into_iter()
                .map(|str| i64::from_str_radix(str.trim(), 10).unwrap())
                .collect();

            IdRange {
                lower: ranges[0],
                upper: ranges[1],
            }
        })
        .collect::<Vec<IdRange>>()
}

struct IdRange {
    lower: i64,
    upper: i64,
}

impl Display for IdRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.lower, self.upper)
    }
}

impl IdRange {
    fn check_for_invalid_part1(&self) -> i64 {
        let mut sum_of_invalides: i64 = 0;
        for i in self.lower..self.upper + 1 {
            // println!("Checking number: {}", i);
            let sum: i64 = check_for_repeat(i).into_iter().sum();
            sum_of_invalides += sum;
        }
        return sum_of_invalides;
    }

    fn check_for_invalid_part2(&self) -> i64 {
        let mut sum_of_invalides: i64 = 0;
        for i in self.lower..self.upper + 1 {
            // println!("Checking number: {}", i);
            let sum: i64 = check_for_repeat_in_whole(i).into_iter().sum();
            sum_of_invalides += sum;
        }
        return sum_of_invalides;
    }
}

fn check_for_repeat(number: i64) -> Vec<i64> {
    let mut found_repeats = Vec::new();
    let input = number.to_string();

    if input.len() % 2 != 0 {
        return found_repeats; // Not even length -> no repeats
    }

    let half_size = input.len() / 2;

    let to_check = input.split_at(half_size).0;
    let possible_repeat = input.split_at(half_size).1;

    if to_check == possible_repeat {
        // println!("  Found repeat in: {}", number);
        found_repeats.push(number);
    }

    return found_repeats;
}

fn check_for_repeat_in_whole(number: i64) -> Vec<i64> {
    let mut found_repeats = Vec::new();
    let input = number.to_string();

    let haldf_size = input.len() / 2;
    for i in 1..haldf_size + 1 {
        let to_check = input.split_at(i).0;
        let rest = input.split_at(i).1;
        // println!("  Checking for repeat: {}", to_check);
        // println!("  Rest: {}", rest);

        let size_to_check = to_check.len();
        // println!("  Size to check: {}", size_to_check);

        if (input.len() % size_to_check) != 0 {
            // println!("    Skipping size: {} (not divisible)", size_to_check);
            continue;
        }

        let mut build_possible_repeat = Vec::new();
        for j in (size_to_check..input.len()).step_by(size_to_check) {
            let possible = &input[j..j + size_to_check];
            build_possible_repeat.push(possible);
        }

        let mut all_match = true;
        build_possible_repeat.into_iter().for_each(|part| {
            if part != to_check {
                all_match = false;
            }
        });

        if (all_match) {
            found_repeats.push(number);
            break;
        }
    }

    found_repeats.iter().for_each(|i| {
        // println!("  Found repeat number: {}", i);
    });

    return found_repeats;
}
