use std::fs;

#[derive(Debug)]
struct ConversionRange {
    dst_range_start: u64,
    src_range_start: u64,
    range_length: u64,
}

impl ConversionRange {
    fn convert(&self, value: u64) -> Option<u64> {
        if value >= self.src_range_start && value <= self.src_range_start + self.range_length {
            let a = value - self.src_range_start;

            return Some(self.dst_range_start + a);
        }

        None
    }
}

#[derive(Debug)]
struct Map {
    source_category: String,
    destination_category: String,
    conversion_ranges: Vec<ConversionRange>,
}

impl Map {
    fn convert(&self, value: u64) -> u64 {
        let a: Vec<u64> = self
            .conversion_ranges
            .iter()
            .flat_map(|range| range.convert(value))
            .collect();

        if a.len() == 0 {
            return value;
        }

        a[0]
    }
}

fn parse_seeds(first_row: &str) -> Vec<u64> {
    first_row
        .split(":")
        .last()
        .unwrap()
        .split(" ")
        .flat_map(|x| x.parse::<u64>().ok())
        .collect()
}

fn parse_maps(file: &String) -> Vec<Map> {
    file.split("\r\n\r")
        .skip(1)
        .map(|x| {
            let source_category = x.split("-").next().unwrap().replace("\n", "");
            let destination_category = x.split("to-").last().unwrap().split(" ").next().unwrap();

            let conversion_ranges: Vec<ConversionRange> = x
                .split("map:")
                .skip(1)
                .map(|z: &str| {
                    z.lines()
                        .filter(|x| !x.is_empty())
                        .map(|nums| {
                            let conversion_numbers: Vec<u64> = nums
                                .split(" ")
                                .flat_map(|num| num.parse::<u64>().ok())
                                .collect();

                            let dst_range_start = conversion_numbers[0];
                            let src_range_start = conversion_numbers[1];
                            let range_length = conversion_numbers[2];

                            ConversionRange {
                                dst_range_start,
                                src_range_start,
                                range_length,
                            }
                        })
                        .collect::<Vec<ConversionRange>>()
                })
                .flatten()
                .collect();

            Map {
                source_category,
                destination_category: destination_category.to_string(),
                conversion_ranges,
            }
        })
        .collect()
}

fn run_seed(seed: u64, maps: &Vec<Map>, source_category: &str) -> u64 {
    let map = maps.iter().find(|x| x.source_category == source_category);

    match map {
        Some(m) => {
            let value = m.convert(seed);

            run_seed(value, &maps, &m.destination_category)
        }
        None => seed,
    }
}

pub fn run() {
    let file = fs::read_to_string("src/inputs/day5.txt").unwrap();
    let rows: Vec<&str> = file.lines().collect();

    let seeds: Vec<u64> = parse_seeds(rows[0]);

    let maps: Vec<Map> = parse_maps(&file);

    let part1 = seeds
        .iter()
        .map(|seed| run_seed(*seed, &maps, "seed"))
        .min()
        .unwrap();

    println!("Part 1: {}", part1);
    //println!("Part 2: {}", 0);
}
