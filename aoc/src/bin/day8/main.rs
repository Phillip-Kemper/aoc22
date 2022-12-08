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

    // println!(
    //     "Rows: {:?}",
    //     columns
    //         .clone()
    //         .into_iter()
    //         .map(|str| { str.chars().rev().collect() })
    //         .collect::<Vec<String>>()
    // );
    // exit(1);

    let mut visible_trees = check_forest(rows.clone(), columns.clone());

    println!("Visible Trees: {:?} ", visible_trees);
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
