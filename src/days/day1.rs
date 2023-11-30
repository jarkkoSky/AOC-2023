use std::fs;

pub fn run() {
    let file_path = "src/inputs/day1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
}
