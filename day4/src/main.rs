const ROLL_CHAR: char = '@';
const EMPTY_CHAR: char = '.';
const MARKED_CHAR: char = 'X';

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let file_name: &str = args.get(2).map(|f| f.as_str()).unwrap_or("input.txt");

    match args[1].as_str() {
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

fn read_input(file_name: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(file_name)
        .expect("Failed to read input file")
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn part1(file_name: &str) {
    let mut field = read_input(file_name);
    println!("Input Field:");
    print_field(&field);

    println!("Searching for rolls...");

    let count = remove_marked_rolls(&mut field);

    println!("Field after removing rolls:");
    print_field(&field);

    println!("Total removable rolls: {}", count);
}

fn part2(file_name: &str) {
    let mut field = read_input(file_name);
    println!("Input Field:");
    print_field(&field);


    let mut total_count = 0;
    let mut has_removables = true;
    while has_removables {
        let count = remove_marked_rolls(&mut field);
        total_count += count;

        if count == 0 {
            has_removables = false;
        }
    }

    println!("Field after removing all rolls:");
    print_field(&field);

    println!("Total removable rolls: {}", total_count);
}

fn print_field(field: &[Vec<char>]) {
    field.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{}", c));
        println!();
    });
}

fn remove_marked_rolls(field: &mut [Vec<char>]) -> i32 {
    let field_copy = field.to_vec();

    // println!("Searching for rolls...");
    let mut total_remobale_rolls = 0;
    for y in 0..field_copy.len() {
        for x in 0..field_copy[y].len() {
            if is_roll(&field_copy, y, x) {
                let roll_neighbors = count_roll_neighbors(&field_copy, y, x);
                if roll_neighbors < 4 {
                    // We can pick this roll (Mark for removel)
                    // println!("Roll can be removed at ({}, {})", roll_neighbors, y);
                    total_remobale_rolls += 1;
                    field[y][x] = MARKED_CHAR;
                }
            }
        }
    }

    return total_remobale_rolls;
}

fn count_roll_neighbors(field: &[Vec<char>], y: usize, x: usize) -> i32 {
    // println!("Counting roll neighbors for ({}, {})", x, y);

    let mut roll_neighbor_cout = 0;

    let field_height = field.len() as isize;
    let field_width = field[0].len() as isize;
    // println!("field size: {} x {}", field_width, field_height);

    #[rustfmt::skip]
    let directions: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    for (dx, dy) in directions {
        let actual_x: isize = x as isize + dx;
        let actual_y: isize = y as isize + dy;

        // print!("Checking neighbor at ({}, {})", actual_x, actual_y);

        if actual_x < 0 || actual_x > field_width - 1 {
            // out of bounds
            // println!("Out of bounds X");
            // println!();
            continue;
        }

        if actual_y < 0 || actual_y > field_height - 1 {
            // out of bounds
            // println!("Out of bounds Y");
            // println!();
            continue;
        }

        if is_roll(field, actual_y as usize, actual_x as usize) {
            // println!(" - Found roll");
            roll_neighbor_cout += 1;
        }
    }

    roll_neighbor_cout
}

fn is_empty(field: &[Vec<char>], y: usize, x: usize) -> bool {
    is_char(field, y, x, EMPTY_CHAR)
}

fn is_roll(field: &[Vec<char>], y: usize, x: usize) -> bool {
    is_char(field, y, x, ROLL_CHAR)
}

fn is_char(field: &[Vec<char>], y: usize, x: usize, check_char: char) -> bool {
    field[y][x] == check_char
}
