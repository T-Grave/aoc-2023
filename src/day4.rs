use crate::input_utils::read_input_lines;

pub fn part1() {
    let input = read_input_lines("input/day4_part1.txt").expect("Could not read input file");

    let result: i32 = input
        .iter()
        .map(|line| {
            let card = line.split(":").collect::<Vec<&str>>();
            let numbers = card[1].split("|").collect::<Vec<&str>>();
            let winning_numbers = numbers[0].trim().split_whitespace().collect::<Vec<&str>>();
            let my_numbers = numbers[1].trim().split_whitespace().collect::<Vec<&str>>();

            let mut points: i32 = 0;

            for i in 0..my_numbers.len() {
                if winning_numbers.contains(&my_numbers[i]) {
                    if points == 0 {
                        points = 1;
                    } else {
                        points *= 2;
                    }
                }
            }
            println!(
                "{}: {}, {:?}, {:?}",
                card[0], points, winning_numbers, my_numbers
            );

            return points;
        })
        .sum();

    println!("Result: {}", result);
}

pub fn part2() {
    let input = read_input_lines("input/day4_part2.txt").expect("Could not read input file");

    let mut card_counts: Vec<i32> = vec![1; input.len()];

    for (index, line) in input.iter().enumerate() {
        let card = line.split(":").collect::<Vec<&str>>();
        let numbers = card[1].split("|").collect::<Vec<&str>>();
        let winning_numbers = numbers[0].trim().split_whitespace().collect::<Vec<&str>>();
        let my_numbers = numbers[1].trim().split_whitespace().collect::<Vec<&str>>();

        let mut winning_number_count: usize = 0;

        for i in 0..my_numbers.len() {
            if winning_numbers.contains(&my_numbers[i]) {
                winning_number_count += 1;
                card_counts[index + winning_number_count] += card_counts[index];
            }
        }
    }

    let result: i32 = card_counts.iter().sum();

    println!("Result: {}", result);
}
