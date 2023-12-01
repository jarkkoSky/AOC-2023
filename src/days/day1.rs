use std::fs;

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_row_part_1(row: &str) -> i32 {
    let first = row.chars().find(|char| char.is_numeric()).unwrap();
    let last = row.chars().filter(|char| char.is_numeric()).last().unwrap();

    format!("{}{}", first, last).parse::<i32>().unwrap()
}

fn find_all_written_numbers(row: &str) -> Vec<(usize, usize)> {
    NUMBERS
        .iter()
        .flat_map(|num| {
            row.match_indices(num)
                .map(|(index, _)| {
                    let numeric_value = NUMBERS.iter().position(|r| r == num).unwrap() + 1;
                    Some((numeric_value, index))
                })
                .collect::<Vec<Option<(usize, usize)>>>()
        })
        .flatten()
        .collect()
}

fn find_all_numeric_numbers(row: &str) -> Vec<(usize, usize)> {
    row.chars()
        .enumerate()
        .filter_map(|(index, c)| match c.is_numeric() {
            true => Some((c.to_digit(10).unwrap() as usize, index)),
            false => None,
        })
        .collect()
}

fn parse_row_part_2(row: &str) -> i32 {
    let mut nums: Vec<(usize, usize)> = find_all_written_numbers(row);
    let mut numeric_numbers = find_all_numeric_numbers(row);

    nums.append(&mut numeric_numbers);
    nums.sort_by(|a, b| a.1.cmp(&b.1));

    let first = nums.first().unwrap().0;
    let last = nums.last().unwrap().0;

    format!("{}{}", first, last).parse::<i32>().unwrap()
}

pub fn run() {
    let file = fs::read_to_string("src/inputs/day1.txt").unwrap();

    let part1: i32 = file.lines().map(parse_row_part_1).sum();
    let part2: i32 = file.lines().map(parse_row_part_2).sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
