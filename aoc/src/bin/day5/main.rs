use std::{fs, vec};

fn main() {
    let file_path = "day5/day5.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let rows: Vec<&str> = contents.split("\n").collect();

    // demo
    //let mut initial_state: Vec<Vec<&str>> = vec![vec!["N", "Z"], vec!["D", "C", "M"], vec!["P"]];
    // actual
    let mut initial_state: Vec<Vec<&str>> = vec![
        vec!["Q", "G", "P", "R", "L", "C", "T", "F"],
        vec!["J", "S", "F", "R", "W", "H", "Q", "N"],
        vec!["Q", "M", "P", "W", "H", "B", "F"],
        vec!["F", "D", "T", "S", "V"],
        vec!["Z", "F", "V", "W", "D", "L", "Q"],
        vec!["S", "L", "C", "Z"],
        vec!["F", "D", "V", "M", "B", "Z"],
        vec!["B", "J", "T"],
        vec!["H", "P", "S", "L", "G", "B", "N", "Q"],
    ];

    for row in rows.iter() {
        let parts: Vec<&str> = row.split(" ").collect();
        let amount: usize = parts[1].parse().unwrap();
        let from: usize = parts[3].parse().unwrap();
        let to: usize = parts[5].parse().unwrap();

        println!("{:?}", amount);
        println!("{:?}", from);
        println!("{:?}", to);
        let temp_initial_state_from = initial_state[from - 1].clone();
        let temp_initial_state_to = initial_state[to - 1].clone();
        //initial_state[to - 1].extend(temp_initial_state_from.clone().into_iter().take(amount));

        initial_state[to - 1] = temp_initial_state_to
            .into_iter()
            .rev()
            .chain(temp_initial_state_from.clone().into_iter().take(amount))
            .rev()
            .collect();
        // .collect::<Vec<&str>>()
        // .into_iter()
        // .collect();

        initial_state[from - 1] = temp_initial_state_from
            .clone()
            .into_iter()
            .rev()
            .take(temp_initial_state_from.len() - amount)
            .rev()
            .collect();
    }

    for i in initial_state.iter() {
        println!("End Result: {:?}", i.first());
    }
}
