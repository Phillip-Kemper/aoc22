use std::fs;

fn main() {
    let file_path = "day6/day6.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let marker_length = 14;

    for (index, _char) in contents.chars().enumerate() {
        let mut four_connected_chars = vec![];
        for i in index..index + marker_length {
            four_connected_chars.push(contents.chars().nth(i).unwrap());
        }
        println!("{:?}", four_connected_chars);
        four_connected_chars.sort();
        four_connected_chars.dedup();
        if four_connected_chars.len() == marker_length {
            println!("Starts at position: {:?}", index + marker_length);
            return;
        }
    }
}
