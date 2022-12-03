use std::fs;
use substring::Substring;

fn main() {
    let file_path = "day3/day3.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let backpacks: Vec<&str> = contents.split("\n").collect();

    let mut total_sum = 0;

    for bag in backpacks.into_iter() {
        let first_compartment = bag.substring(0, bag.len() / 2);
        let second_compartment = bag.substring(bag.len() / 2, bag.len());

        println!("First Compartment: {:?}", first_compartment);
        println!("2nd Compartment: {:?}", second_compartment);

        let mut char_vec: Vec<char> = Vec::new();

        for char_1 in first_compartment.chars() {
            for char_2 in second_compartment.chars() {
                if char_1.eq(&char_2) && !char_vec.contains(&char_1) {
                    total_sum = total_sum + get_priority_from_char(char_1);
                    char_vec.push(char_1.clone());
                }
            }
        }
    }
    println!("Total priority sum: {:?}", total_sum);
}

fn get_priority_from_char(char_1: char) -> u32 {
    println!("{:?}", char_1);
    if char_1.is_uppercase() {
        char_1 as u32 - 38
    } else {
        char_1 as u32 - 96
    }
}
