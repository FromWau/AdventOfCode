use std::collections::HashMap;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let file_name: &str = args.get(2).map(|f| f.as_str()).unwrap_or("input.txt");
    let part: &str = args.get(1).map(|p| p.as_str()).unwrap_or("-p1");

    match part {
        "-p1" => {
            println!("Part 1 - File: {}", file_name);
            part1(file_name);
        }

        "-p2" => {
            println!("Part 2 - File: {}", file_name);
            part2(file_name);
        }
        _ => {
            println!("Please specify part 1 or part 2 with -p1 or -p2");
        }
    }
}

fn read_input(file_name: &str) -> Vec<Calc> {
    let rows: Vec<Vec<String>> = std::fs::read_to_string(file_name)
        .expect("Failed to read input file")
        .trim()
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.to_string()).collect())
        .collect();

    let mut calculations: Vec<Calc> = vec![];

    (0..=rows[0].len() - 1).into_iter().for_each(|col_idx| {
        let mut numbers: Vec<i64> = vec![];
        (0..=rows.len() - 2).into_iter().for_each(|row_idx| {
            let val = &rows[row_idx][col_idx];
            numbers.push(val.parse::<i64>().unwrap());
        });
        let op = match rows[rows.len() - 1][col_idx].as_str() {
            "+" => Operation::ADD,
            "-" => Operation::SUB,
            "*" => Operation::MUL,
            "/" => Operation::DIV,
            _ => panic!("Unknown operation"),
        };

        calculations.push(Calc {
            operation: op,
            numbers,
        });
    });

    return calculations;
}

fn read_input2(file_name: &str) -> Vec<Calc> {
    let input = std::fs::read_to_string(file_name)
        .expect("Failed to read input file")
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let longest_line = input.iter().map(|line| line.len()).max().unwrap();
    let last_row: Vec<char> = {
        let mut last_row_build: Vec<char>;

        last_row_build = input.last().unwrap().chars().collect();

        // add missing spaces to the last row
        if last_row_build.len() < longest_line {
            let mut extended_last_row: Vec<char> = last_row_build.clone();
            for _ in 0..(longest_line - last_row_build.len()) {
                extended_last_row.push(' ');
            }

            // end delimiter
            extended_last_row.push(' ');

            last_row_build = extended_last_row;
        }

        last_row_build
    };

    let ops: Vec<Operation> = input
        .last()
        .unwrap()
        .chars()
        .filter_map(|c| match c {
            '+' => Some(Operation::ADD),
            '-' => Some(Operation::SUB),
            '*' => Some(Operation::MUL),
            '/' => Some(Operation::DIV),
            _ => None,
        })
        .collect();

    let mut space_map: HashMap<usize, usize> = HashMap::new();
    {
        let mut idx = 0;
        let mut spaces = 0;
        let mut ignore_first = true;

        for char in &last_row {
            match char {
                '+' | '-' | '*' | '/' => {
                    if ignore_first {
                        ignore_first = false;
                        continue;
                    }

                    space_map.insert(idx, spaces);
                    spaces = 0;
                    idx += 1;
                }

                ' ' => {
                    spaces += 1;
                }

                _ => panic!("Unknown operation"),
            }
        }
        // insert last entry
        space_map.insert(idx, spaces);
    }

    let rows: Vec<Vec<Vec<char>>> = (0..=input.len() - 2)
        .into_iter()
        .map(|idx| input[idx].as_str())
        .map(|line| {
            // give max line padding + end delimiter
            if longest_line > line.len() {
                let mut extended_line = line.to_string();
                for _ in 0..(longest_line - line.len()) {
                    extended_line.push(' ');
                }
                extended_line.push(' '); // end delimiter
                extended_line
            } else {
                let mut extended_line = line.to_string();
                extended_line.push(' '); // end delimiter
                extended_line
            }
        })
        .map(|line| {
            let mut nums: Vec<Vec<char>> = vec![];

            let mut map_index = 0;
            let mut calc_line = line.clone();

            while 0 < calc_line.len() {
                let spaces = space_map.get(&map_index).unwrap();
                let num = calc_line.clone().split_at(*spaces).0.to_string();
                calc_line = calc_line.split_at(*spaces + 1).1.to_string();

                // println!("num: '{}'", num);
                let char_vec: Vec<char> = num.chars().collect();
                nums.push(char_vec);

                map_index += 1;
            }

            nums
        })
        .collect();

    let transformed_rows: Vec<Vec<Vec<char>>> = (0..rows[0].len())
        .into_iter()
        .map(|col_idx| {
            let mut new_row: Vec<Vec<char>> = vec![];

            (0..rows.len()).into_iter().for_each(|row_idx| {
                new_row.push(rows[row_idx][col_idx].clone());
            });

            new_row
        })
        .collect();

    let mut calcs: Vec<Calc> = Vec::new();

    (0..transformed_rows.len())
        .into_iter()
        .for_each(|section_idx| {
            // println!("section_idx: {}", section_idx);

            let section = &transformed_rows[section_idx];
            // println!("section: {:?}", section);

            let calc_numbers = (0..section[0].len())
                .into_iter()
                .map(|col_idx| {
                    let col_numbers = (0..section.len())
                        .into_iter()
                        .map(|row_idx| section[row_idx][col_idx])
                        .collect::<Vec<char>>();

                    into_number(col_numbers)
                })
                .rev()
                .collect::<Vec<i64>>();

            let op = ops[section_idx].clone();

            calcs.push(Calc {
                operation: op,
                numbers: calc_numbers,
            });
        });

    // println!("Calcs: {:?}", calcs);

    return calcs;
}

#[derive(Debug)]
struct Calc {
    operation: Operation,
    numbers: Vec<i64>,
}

impl Calc {
    fn execute(&self) -> i64 {
        match self.operation {
            Operation::ADD => self.numbers.iter().sum(),
            Operation::SUB => self.numbers.iter().fold(0, |acc, &x| acc - x),
            Operation::MUL => self.numbers.iter().product(),
            Operation::DIV => self.numbers.iter().fold(1, |acc, &x| acc / x),
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    ADD,
    SUB,
    MUL,
    DIV,
}

fn part1(file_name: &str) {
    let input = read_input(file_name);
    let result: i64 = input.iter().map(|calc| calc.execute()).sum();

    println!("{:?}", result);
}

fn part2(file_name: &str) {
    let input = read_input2(file_name);
    let result: i64 = input.iter().map(|calc| calc.execute()).sum();

    println!("{:?}", result);
}

fn into_number(vec: Vec<char>) -> i64 {
    let str: String = vec
        .iter()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_string())
        .collect();

    i64::from_str_radix(&str, 10).unwrap()
}
