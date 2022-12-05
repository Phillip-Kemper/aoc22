use std::fs;

fn main() {
    let file_path = "day5/day5.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let rows: Vec<&str> = contents.split("\n").collect();
}
