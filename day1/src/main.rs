use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn read_data() -> String {
    let file_name = "input";
    std::fs::read_to_string(file_name).expect("Error reading the file")
}

fn parse_to_lists(data: String) -> (Vec<usize>, Vec<usize>) {
    let x: Vec<(usize, usize)> = data
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split_whitespace().collect();
            (
                split[0].to_string().parse().unwrap(),
                split[1].to_string().parse().unwrap(),
            )
        })
        .collect();

    let mut a = Vec::new();
    let mut b = Vec::new();
    x.into_iter().for_each(|(x, y)| {
        a.push(x);
        b.push(y);
    });

    a.sort();
    b.sort();

    (a, b)
}

fn part1() {
    let data = read_data();
    let lists = parse_to_lists(data);

    assert!(lists.0.len() == lists.1.len());

    let mut total = 0;
    for i in 0..lists.0.len() {
        let a = lists.0[i];
        let b = lists.1[i];
        let diff = a.abs_diff(b);
        total += diff;
    }

    println!("Part1 Total: {}", total);
}

fn part2() {
    let data = read_data();
    let lists = parse_to_lists(data);

    assert!(lists.0.len() == lists.1.len());

    let occurances: HashMap<usize, usize> = list_to_ocurances(lists.1);

    let total: usize = lists
        .0
        .iter()
        .map(|x| occurances.get(x).unwrap_or(&0) * x)
        .sum();
    println!("Part2 Total: {}", total);
}

fn list_to_ocurances(list: Vec<usize>) -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    list.iter().for_each(|x| *map.entry(*x).or_insert(0) += 1);

    map
}
