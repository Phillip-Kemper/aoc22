use std::{collections::HashMap, fs, hash::Hash, process::exit, thread, vec};

use substring::Substring;

fn main() {
    let num: u64 = 900_000;

    thread::Builder::new()
        .stack_size(num as usize * 0xFF)
        .spawn(move || {
            let file_path = "day7/day7.txt";

            let contents =
                fs::read_to_string(file_path).expect("Should have been able to read the file");
            let rows: Vec<&str> = contents.split("\n").collect();
            let mut iter_rows = rows.into_iter().peekable();

            let mut dirs: Vec<&str> = vec![];
            let mut directory_sizes: HashMap<&str, i32> = HashMap::new();
            let mut directory_structure: HashMap<&str, Vec<&str>> = HashMap::new();

            let mut current_dir_index = "";

            while let Some(row) = iter_rows.next() {
                if row.starts_with("$ cd") {
                    let dir = row.substring(5, row.len());
                    if dir.ne("..") {
                        dirs.push(dir);
                        if let None = directory_sizes.get(dir) {
                            directory_sizes.insert(dir, 0);
                        }
                        current_dir_index = dir;
                        // println!("Current dir: {:?}", dir);
                    } else {
                        current_dir_index = dir;
                        //println!("new index {:?}", dir);
                    }
                }
                if row.starts_with("$ ls") {
                    let current_dir = current_dir_index;
                    //println!("Content of dir {:?}:", current_dir);
                    //   for ls_row in iter_rows {

                    while let Some(ls_row) = iter_rows.peek() {
                        if ls_row.starts_with("$") {
                            break;
                        } else if ls_row.starts_with("dir") {
                            // dir a is below the current one
                            let sub_dir: &str =
                                ls_row.split(" ").collect::<Vec<&str>>().last().unwrap();

                            //println!("subdir {:?}", sub_dir);
                            println!("{:?}", directory_structure.get(current_dir));
                            println!("{:?}", directory_structure.get(current_dir));
                            directory_structure
                                .entry(current_dir)
                                .and_modify(|e| {
                                    e.push(sub_dir);
                                    //e.retain(|&x| x.ne(current_dir));
                                })
                                .or_insert(vec![sub_dir]);
                            println!("{:?}", directory_structure.get(current_dir).unwrap());

                            // println!("{:?}", directory_structure);
                            // exit(1);
                            // match directory_structure.get(current_dir) {
                            //     Some(d) => {
                            //         directory_structure.insert(
                            //             current_dir,
                            //             directory_structure[current_dir].push(sub_dir),
                            //         );
                            //     }
                            //     None => {
                            //         directory_structure.insert(current_dir, vec![sub_dir]);
                            //     }
                            // };
                            iter_rows.next().unwrap();
                        } else {
                            let nr: &str = iter_rows
                                .next()
                                .unwrap()
                                .split(" ")
                                .collect::<Vec<&str>>()
                                .first()
                                .unwrap();

                            let nr_parse: i32 = nr.parse::<i32>().unwrap();
                            *directory_sizes.get_mut(current_dir).unwrap() += nr_parse;
                            //println!("nr: {:?}", nr_parse);
                        }
                    }
                }
            }

            //println!("All dirs: {:?}", directory_sizes);
            //println!("All dirs: {:?}", dirs);

            // for (key,value) in directory_structure.into_iter().enumerate(){
            //     directory_structure.
            // }

            let mut above_100k: Vec<i32> = vec![];
            println!("dirs length {:?}", dirs.len());
            dirs.sort_unstable();
            dirs.dedup();
            for key in dirs.clone().into_iter().take(1) {
                let sum = get_sum_of_all_sub_dirs(
                    key,
                    directory_sizes.clone(),
                    directory_structure.clone(),
                );
                if sum <= 100000 {
                    above_100k.push(sum);
                    println!("Total sum for dir {:?}: {:?}", key, sum);
                }
            }
            println!("Sum {:?}", above_100k.iter().sum::<i32>());
        })
        .unwrap()
        .join();
}

fn get_sum_of_all_sub_dirs(
    dir: &str,
    dir_size: HashMap<&str, i32>,
    dir_map: HashMap<&str, Vec<&str>>,
) -> i32 {
    println!("1");
    println!("{:?}", dir);
    if dir_map.clone().get(dir).is_some() {
        let mut subdirs = dir_map.get(dir).unwrap().to_owned();
        println!("{:?}", subdirs);
        println!("Sub ngn: {:?}", dir_map.get("ngn").unwrap());
        println!("2");
        exit(1);
        if subdirs.contains(&dir) {
            print!("OH!!");
        }
        subdirs.sort_unstable();
        subdirs.dedup();
        println!("{:?}", subdirs);
        let sums: Vec<i32> = subdirs
            .into_iter()
            .map(|sub| {
                return get_sum_of_all_sub_dirs(sub, dir_size.clone(), dir_map.clone());
            })
            .collect();
        let sum: i32 = sums.iter().sum();
        sum + *dir_size.get(dir).unwrap()
        //5
    } else {
        //println!("dir: {:?}", dir);
        return *dir_size.get(dir).unwrap();
    }
}
