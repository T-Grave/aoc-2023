use crate::input_utils::read_input_lines;

pub fn part1() {
    let input = read_input_lines("input/day1_part1.txt").expect("Could not read input file");

    let result = input
        .iter()
        .map(|line| {
            println!("{}", line);
            let first_digit_index: usize = line.find(char::is_numeric).unwrap();
            let last_digit_index: usize = line.rfind(char::is_numeric).unwrap();

            format!(
                "{}{}",
                line.chars().nth(first_digit_index).unwrap(),
                line.chars().nth(last_digit_index).unwrap()
            )
        })
        .map(|digits| digits.parse().unwrap())
        .reduce(|acc: i32, next| acc + next);

    println!("Result: {}", result.unwrap());
}

pub fn part2() {
    let input = read_input_lines("input/day1_part2.txt").expect("Could not read input file");

    let replacements = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("zero", "0"),
    ];

    let result = input
        .iter()
        .map(|line| {
            let mut first_index = line.find(char::is_numeric).unwrap();
            let mut first_to_replace = ("", "");
            let mut last_index = line.rfind(char::is_numeric).unwrap();
            let mut last_to_replace = ("", "");
            let mut new_line = line.clone();

            replacements.iter().for_each(|(target, replacement)| {
                let result = line.find(target);
                if result.is_some() && result.unwrap() < first_index {
                    first_index = result.unwrap();
                    first_to_replace = (target, replacement);
                }

                let result_r = line.rfind(target);
                if result_r.is_some() && result_r.unwrap() > last_index {
                    last_index = result_r.unwrap();
                    last_to_replace = (target, replacement);
                }
            });

            new_line = new_line.replace(first_to_replace.0, first_to_replace.1);
            new_line = new_line.replace(last_to_replace.0, last_to_replace.1);

            println!("old: {}", line);
            println!("new: {}", new_line);

            new_line
        })
        .map(|line| {
            // println!("{}", line);
            let first_digit_index: usize = line.find(char::is_numeric).unwrap();
            let last_digit_index: usize = line.rfind(char::is_numeric).unwrap();

            println!(
                "number: {}{}",
                line.chars().nth(first_digit_index).unwrap(),
                line.chars().nth(last_digit_index).unwrap()
            );

            format!(
                "{}{}",
                line.chars().nth(first_digit_index).unwrap(),
                line.chars().nth(last_digit_index).unwrap()
            )
        })
        .map(|digits| digits.parse().unwrap())
        .reduce(|acc: i32, next| acc + next);

    println!("Result: {}", result.unwrap());
}
