use std::fs;

fn main() {
    let file_path = "day4/day4.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let rows: Vec<&str> = contents.split("\n").collect();

    let result: Vec<i32> = rows
        .clone()
        .into_iter()
        .map(|row| {
            let parts: Vec<&str> = row.split(",").collect();
            let first_part_parsed = parse_range(parts.first().unwrap());
            let second_part_parsed = parse_range(parts.last().unwrap());

            if first_part_parsed.0 <= second_part_parsed.0
                && first_part_parsed.1 >= second_part_parsed.1
            {
                return 1;
            }
            if second_part_parsed.0 <= first_part_parsed.0
                && second_part_parsed.1 >= first_part_parsed.1
            {
                return 1;
            }
            return 0;
        })
        .collect();

    let result_overlapped: Vec<i32> = rows
        .into_iter()
        .map(|row| {
            let parts: Vec<&str> = row.split(",").collect();
            let first_part_parsed = parse_range(parts.first().unwrap());
            let second_part_parsed = parse_range(parts.last().unwrap());

            if first_part_parsed.0 <= second_part_parsed.0
                && first_part_parsed.1 >= second_part_parsed.0
            {
                return 1;
            }
            if second_part_parsed.0 <= first_part_parsed.0
                && second_part_parsed.1 >= first_part_parsed.0
            {
                return 1;
            }
            return 0;
        })
        .collect();

    let sum: i32 = result.iter().sum();
    let sum_overlapped: i32 = result_overlapped.iter().sum();
    println!("Sum: {:?}", sum);
    println!("Sum Overlapped: {:?}", sum_overlapped);
}

fn parse_range(range_text: &str) -> (u32, u32) {
    let split_numbers: Vec<&str> = range_text.split('-').collect();
    let first: u32 = split_numbers.first().unwrap().parse().unwrap();
    let second: u32 = split_numbers.last().unwrap().parse().unwrap();
    (first, second)
}
