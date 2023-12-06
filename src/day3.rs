use crate::input_utils::read_input_lines;

fn is_symbol(c: char) -> bool {
    return c != '.' && !c.is_numeric();
}

fn check_line(line: Vec<char>, position: usize) -> bool {
    if is_symbol(line[position]) {
        return true;
    }

    if position > 0 {
        if is_symbol(line[position - 1]) {
            return true;
        }
    }

    if position < line.len() - 1 {
        if is_symbol(line[position + 1]) {
            return true;
        }
    }

    return false;
}

pub fn part1() {
    let input = read_input_lines("input/day3_part1.txt").expect("Could not read input file");
    let mut valid_numbers: Vec<i32> = Vec::new();

    let number_regex = regex::Regex::new(r"\d+").unwrap();

    for (line_index, line) in input.iter().enumerate() {
        for number_match in number_regex.find_iter(&line) {
            let number_start = number_match.start();
            let number_end = number_match.end();

            let mut adjacent_symbol = false;

            // For each of the positions, check if there is a adjacent symbol (except for dots)
            for char_index in number_start..number_end {
                // Check the line above
                if line_index > 0 {
                    let line_above: Vec<char> = input[line_index - 1].chars().collect();
                    if check_line(line_above, char_index) {
                        adjacent_symbol = true;
                        break;
                    }
                }

                if line_index < input.len() - 1 {
                    let line_below: Vec<char> = input[line_index + 1].chars().collect();
                    if check_line(line_below, char_index) {
                        adjacent_symbol = true;
                        break;
                    }
                }

                let current_line: Vec<char> = line.chars().collect();
                if check_line(current_line, char_index) {
                    adjacent_symbol = true;
                    break;
                }
            }

            if adjacent_symbol {
                valid_numbers.push(number_match.as_str().parse::<i32>().unwrap());
                println!("Valid number: {}", number_match.as_str());
            }
        }
    }

    let result = valid_numbers.iter().sum::<i32>();

    println!("Result: {}", result);
}

// position 5
fn check_gear_line(line: &str, position: usize) -> Vec<i32> {
    let number_regex = regex::Regex::new(r"\d+").unwrap();
    let mut adjacent_numbers: Vec<i32> = Vec::new();

    for number_match in number_regex.find_iter(&line) {
        let number_start = number_match.start(); // 1
        let number_end = number_match.end() - 1; // 3

        if position > 0 {
            // if 1 <= 4 && 4 <= 3
            if number_start <= position - 1 && position - 1 <= number_end {
                adjacent_numbers.push(number_match.as_str().parse::<i32>().unwrap());
                continue;
            }
        }

        if position < line.len() - 1 {
            // if 1 <= 6 && 6 <= 3
            if number_start <= position + 1 && position + 1 <= number_end {
                adjacent_numbers.push(number_match.as_str().parse::<i32>().unwrap());
                continue;
            }
        }

        // if 1 <= 5 && 5 <= 3
        if number_start <= position && position <= number_end {
            adjacent_numbers.push(number_match.as_str().parse::<i32>().unwrap());
        }
    }

    return adjacent_numbers;
}

pub fn part2() {
    let input = read_input_lines("input/day3_part2.txt").expect("Could not read input file");
    let mut valid_numbers: Vec<i32> = Vec::new();

    let gear_regex = regex::Regex::new(r"\*").unwrap();

    for (line_index, line) in input.iter().enumerate() {
        for gear_match in gear_regex.find_iter(&line) {
            let gear_index = gear_match.start();
            let mut adjacent_numbers: Vec<i32> = Vec::new();

            if line_index > 0 {
                let mut found = check_gear_line(&input[line_index - 1], gear_index);
                adjacent_numbers.append(&mut found);
            }

            if line_index < input.len() - 1 {
                let mut found = check_gear_line(&input[line_index + 1], gear_index);
                adjacent_numbers.append(&mut found);
            }

            let mut found = check_gear_line(line, gear_index);
            adjacent_numbers.append(&mut found);

            if adjacent_numbers.len() == 2 {
                valid_numbers.push(adjacent_numbers[0] * adjacent_numbers[1]);
                println!("{:?}", adjacent_numbers)
            }
        }
    }

    let result = valid_numbers.iter().sum::<i32>();

    println!("Result: {}", result);
}
