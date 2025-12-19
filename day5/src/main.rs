use core::panic;
use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

fn main() {
    println!("part1:");
    part1();

    println!("part2:");
    part2();
}

fn part1() {
    let db = load_input("input.txt");
    let fresh_count = db.calc_fresh_count();
    println!("fresh count: {}", fresh_count);
}

fn part2() {
    // let mut db = load_input("input_example.txt");
    // let mut db = load_input("input_example2.txt");
    let mut db = load_input("input.txt");
    // db.fresh_ranges.iter().for_each(|range| {
    //     println!("range: {:?}", range);
    // });
    db.flatten_ranges();
    // db.fresh_ranges.iter().for_each(|range| {
    //     println!("flattened range: {:?}", range);
    // });
    let fresh_ids = db.calc_fresh_ids_count();
    println!("fresh ids: {:?}", fresh_ids);
}

fn load_input(filename: &str) -> Db {
    let mut fresh_ranges: Vec<RangeInclusive<i64>> = Vec::new();
    let mut found_spacer: bool = false;
    let mut availables: Vec<i64> = Vec::new();

    std::fs::read_to_string(filename)
        .expect("Unable to read input!")
        .trim()
        .lines()
        .for_each(|line| {
            let line = line.trim();

            if line.is_empty() {
                found_spacer = true;
                return;
            }

            if found_spacer {
                let available: i64 = line.parse().unwrap();
                availables.push(available);
            } else {
                let start = line.split('-').next().unwrap().parse().unwrap();
                let end = line.split('-').nth(1).unwrap().parse().unwrap();

                if start < 0 {
                    panic!("start is negative!");
                }

                if end < 0 {
                    panic!("end is negative!");
                }

                if start > end {
                    panic!("end is lower then start");
                }

                fresh_ranges.push(start..=end);
            }
        });

    return Db {
        fresh_ranges: fresh_ranges,
        availables: availables,
    };
}

#[derive(Debug)]
struct Db {
    fresh_ranges: Vec<RangeInclusive<i64>>,
    availables: Vec<i64>,
}

impl Db {
    fn calc_fresh_count(&self) -> i64 {
        let mut fresh_count: i64 = 0;

        self.availables.iter().for_each(|available| {
            let any_in_range = self
                .fresh_ranges
                .iter()
                .any(|range| range.contains(available));

            if any_in_range {
                fresh_count += 1;
            }
        });

        return fresh_count;
    }

    fn flatten_ranges(&mut self) {
        self.fresh_ranges.sort_by(|a, b| a.start().cmp(b.start()));

        let mut new_ranges: Vec<RangeInclusive<i64>> = self.fresh_ranges.clone();

        let mut current_range_index = 0;
        while current_range_index < new_ranges.len() - 1 {
            // println!("current_range_index: {}", current_range_index);
            // println!("new_ranges: {:?}", new_ranges);
            let current_range = &new_ranges[current_range_index];
            let next_range = &new_ranges[current_range_index + 1];

            // println!(
            //     "current_range: {:?}, next_range: {:?}",
            //     current_range, next_range
            // );

            // check if ranges overlap or are directly adjacent
            if *current_range.end() +1 >= *next_range.start() {
                // create new merged range
                let lowest_start = min(*current_range.start(), *next_range.start());
                let highest_end = max(*current_range.end(), *next_range.end());
                let merged_range = lowest_start..=highest_end;

                // remove current and next ranges (after the removing current_indx, the current_idx
                // points to the next_idx) and repplace with merged
                new_ranges.remove(current_range_index);
                new_ranges.remove(current_range_index);

                new_ranges.insert(current_range_index, merged_range.clone());

                // no need to increment current_range_index, as we need to check the new merged
                // range with the next one

                // println!("merged_range: {:?}", merged_range);
            } else {
                // println!("no overlap between ranges");
                // keep current_range and check next
                current_range_index += 1;
            }
            // println!()
        }

        self.fresh_ranges = new_ranges;
    }

    fn calc_fresh_ids_count(&self) -> i64 {
        let sum: i64 = self
            .fresh_ranges
            .iter()
            .map(|range| range.clone().count() as i64)
            .sum();

        return sum;
    }
}
