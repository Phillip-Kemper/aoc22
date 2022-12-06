use std::fs;

fn main() {
    let file_path = "day6/day6.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    for (index, _char) in contents.chars().enumerate() {
        let mut four_connected_chars = vec![];
        for i in index..index + 4 {
            four_connected_chars.push(contents.chars().nth(i).unwrap());
        }
        println!("{:?}", four_connected_chars);
        four_connected_chars.sort();
        four_connected_chars.dedup();
        if four_connected_chars.len() == 4 {
            println!("Starts at position: {:?}", index + 4);
            return;
        }
    }
}
