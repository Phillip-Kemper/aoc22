//use std::env;
use std::{env, fs};

fn main() {
    let file_path = "day1/day1.txt";

    println!("{:?}", env::current_dir());
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let chunks: Vec<&str> = contents.split("\n\n").collect();

    let mut max = 0;
    let mut max_array: [i32; 3] = [0; 3];

    for chunk in chunks.into_iter() {
        let energies: Vec<&str> = chunk.split("\n").collect();
        println!("{:?}", energies);
        let mut sum = 0;
        for number in energies.into_iter() {
            let num: i32 = number.parse().unwrap();
            sum = sum + num;
        }
        if sum > max {
            max = sum;
        }

        let current_min = max_array.into_iter().min().unwrap();
        if sum > current_min {
            let index = max_array.iter().position(|&r| r == current_min).unwrap();
            max_array[index] = sum;
        }
    }

    let total_sum: i32 = max_array.iter().sum();
    println!("Max: {:?}", max_array);
    println!("Max 3 Sum: {:?}", total_sum)
}
