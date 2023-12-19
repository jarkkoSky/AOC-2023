use std::fs;

#[derive(PartialEq, Debug, Clone, Copy)]
enum DIRECTION {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

const DIRECTIONS: [DIRECTION; 4] = [
    DIRECTION::NORTH,
    DIRECTION::EAST,
    DIRECTION::SOUTH,
    DIRECTION::WEST,
];

fn value_at_position(
    grid: &Vec<Vec<char>>,
    position: (usize, usize),
) -> Option<(char, (usize, usize))> {
    match grid.get(position.1)?.get(position.0) {
        Some(l) => Some((*l, position)),
        None => None,
    }
}

fn position_at_direction(
    position: (usize, usize),
    direction: &DIRECTION,
) -> Option<(usize, usize)> {
    match direction {
        DIRECTION::WEST => {
            return Some((position.0 + 1, position.1));
        }
        DIRECTION::EAST => {
            if position.0 == 0 {
                return None;
            }

            return Some((position.0 - 1, position.1));
        }
        DIRECTION::SOUTH => {
            return Some((position.0, position.1 + 1));
        }
        DIRECTION::NORTH => {
            if position.1 == 0 {
                return None;
            }

            return Some((position.0, position.1 - 1));
        }
    }
}

fn value_at_direction(
    grid: &Vec<Vec<char>>,
    position: (usize, usize),
    direction: &DIRECTION,
) -> Option<(char, (usize, usize))> {
    match position_at_direction(position, direction) {
        Some(position) => value_at_position(grid, position),
        None => None,
    }
}

fn surrounding_values(
    grid: &Vec<Vec<char>>,
    position: (usize, usize),
) -> Vec<(char, (usize, usize), DIRECTION)> {
    DIRECTIONS
        .iter()
        .flat_map(
            |direction| match value_at_direction(&grid, position, direction) {
                Some(x) => return Some((x.0, x.1, *direction)),
                None => None,
            },
        )
        .collect()
}

fn is_char_any_of(options: Vec<char>, target: char) -> bool {
    options.iter().any(|x| *x == target)
}

fn is_possible_direction(
    target: &(char, (usize, usize), DIRECTION),
    current_point: &(char, (usize, usize)),
) -> bool {
    match current_point.0 {
        'S' => {
            if is_char_any_of(vec!['F', '7', '|'], target.0) && target.2 == DIRECTION::NORTH {
                return true;
            }

            if is_char_any_of(vec!['|', 'L', 'J'], target.0) && target.2 == DIRECTION::SOUTH {
                return true;
            }

            if is_char_any_of(vec!['-', 'L', 'F'], target.0) && target.2 == DIRECTION::EAST {
                return true;
            }

            if is_char_any_of(vec!['-', '7', 'J'], target.0) && target.2 == DIRECTION::WEST {
                return true;
            }

            return false;
        }
        '|' => {
            if is_char_any_of(vec!['F', '7', '|'], target.0) && target.2 == DIRECTION::NORTH {
                return true;
            }

            if is_char_any_of(vec!['J', 'L', '|'], target.0) && target.2 == DIRECTION::SOUTH {
                return true;
            }

            return false;
        }
        '-' => {
            if is_char_any_of(vec!['J', '7', '-'], target.0) && target.2 == DIRECTION::WEST {
                return true;
            }

            if is_char_any_of(vec!['L', 'F', '-'], target.0) && target.2 == DIRECTION::EAST {
                return true;
            }

            return false;
        }
        'L' => {
            if is_char_any_of(vec!['|', '7', 'F'], target.0) && target.2 == DIRECTION::NORTH {
                return true;
            }

            if is_char_any_of(vec!['-', 'J', '7'], target.0) && target.2 == DIRECTION::WEST {
                return true;
            }

            return false;
        }
        'J' => {
            if is_char_any_of(vec!['|', '7', 'F'], target.0) && target.2 == DIRECTION::NORTH {
                return true;
            }

            if is_char_any_of(vec!['-', 'F', 'L'], target.0) && target.2 == DIRECTION::EAST {
                return true;
            }

            return false;
        }
        '7' => {
            if is_char_any_of(vec!['|', 'J', 'L'], target.0) && target.2 == DIRECTION::SOUTH {
                return true;
            }

            if is_char_any_of(vec!['-', 'L', 'F'], target.0) && target.2 == DIRECTION::EAST {
                return true;
            }

            return false;
        }
        'F' => {
            if is_char_any_of(vec!['|', 'J', 'L'], target.0) && target.2 == DIRECTION::SOUTH {
                return true;
            }

            if is_char_any_of(vec!['-', 'J', '7'], target.0) && target.2 == DIRECTION::WEST {
                return true;
            }

            return false;
        }
        _ => return false,
    }
}

pub fn run() {
    let grid: Vec<Vec<char>> = fs::read_to_string("src/inputs/day10.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start_pos: (usize, usize) = grid
        .iter()
        .enumerate()
        .flat_map(|(index, row)| match row.iter().position(|x| *x == 'S') {
            Some(p) => Some((p, index)),
            None => None,
        })
        .last()
        .unwrap();

    let start = ('S', start_pos);

    let mut tiles: Vec<(char, (usize, usize))> = vec![start];
    let mut stop = false;

    while stop == false {
        let current = tiles.last().unwrap();
        let possible: Vec<(char, (usize, usize))> = surrounding_values(&grid, current.1)
            .iter()
            .filter(|x| {
                let exists: bool = tiles.iter().any(|tile| tile.1 == x.1);

                is_possible_direction(&x, &current) && exists == false
            })
            .map(|x| (x.0, x.1))
            .collect();

        if possible.len() == 0 {
            stop = true;
            break;
        }

        let next_tile = possible.first().unwrap();

        tiles.push(next_tile.clone());
    }

    println!("Part 1: {}", tiles.len() / 2);

    // TBD Part 2, shoelace formula + picks theorem
}
