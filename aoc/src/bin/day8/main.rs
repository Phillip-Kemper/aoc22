use std::{fs, process::exit};

fn main() {
    let file_path = "day8/day8.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let rows: Vec<&str> = contents.split("\n").collect();
    let columns: Vec<String> = (0..rows.first().unwrap().len())
        .map(|index| {
            let mut column: String = String::from("");
            for row in rows.iter() {
                column.push(row.chars().nth(index).unwrap());
            }
            column
        })
        .collect();

    let max_viewing_distance = check_max_viewing_distance(rows.clone(), columns.clone());
    println!("Max distance {:?}", max_viewing_distance);

    // let mut visible_trees = check_forest(rows.clone(), columns.clone());

    // println!("Visible Trees: {:?} ", visible_trees);
}

fn is_visible_from_side(row: &str, index: usize, tree_height: u32) -> bool {
    println!("String {:?}", row);
    println!("Index {:?}", index);
    for tree in row.chars().take(index) {
        //println!("tree checked: {:?}", tree);
        if (tree as u32) >= tree_height {
            return false;
        }
    }
    true
}

fn check_forest(rows: Vec<&str>, columns: Vec<String>) -> i32 {
    let mut visible_trees = 0;
    for (row_index, row) in rows.iter().enumerate() {
        for (column_index, tree_height) in row.clone().chars().enumerate() {
            println!(
                "Checking Column: {:?} Row: {:?}, Tree: {:?}",
                column_index, row_index, tree_height
            );
            if row_index == 0
                || column_index == 0
                || row.len() - row_index == 1
                || row.len() - column_index == 1
            {
                println!("Edge");
                visible_trees += 1;
            } else if is_visible_from_side(row.clone(), column_index, tree_height as u32) {
                // is visible from side
                println!("Left side");
                visible_trees += 1;
            } else if is_visible_from_side(&*columns[column_index], row_index, tree_height as u32) {
                // is visible from top
                visible_trees += 1;
                println!("Top side");
            } else if is_visible_from_side(
                &*(row.chars().rev().collect::<String>()),
                row.len() - column_index - 1,
                tree_height as u32,
            ) {
                visible_trees += 1;
                println!("Right side");
            } else if is_visible_from_side(
                &*(columns[column_index].chars().rev().collect::<String>()),
                row.len() - row_index - 1,
                tree_height as u32,
            ) {
                println!("Bottom side");
                visible_trees += 1;
            }
        }
    }
    visible_trees
}

fn check_max_viewing_distance(rows: Vec<&str>, columns: Vec<String>) -> u32 {
    let mut max_distance = 0;
    for (row_index, row) in rows.iter().enumerate().skip(1) {
        for (column_index, tree_height) in row.clone().chars().enumerate().skip(1) {
            println!(
                "Checking Column: {:?} Row: {:?}, Tree: {:?}",
                column_index, row_index, tree_height
            );

            let mut viewing_distance = 1;
            //get viewing distance

            println!("{:?}", row);
            viewing_distance *=
                get_viewing_distance(row, column_index, tree_height.to_digit(10).unwrap());

            println!("{:?}", columns[column_index]);
            viewing_distance *= get_viewing_distance(
                &*columns[column_index],
                row_index,
                tree_height.to_digit(10).unwrap(),
            );

            println!("{:?}", row.chars().rev().collect::<String>());
            viewing_distance *= get_viewing_distance(
                &*(row.chars().rev().collect::<String>()),
                row.len() - column_index - 1,
                tree_height.to_digit(10).unwrap(),
            );

            println!(
                "{:?}",
                &*columns[column_index].chars().rev().collect::<String>()
            );
            viewing_distance *= get_viewing_distance(
                &*((&*columns[column_index]).chars().rev().collect::<String>()),
                row.len() - row_index - 1,
                tree_height.to_digit(10).unwrap(),
            );

            if viewing_distance > max_distance {
                println!("yes");
                max_distance = viewing_distance;
            }
        }
    }
    max_distance
}

fn get_viewing_distance(row: &str, index: usize, value: u32) -> u32 {
    println!("{:?}", value);
    println!("{:?}", index);
    let mut viewing_distance = 0;
    if (index == row.len()) {
        return 0;
    }

    let mut iteri = row.chars().skip(index + 1).into_iter().peekable();
    while let Some(el) = iteri.next() {
        viewing_distance += 1;
        if value <= (el.to_digit(10).unwrap()) {
            println!("nop");
            break;
        }
    }

    println!("viewing distance: {:?}", viewing_distance);
    viewing_distance
}
