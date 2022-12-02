//use std::env;
use std::fs;

fn main() {
    let file_path = "day2/day2.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let rounds: Vec<&str> = contents.split("\n").collect();

    let mut total_sum = 0;

    for round in rounds.iter() {
        let choices: Vec<&str> = round.split(" ").collect();

        let opponent_choice = choices.first().unwrap();
        let proposed_choice = choices.last().unwrap();
        println!("Choice 1: {:?}", opponent_choice);
        println!("Choice 2: {:?}", proposed_choice);
    }

    // for chunk in chunks.into_iter() {
    //     let energies: Vec<&str> = chunk.split("\n").collect();
    //     println!("{:?}", energies);
    //     let mut sum = 0;
    //     for number in energies.into_iter() {
    //         let num: i32 = number.parse().unwrap();
    //         sum = sum + num;
    //     }
    //     if sum > max {
    //         max = sum;
    //     }

    //     let current_min = max_array.into_iter().min().unwrap();
    //     if sum > current_min {
    //         let index = max_array.iter().position(|&r| r == current_min).unwrap();
    //         max_array[index] = sum;
    //     }
    // }

    // let total_sum: i32 = max_array.iter().sum();
    // println!("Max: {:?}", max_array);
    // println!("Max 3 Sum: {:?}", total_sum)
}
