use std::{collections::HashSet, fs};

#[derive(Debug, Clone)]
struct Card {
    id: i32,
    winning_numbers: HashSet<i32>,
    raffle_numbers: HashSet<i32>,
}

impl Card {
    fn matches(&self) -> HashSet<&i32> {
        self.raffle_numbers
            .intersection(&self.winning_numbers)
            .collect::<HashSet<&i32>>()
    }

    fn matches_count(&self) -> usize {
        self.matches().len()
    }

    fn points(&self) -> i32 {
        match self.matches_count() {
            0 => 0,
            count => 1 * (2 as i32).pow((count - 1) as u32),
        }
    }
}

fn cards_from_rows(rows: Vec<&str>) -> Vec<Card> {
    rows.iter()
        .map(|row| {
            let id: i32 = row
                .split(":")
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let numbers: Vec<Vec<i32>> = row
                .split(":")
                .skip(1)
                .next()
                .unwrap()
                .split("|")
                .map(|nums| {
                    nums.trim()
                        .split(" ")
                        .filter_map(|x| x.parse::<i32>().ok())
                        .collect::<Vec<i32>>()
                })
                .collect();

            Card {
                id,
                winning_numbers: numbers[0].clone().into_iter().collect(),
                raffle_numbers: numbers[1].clone().into_iter().collect(),
            }
        })
        .collect()
}

fn card_winnings<'a>(card: &Card, all_cards: &'a Vec<Card>) -> Vec<&'a Card> {
    let count = card.matches_count();
    let start_id: usize = (card.id as usize) + 1;

    (start_id..start_id + count)
        .map(|id| all_cards.iter().find(|x| x.id as usize == id).unwrap())
        .collect()
}

pub fn run() {
    let file = fs::read_to_string("src/inputs/day4.txt").unwrap();
    let rows: Vec<&str> = file.lines().collect();
    let cards: Vec<Card> = cards_from_rows(rows);

    let part1: i32 = cards.iter().map(|card| card.points()).sum();
    println!("Part 1: {}", part1);

    let won_cards: Vec<&Card> = cards
        .iter()
        .map(|card| {
            let original: Vec<&Card> = card_winnings(card, &cards);

            let mut copies: Vec<&Card> = vec![];
            let mut previous_iteration: Vec<&Card> = original.clone();

            while previous_iteration.len() != 0 {
                let mut temp: Vec<&Card> = previous_iteration
                    .iter()
                    .map(|card| card_winnings(card, &cards))
                    .flatten()
                    .collect();

                previous_iteration = temp.clone();
                copies.append(&mut temp);
            }

            original
                .iter()
                .chain(&copies)
                .cloned()
                .collect::<Vec<&Card>>()
        })
        .flatten()
        .collect();

    let part2 = won_cards.len() + cards.len();

    println!("Part 2: {}", part2);
}
