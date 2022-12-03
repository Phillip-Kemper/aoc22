use std::fs;

fn main() {
    let file_path = "day3/day3.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let backpacks: Vec<&str> = contents.split("\n").collect();

    let mut total_sum = 0;

    for bag in backpacks.into_iter() {
        let first_compartment = bag.to_string().substring(0, 3);
    }
}
