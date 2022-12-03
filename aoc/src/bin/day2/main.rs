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

        // println!(
        //     "{:?}",
        //     get_score_for_round(opponent_choice, proposed_choice)
        // );

        total_sum = total_sum + get_score_for_round(opponent_choice, proposed_choice);
    }

    println!("Total Sum: {:?}", total_sum);
}

fn get_score_for_round(opponent_choice: &str, proposed_choice: &str) -> i32 {
    let mut score = 0;
    println!("Choice 1: {:?}", opponent_choice);
    println!("Choice 2: {:?}", proposed_choice);
    println!(
        "Generated Choice: {:?}",
        convert_player_b_choice_task2(opponent_choice, proposed_choice)
    );

    let generated_choice = convert_player_b_choice_task2(opponent_choice, proposed_choice);
    score = score
        + calculate_rock_paper_scissors_result(opponent_choice, generated_choice)
        + get_choice_score(generated_choice);

    score
}

fn convert_player_b_choice(choice: &str) -> &str {
    match choice {
        "X" => "A",
        "Y" => "B",
        "Z" => "C",
        _ => "A",
    }
}

fn convert_player_b_choice_task2<'a>(choice_a: &'a str, choice_b: &'a str) -> &'a str {
    match choice_b {
        "X" => match choice_a {
            "A" => "C",
            "B" => "A",
            "C" => "B",
            _ => "A",
        },
        "Y" => choice_a.clone(),
        "Z" => match choice_a {
            "A" => "B",
            "B" => "C",
            "C" => "A",
            _ => "A",
        },
        _ => "A",
    }
}

fn get_choice_score(choice: &str) -> i32 {
    match choice {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0,
    }
}
fn calculate_rock_paper_scissors_result(choice_a: &str, choice_b: &str) -> i32 {
    if choice_a.eq(choice_b) {
        return 3;
    }

    if (choice_a.eq("A") && choice_b.eq("B"))
        || (choice_a.eq("B") && choice_b.eq("C"))
        || choice_a.eq("C") && choice_b.eq("A")
    {
        return 6;
    } else {
        return 0;
    }
}
