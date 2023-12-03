use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Part {
    start_index: usize,
    end_index: usize,
    row_index: usize,
    number: i32,
}

#[derive(Debug)]
struct Symbol {
    index: usize,
    symbol: char,
}

fn check_row(part: &Part, symbols: &Vec<Vec<Symbol>>, row_index: usize) -> bool {
    let symbol_before_part = symbols[row_index]
        .iter()
        .find(|x| x.index == part.start_index.checked_sub(1).unwrap_or(0));

    let symbol_after_part = symbols[row_index]
        .iter()
        .find(|x| x.index == part.end_index + 1);

    let symbol_inside_part = symbols[row_index]
        .iter()
        .find(|x| x.index <= part.end_index && x.index >= part.start_index);

    symbol_before_part.is_some() || symbol_after_part.is_some() || symbol_inside_part.is_some()
}

fn is_symbol_adjacent(part: &Part, symbols: &Vec<Vec<Symbol>>) -> bool {
    let same_row = check_row(part, &symbols, part.row_index);
    let row_above = check_row(part, &symbols, part.row_index.checked_sub(1).unwrap_or(0));
    let row_below_index = (part.row_index + 1).clamp(0, symbols.len() - 1);
    let row_below = check_row(part, &symbols, row_below_index);

    vec![same_row, row_above, row_below]
        .iter()
        .any(|x| x == &true)
}

fn row_value(row: &Vec<Part>, symbols: &Vec<Vec<Symbol>>) -> i32 {
    row.iter()
        .filter(|part| is_symbol_adjacent(part, symbols))
        .map(|part| part.number)
        .sum()
}

fn check_row_part_2(symbol: &Symbol, parts: &Vec<Vec<Part>>, row_index: usize) -> Vec<i32> {
    let value_before_symbol = parts[row_index]
        .iter()
        .find(|part| part.end_index == symbol.index - 1)
        .map(|x| x.number)
        .unwrap_or_else(|| 0);

    let value_after_symbol = parts[row_index]
        .iter()
        .find(|part| part.start_index == symbol.index + 1)
        .map(|x| x.number)
        .unwrap_or_else(|| 0);

    let symbol_inside_part = parts[row_index]
        .iter()
        .find(|part| symbol.index <= part.end_index && symbol.index >= part.start_index)
        .map(|x| x.number)
        .unwrap_or_else(|| 0);

    vec![value_before_symbol, value_after_symbol, symbol_inside_part]
}

fn row_gear_ratio(row: &Vec<Symbol>, parts: &Vec<Vec<Part>>, row_index: usize) -> i32 {
    row.iter()
        .filter(|symbol| symbol.symbol == '*')
        .map(|symbol| {
            let same_row = check_row_part_2(symbol, &parts, row_index);
            let row_above = check_row_part_2(symbol, &parts, row_index.checked_sub(1).unwrap_or(0));
            let row_below_index = (row_index + 1).clamp(0, parts.len() - 1);
            let row_below = check_row_part_2(symbol, &parts, row_below_index);

            let adjacent_values: Vec<i32> = same_row
                .iter()
                .chain(&row_above)
                .chain(&row_below)
                .cloned()
                .filter(|x| x != &0)
                .collect();

            if adjacent_values.len() == 2 {
                return adjacent_values[0] * adjacent_values[1];
            }

            0
        })
        .sum()
}

pub fn run() {
    let file = fs::read_to_string("src/inputs/day3.txt").unwrap();

    let rows: Vec<&str> = file.lines().collect();

    let rows_parts: Vec<Vec<Part>> = rows
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            let rgx = Regex::new(r"(\d*)").unwrap();

            rgx.find_iter(row)
                .flat_map(|x| {
                    if x.is_empty() == false {
                        let part = Part {
                            start_index: x.start(),
                            end_index: x.end() - 1,
                            number: x.as_str().parse::<i32>().unwrap(),
                            row_index,
                        };

                        return Some(part);
                    }

                    None
                })
                .collect()
        })
        .collect();

    let rows_symbols: Vec<Vec<Symbol>> = rows
        .iter()
        .map(|row| {
            row.chars()
                .enumerate()
                .filter(|(_, symbol)| symbol.is_numeric() == false && symbol != &'.')
                .map(|(index, symbol)| Symbol { symbol, index })
                .collect()
        })
        .collect();

    let part1: i32 = rows_parts
        .iter()
        .map(|parts| row_value(parts, &rows_symbols))
        .sum();

    let part2: i32 = rows_symbols
        .iter()
        .enumerate()
        .map(|(row_index, row)| row_gear_ratio(row, &rows_parts, row_index))
        .sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
