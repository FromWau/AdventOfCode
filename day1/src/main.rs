use std::fmt::Display;

fn main() {
    // println!("Part 1:");
    // part1();
    //
    // println!("==============================");

    println!("Part 2:");
    part2();
}

fn part1() {
    const MAX_VALUE: i32 = 100;
    const START_NUMBER: i32 = 50;
    const POSITION_TO_BE_COUNTED: i32 = 0;
    let mut position_count = 0;

    let mut number: i32 = START_NUMBER;
    let ops = read_input("input.txt");

    println!("The dial starts by pointing at {}.", number);
    for op in ops {
        match op.sign {
            'R' => {
                number = (number + op.amount) % MAX_VALUE;
            }
            'L' => {
                number = (number - op.amount + MAX_VALUE) % MAX_VALUE;
            }
            _ => println!("Unknown operation: {}", op.sign),
        }

        println!("The dial is rotatet {} to point at {}.", op, number);
        if number == POSITION_TO_BE_COUNTED {
            position_count += 1;
        }
    }

    println!("Final number: {}", number);
    println!(
        "The dial pointed at {} a total of {} times.",
        POSITION_TO_BE_COUNTED, position_count
    );
}

fn part2() {
    const MAX_VALUE: i32 = 100;
    const START_NUMBER: i32 = 50;
    const POSITION_TO_BE_COUNTED: i32 = 0;
    let mut position_count: i32 = 0;

    let mut number: i32 = START_NUMBER;
    // let ops = read_input("input_example2.txt");
    // let ops = read_input("input_example.txt");
    let ops = read_input("input.txt");

    println!("The dial starts by pointing at {}.", number);
    for op in ops {
        let mut how_often: i32 = 0;

        match op.sign {
            'R' => {
                let add = number + op.amount;
                how_often = add / MAX_VALUE;

                number = (number + op.amount) % MAX_VALUE;
            }
            'L' => {
                how_often = if number == 0 {
                    // Special case: starting at 0
                    // Only cross 0 after full rotations (100, 200, 300 steps)
                    op.amount / MAX_VALUE
                } else if op.amount >= number {
                    // We'll cross 0 at least once
                    // First crossing after 'number' steps, then every 100 steps after that
                    1 + (op.amount - number) / MAX_VALUE
                } else {
                    // Not enough rotation to reach 0
                    0
                };

                number = (number - op.amount).rem_euclid(MAX_VALUE);
            }
            _ => println!("Unknown operation: {}", op.sign),
        }

        print!("The dial is rotatet {} to point at {}", op, number);
        if how_often > 0 {
            position_count += how_often;
            println!(
                "; during this rotation, it points at {}, {} times.",
                POSITION_TO_BE_COUNTED, how_often
            );
        } else {
            println!(".");
        }
    }

    println!(
        "The dial pointed at {} a total of {} times.",
        POSITION_TO_BE_COUNTED, position_count
    );
}

fn read_input(file_name: &str) -> Vec<Opertation> {
    let input = std::fs::read_to_string(file_name).expect("Failed to read input file");

    input
        .lines()
        .map(|line| {
            let sign = line.chars().next().unwrap();
            let amount: i32 = line[1..].parse().unwrap();
            Opertation { sign, amount }
        })
        .collect()
}

#[derive(Debug)]
struct Opertation {
    sign: char,
    amount: i32,
}

impl Display for Opertation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}{}", self.sign, self.amount))
    }
}
