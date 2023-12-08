use num::integer::lcm;
use std::fs;

struct Node {
    id: String,
    elements: (String, String),
}

impl Node {
    fn get_element(&self, instruction: char) -> String {
        if instruction == 'L' {
            return self.elements.0.clone();
        }

        return self.elements.1.clone();
    }
}

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

            Node {
                id: id.to_string(),
                elements: (elements[0].clone(), elements[1].clone()),
            }
        })
        .collect()
}

fn run_map(
    starting_node: String,
    ending_node_pattern: String,
    nodes: &Vec<Node>,
    instructions: &Vec<char>,
) -> u64 {
    let mut instruction_index = 0;
    let mut steps = 0;
    let mut current_id = starting_node.clone();

    while current_id.ends_with(&ending_node_pattern) == false {
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

    steps
}

pub fn run() {
    let file = fs::read_to_string("src/inputs/day8.txt").unwrap();
    let rows: Vec<&str> = file.lines().collect();

    let instructions: Vec<char> = rows[0].chars().collect();
    let nodes: Vec<Node> = parse_nodes(&rows);

    let part1 = run_map(
        String::from("AAA"),
        String::from("ZZZ"),
        &nodes,
        &instructions,
    );

    let part2: u64 = nodes
        .iter()
        .filter(|x| x.id.ends_with("A"))
        .map(|x| run_map(x.id.clone(), String::from("Z"), &nodes, &instructions))
        .reduce(|acc, e| lcm(acc, e))
        .unwrap();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
