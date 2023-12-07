use std::fs;

const CARDS_PART_1: [(char, i32); 13] = [
    ('A', 13),
    ('K', 12),
    ('Q', 11),
    ('J', 10),
    ('T', 9),
    ('9', 8),
    ('8', 7),
    ('7', 6),
    ('6', 5),
    ('5', 4),
    ('4', 3),
    ('3', 2),
    ('2', 1),
];

const CARDS_PART_2: [(char, i32); 13] = [
    ('A', 13),
    ('K', 12),
    ('Q', 11),
    ('T', 10),
    ('9', 9),
    ('8', 8),
    ('7', 7),
    ('6', 6),
    ('5', 5),
    ('4', 4),
    ('3', 3),
    ('2', 2),
    ('J', 0),
];

fn get_hand_type(cards: &Vec<i32>) -> i32 {
    let cards_grouped = &cards.iter().fold(vec![], |mut acc, card| {
        match acc.iter().position(|x: &Vec<&i32>| x[0] == card) {
            Some(pos) => acc[pos].push(card),
            None => acc.push(vec![card]),
        }

        acc
    });

    if cards_grouped.len() == 1 {
        return 7;
    }

    let jokers_count = match cards_grouped.iter().find(|x| x[0] == &0) {
        Some(jokers) => jokers.len(),
        None => 0,
    };

    let biggest_group = cards_grouped
        .iter()
        .filter(|x| x[0] != &0)
        .map(|group| group.len())
        .max()
        .unwrap()
        + jokers_count;

    let non_joker_groups = cards_grouped.iter().filter(|x| x[0] != &0).count();

    // Five of a kind
    if biggest_group == 5 {
        return 7;
    }
    // Four of a kind
    if biggest_group == 4 {
        return 6;
    }
    // Full house
    if non_joker_groups == 2 && biggest_group == 3 {
        return 5;
    }
    // Three of a kind
    if biggest_group == 3 {
        return 4;
    }
    // Two pair
    if non_joker_groups == 3 && biggest_group == 2 {
        return 3;
    }
    // One pair
    if biggest_group == 2 {
        return 2;
    }

    return 1;
}

#[derive(Debug)]
struct Hand {
    bid: u64,
    cards: Vec<i32>,
    hand_type: i32,
}

impl Hand {
    fn parse(row: &str, part: i32) -> Hand {
        let r: Vec<_> = row.split(" ").collect();

        let cards = r[0].chars().map(|c| parse_card(c, part)).collect();

        let bid = r[1].parse::<u64>().unwrap();
        let hand_type = get_hand_type(&cards);

        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}

fn parse_card(c: char, part: i32) -> i32 {
    if part == 1 {
        return CARDS_PART_1.iter().find(|x| x.0 == c).map(|x| x.1).unwrap();
    }

    CARDS_PART_2.iter().find(|x| x.0 == c).map(|x| x.1).unwrap()
}

fn result(rows: &Vec<&str>, part: i32) {
    let mut hands: Vec<Hand> = rows.iter().map(|row| Hand::parse(row, part)).collect();

    hands.sort_unstable_by_key(|item| {
        (
            item.hand_type,
            item.cards[0],
            item.cards[1],
            item.cards[2],
            item.cards[3],
            item.cards[4],
        )
    });

    let res: usize = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| {
            let rank = index + 1;

            (hand.bid as usize) * rank
        })
        .sum();

    println!("Part {}: {}", part, res);
}

pub fn run() {
    let file = fs::read_to_string("src/inputs/day7.txt").unwrap();
    let rows: Vec<&str> = file.lines().collect();

    result(&rows, 1);
    result(&rows, 2);
}
