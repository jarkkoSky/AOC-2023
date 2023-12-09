use std::fs;

fn aperture(row: &Vec<i32>) -> Vec<Vec<&i32>> {
    row.iter()
        .enumerate()
        .flat_map(|(index, item)| match &row.get(index + 1) {
            Some(next_item) => Some(vec![item, next_item]),
            _ => None,
        })
        .collect::<Vec<Vec<&i32>>>()
}

fn row_prediction(row: Vec<i32>, part: i32) -> i32 {
    let mut sequences: Vec<Vec<i32>> = vec![row];

    while sequences.last().unwrap().iter().all(|x| *x == 0) == false {
        sequences.push(
            aperture(sequences.last().unwrap())
                .iter()
                .map(|x| x[1] - x[0])
                .collect(),
        );
    }

    sequences.reverse();
    sequences.iter().fold(0, |acc, e| {
        if part == 2 {
            return e.first().unwrap() - acc;
        }

        acc + e.last().unwrap()
    })
}

pub fn run() {
    let rows: Vec<Vec<i32>> = fs::read_to_string("src/inputs/day9.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split(" ")
                .flat_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect();

    let part1: i32 = rows.iter().map(|row| row_prediction(row.clone(), 1)).sum();
    let part2: i32 = rows.iter().map(|row| row_prediction(row.clone(), 2)).sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
