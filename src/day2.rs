use crate::input_utils::read_input_lines;
use regex::Regex;

fn is_possible(blue: i32, red: i32, green: i32) -> bool {
    if blue > 14 || red > 12 || green > 13 {
        return false;
    }

    return true;
}

pub fn part1() {
    let input = read_input_lines("input/day2_part1.txt").expect("Could not read input file");

    let game_id_regex = Regex::new(r"\d+").unwrap();
    let blue_score_regex = Regex::new(r"(\d+) blue").unwrap();
    let red_score_regex = Regex::new(r"(\d+) red").unwrap();
    let green_score_regex = Regex::new(r"(\d+) green").unwrap();

    let possible_games = input.iter().map(|line| {
        let game_id = game_id_regex.find(line);

        let mut game_id_number: i32 = 0;
        let mut blue_score: i32 = 0;
        let mut red_score: i32 = 0;
        let mut green_score: i32 = 0;

        // Get game ID
        if let Some(matched) = game_id {
            let number_str = matched.as_str();
            match number_str.parse::<i32>() {
                Ok(number_value) => {
                    game_id_number = number_value;
                }
                Err(e) => println!("Failed to parse the number: {}", e),
            }
        }

        // Collect largest color count
        line.split(";").into_iter().for_each(|game_string| {
            let blue = blue_score_regex.captures(game_string);
            let red = red_score_regex.captures(game_string);
            let green = green_score_regex.captures(game_string);

            if let Some(matched_group) = blue {
                let number_str = matched_group.get(1).unwrap().as_str();
                match number_str.parse::<i32>() {
                    Ok(number_value) => {
                        if blue_score < number_value {
                            blue_score = number_value;
                        }
                    }
                    Err(e) => println!("Failed to parse the number: {}", e),
                }
            }

            if let Some(matched_group) = red {
                let number_str = matched_group.get(1).unwrap().as_str();
                match number_str.parse::<i32>() {
                    Ok(number_value) => {
                        if red_score < number_value {
                            red_score = number_value;
                        }
                    }
                    Err(e) => println!("Failed to parse the number: {}", e),
                }
            }

            if let Some(matched_group) = green {
                let number_str = matched_group.get(1).unwrap().as_str();
                match number_str.parse::<i32>() {
                    Ok(number_value) => {
                        if green_score < number_value {
                            green_score = number_value;
                        }
                    }
                    Err(e) => println!("Failed to parse the number: {}", e),
                }
            }
        });

        return (
            game_id_number,
            is_possible(blue_score, red_score, green_score),
        );
    });

    let result: i32 = possible_games
        .filter(|(_, is_possible)| *is_possible)
        .map(|(game_id, _)| game_id)
        .sum();

    println!("Result: {}", result);
}

pub fn part2() {
    let input = read_input_lines("input/day2_part2.txt").expect("Could not read input file");

    let game_id_regex = Regex::new(r"\d+").unwrap();
    let blue_score_regex = Regex::new(r"(\d+) blue").unwrap();
    let red_score_regex = Regex::new(r"(\d+) red").unwrap();
    let green_score_regex = Regex::new(r"(\d+) green").unwrap();

    let possible_games = input.iter().map(|line| {
        let game_id = game_id_regex.find(line);

        let mut game_id_number: i32 = 0;
        let mut blue_score: i32 = 0;
        let mut red_score: i32 = 0;
        let mut green_score: i32 = 0;

        // Get game ID
        if let Some(matched) = game_id {
            let number_str = matched.as_str();
            match number_str.parse::<i32>() {
                Ok(number_value) => {
                    game_id_number = number_value;
                }
                Err(e) => println!("Failed to parse the number: {}", e),
            }
        }

        // Collect largest color count
        line.split(";").into_iter().for_each(|game_string| {
            let blue = blue_score_regex.captures(game_string);
            let red = red_score_regex.captures(game_string);
            let green = green_score_regex.captures(game_string);

            if let Some(matched_group) = blue {
                let number_str = matched_group.get(1).unwrap().as_str();
                match number_str.parse::<i32>() {
                    Ok(number_value) => {
                        if blue_score < number_value {
                            blue_score = number_value;
                        }
                    }
                    Err(e) => println!("Failed to parse the number: {}", e),
                }
            }

            if let Some(matched_group) = red {
                let number_str = matched_group.get(1).unwrap().as_str();
                match number_str.parse::<i32>() {
                    Ok(number_value) => {
                        if red_score < number_value {
                            red_score = number_value;
                        }
                    }
                    Err(e) => println!("Failed to parse the number: {}", e),
                }
            }

            if let Some(matched_group) = green {
                let number_str = matched_group.get(1).unwrap().as_str();
                match number_str.parse::<i32>() {
                    Ok(number_value) => {
                        if green_score < number_value {
                            green_score = number_value;
                        }
                    }
                    Err(e) => println!("Failed to parse the number: {}", e),
                }
            }
        });

        return (game_id_number, blue_score * red_score * green_score);
    });

    let result: i32 = possible_games.map(|(_, power)| power).sum();

    println!("Result: {}", result);
}
