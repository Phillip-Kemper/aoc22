//use std::env;
use std::fs;

fn main() {
    let file_path = "src/day1.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let chunks: Vec<&str> = contents.split("\n\n").collect();

    let mut max = 0;

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
    }

    println!("Max: {:?}", max);
}
