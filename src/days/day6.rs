#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Race {
        Race { time, distance }
    }

    fn is_winning_speed(&self, speed: &u64) -> bool {
        (self.time - speed) * speed > self.distance
    }

    fn winning_speeds_count(&self) -> usize {
        (1..self.time)
            .filter(|speed| self.is_winning_speed(&speed))
            .collect::<Vec<u64>>()
            .len()
    }
}

fn number_of_ways_to_win_multiplied(races: &Vec<Race>) -> usize {
    races
        .iter()
        .map(|race| race.winning_speeds_count())
        .reduce(|acc, e| acc * e)
        .unwrap()
}

pub fn run() {
    let races = vec![
        Race::new(56, 334),
        Race::new(71, 1135),
        Race::new(79, 1350),
        Race::new(99, 2430),
    ];

    let races_part2 = vec![Race::new(56717999, 334113513502430)];

    let part1 = number_of_ways_to_win_multiplied(&races);
    let part2 = number_of_ways_to_win_multiplied(&races_part2);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
