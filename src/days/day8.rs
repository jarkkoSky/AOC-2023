use chrono::prelude::*;
use std::{fs, time::Instant};

struct Node(String, String, String);

fn parse_nodes(rows: &Vec<&str>) -> Vec<Node> {
    rows.iter()
        .skip(2)
        .map(|row| {
            let id = row.split("=").next().unwrap().trim();
            let elements: Vec<String> = row
                .split("(")
                .skip(1)
                .next()
                .unwrap()
                .split(",")
                .map(|x| x.trim().replace(")", ""))
                .collect();

            Node(id.to_string(), elements[0].clone(), elements[1].clone())
        })
        .collect()
}

/*
fn run_map(
    starting_node: String,
    ending_nodes: &Vec<String>,
    nodes: &Vec<Node>,
    instructions: &Vec<char>,
) -> i32 {
    let mut instruction_index = 0;
    let mut steps = 0;
    let mut current_id = starting_node.clone();

    while ending_nodes.contains(&current_id) == false {
        let current_node = nodes.iter().find(|x| x.id == current_id).unwrap();
        let destination = current_node.get_element(instructions[instruction_index]);

        steps += 1;
        current_id = destination;

        if instruction_index == instructions.len() - 1 {
            instruction_index = 0;
        } else {
            instruction_index += 1;
        }
    }

    dbg!(&starting_node, steps);

    steps
}*/

fn get_node_with_id<'a>(nodes: &'a Vec<Node>, id: &'a String) -> &'a Node {
    nodes.iter().find(|x| x.0.eq(id)).unwrap()
}

fn run_map_2(starting_nodes: &Vec<&Node>, nodes: &Vec<Node>, instructions: &Vec<char>) -> u64 {
    let mut instruction_index = 0;
    let mut steps: u64 = 0;
    let mut current_nodes: Vec<_> = starting_nodes.clone();
    let mut stop = false;

    let mut now = Instant::now();

    while !stop {
        if steps % 1000000 == 0 {
            let elapsed = now.elapsed();
            println!(
                "Steps: {}M , time: {:?} seconds",
                steps / 1000000,
                elapsed.as_secs_f64()
            );
            now = Instant::now();
        }

        let mut temp: Vec<&Node> = vec![];

        for c in current_nodes {
            let element = match instructions[instruction_index] {
                'L' => &c.1,
                _ => &c.2,
            };

            let a = get_node_with_id(nodes, element);

            temp.push(a);
        }

        steps += 1;
        current_nodes = temp;

        if instruction_index == instructions.len() - 1 {
            instruction_index = 0;
        } else {
            instruction_index += 1;
        }

        stop = current_nodes.iter().all(|x| x.0.ends_with("Z"));
    }

    steps
}

pub fn run() {
    let local: DateTime<Local> = Local::now();

    println!("Start - {}", local.format("%Y-%m-%d %H:%M:%S").to_string());

    let file = fs::read_to_string("src/inputs/day8.txt").unwrap();
    let rows: Vec<&str> = file.lines().collect();

    let instructions: Vec<char> = rows[0].chars().collect();
    let nodes: Vec<Node> = parse_nodes(&rows);

    /*let part1 = run_map(
        String::from("AAA"),
        &vec![String::from("ZZZ")],
        &nodes,
        &instructions,
    );*/

    let starting_nodes: Vec<&Node> = nodes.iter().filter(|x| x.0.ends_with("A")).collect();

    let part2 = run_map_2(&starting_nodes, &nodes, &instructions);

    //println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
