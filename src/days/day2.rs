use std::fs;

use itertools::Itertools;

fn get_color_value_from_draw(draw: &Vec<&str>, color: &str) -> i32 {
    match draw.iter().find_position(|x| x.contains(color)) {
        Some(index) => draw[index.0 - 1].parse::<i32>().unwrap(),
        None => 0,
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    /** red, blue, green */
    draws: Vec<(i32, i32, i32)>,
}

impl Game {
    fn power_of_cubes(&self) -> i32 {
        let reds = &self.draws.iter().map(|x| x.0).max().unwrap();
        let blues = &self.draws.iter().map(|x| x.1).max().unwrap();
        let greens = &self.draws.iter().map(|x| x.2).max().unwrap();

        reds * blues * greens
    }

    fn is_possible(&self, (reds, blues, greens): (i32, i32, i32)) -> bool {
        self.draws
            .iter()
            .all(|x| x.0 <= reds && x.1 <= blues && x.2 <= greens)
    }

    pub fn new(row: &str) -> Self {
        let id: i32 = row
            .split(":")
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let draws: Vec<(i32, i32, i32)> = row
            .split(':')
            .last()
            .unwrap()
            .split(';')
            .map(|x| {
                let numbers: Vec<&str> = x.split(' ').skip(1).collect();

                let red = get_color_value_from_draw(&numbers, "red");
                let blue = get_color_value_from_draw(&numbers, "blue");
                let green = get_color_value_from_draw(&numbers, "green");

                (red, blue, green)
            })
            .collect();

        Game { id, draws }
    }
}

pub fn run() {
    let file = fs::read_to_string("src/inputs/day2.txt").unwrap();

    let games: Vec<Game> = file.lines().map(Game::new).collect();

    let part1: i32 = games
        .iter()
        .filter(|game| game.is_possible((12, 14, 13)))
        .map(|game| game.id)
        .sum();

    let part2: i32 = games.iter().map(|game| game.power_of_cubes()).sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
