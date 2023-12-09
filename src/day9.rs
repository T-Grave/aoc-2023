use crate::input_utils::read_input_lines;

fn get_numbers_from_line(line: &String) -> Vec<i64> {
    line.split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect()
}

pub fn map_to_differences_stack_until_zero(numbers: Vec<i64>) -> Vec<Vec<i64>> {
    let mut stack: Vec<Vec<i64>> = Vec::new();
    stack.push(numbers);

    loop {
        let differences = stack
            .last()
            .unwrap()
            .windows(2)
            .map(|slice| slice[1] - slice[0])
            .collect();

        println!("Differences: {:?}", differences);
        stack.push(differences);

        if stack.last().unwrap().iter().all(|a| *a == 0) {
            break;
        }
    }

    // if stack.last().unwrap().is_empty() {
    //     println!("Empty line in stack: {:?}", stack);
    // }

    stack
}

pub fn predict_next_number(stack: Vec<Vec<i64>>) -> i64 {
    stack
        .iter()
        .rev()
        // .skip(0)
        .fold(0, |acc, next| acc + next.last().unwrap_or(&(0 as i64)))
}

pub fn part1() {
    let input = read_input_lines("input/day9_part1.txt").expect("Could not read input file");

    let result: i64 = input
        .iter()
        .map(get_numbers_from_line)
        .map(map_to_differences_stack_until_zero)
        .map(predict_next_number)
        .sum();

    println!("Result: {}", result);
}

pub fn predict_prev_number(stack: Vec<Vec<i64>>) -> i64 {
    stack
        .iter()
        .rev()
        .fold(0, |acc, next| next.first().unwrap_or(&(0 as i64)) - acc)
}

pub fn part2() {
    let input = read_input_lines("input/day9_part2.txt").expect("Could not read input file");

    let result: i64 = input
        .iter()
        .map(get_numbers_from_line)
        .map(map_to_differences_stack_until_zero)
        .map(predict_prev_number)
        .sum();

    println!("Result: {}", result);
}
