fn main() {
    let args: Vec<String> = std::env::args().collect();

    let optional_file = args.get(2).map(|f| f.as_str()).unwrap_or("input.txt");

    match args[1].as_str() {
        "-p1" => {
            println!("Part 1:");
            part1(optional_file);
        }
        "-p2" => {
            println!("Part 2:");
            part2(optional_file);
        }
        _ => {
            println!("Invalid argument. Use '1' for Part 1 or '2' for Part 2.");
        }
    }
}

fn part1(file: &str) {
    let input = read_input(file);

    let mut sum: i64 = 0;
    for line in input {
        // line.iter().for_each(|n| print!("{}", n));
        // println!();

        let subvec = &line[0..(line.len() - 1)];
        let highest_tens_digit_index = find_highest_index(&subvec);
        let tens_diget = subvec[highest_tens_digit_index];

        let subvec = &line[(highest_tens_digit_index + 1)..line.len()];
        let highest_ones_digit_index = find_highest_index(&subvec);
        let ones_digit = subvec[highest_ones_digit_index];

        let number = tens_diget * 10 + ones_digit;
        sum += number as i64;
    }

    println!("Sum: {}", sum);
}

fn find_highest_index(line: &[i8]) -> usize {
    let mut max_index: usize = 0;
    for n in 0..line.len() {
        if line[n] > line[max_index] {
            max_index = n;
        }
    }

    return max_index;
}

fn part2(file: &str) {
    let input = read_input(file);

    let mut sum: i64 = 0;
    for line in input {
        // println!("--------------------");
        // line.iter().for_each(|n| print!("{}", n));
        // println!();

        let subvec = build_max_number(&line);

        // print!("Selected batteries: ");
        // subvec.iter().for_each(|n| print!("{}", n));
        // println!();

        sum += vec_to_int(&subvec);
    }

    println!("Sum: {}", sum);
}

fn build_max_number(line: &[i8]) -> Vec<i8> {
    const MAX_LENGTH: usize = 12;

    let mut duplicate = line.to_vec();

    let mut n = 0;
    while duplicate.len() > MAX_LENGTH {
        // print!("-> ");
        // duplicate.iter().for_each(|d| print!("{}", d));
        // println!();

        let current_value = duplicate[n];
        let next_value = duplicate[n + 1];

        // println!(
        //     "Comparing index {} (value {}) with index {} (value {})",
        //     n,
        //     current_value,
        //     n + 1,
        //     next_value
        // );

        if n >= MAX_LENGTH - 1 {
            if current_value <= next_value {
                duplicate.remove(n);
                // println!("Removed index {} (value {})", n, current_value);
                n = 0;
            } else {
                duplicate.remove(n + 1);
                // println!("Removed index {} (value {})", n + 1, next_value);
                n = 0;
            }
        } else {
            if current_value < next_value {
                duplicate.remove(n);
                // println!("Removed index {} (value {})", n, current_value);
                n = 0;
            } else {
                n += 1;
            }
        }
    }

    if duplicate.len() != MAX_LENGTH {
        panic!(
            "Error: duplicate length is {}, expected {}",
            duplicate.len(),
            MAX_LENGTH
        );
    }

    return duplicate;
}

fn vec_to_int(vec: &[i8]) -> i64 {
    vec.iter().fold(0, |acc, &x| acc * 10 + x as i64)
}

fn read_input(file: &str) -> Vec<Vec<i8>> {
    std::fs::read_to_string(file)
        .expect("Failed to read input file")
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect()
}
