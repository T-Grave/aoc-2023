use regex::Regex;

use crate::input_utils::read_input_lines;

fn get_numbers_from_line(line: &str) -> Vec<i64> {
    let mut numbers: Vec<i64> = Vec::new();
    let number_regex = Regex::new(r"\d+").unwrap();
    for cap in number_regex.captures_iter(line) {
        numbers.push(cap[0].parse().unwrap());
    }
    numbers
}

#[derive(Debug)]
struct Race {
    time: i64,
    distance_record: i64,
}

fn get_win_conditions(race: &Race) -> Vec<i64> {
    let mut win_conditions: Vec<i64> = Vec::new();

    for button_held in 1..race.time {
        let time_left = race.time - button_held;
        let distance = button_held * time_left;
        if distance > race.distance_record {
            win_conditions.push(button_held);
        }
    }

    win_conditions
}

pub fn part1() {
    let input = read_input_lines("input/day6_part1.txt").expect("Could not read input file");
    let mut input_iter = input.iter();

    let time_input = input_iter.next().unwrap();
    let distance_input = input_iter.next().unwrap();

    let times = get_numbers_from_line(time_input);
    let distances = get_numbers_from_line(distance_input);

    let mut races = Vec::new();
    for index in 0..times.len() {
        races.push(Race {
            time: times[index],
            distance_record: distances[index],
        });
    }

    println!("Races: {:?}", races);

    let result: i64 = races
        .iter()
        .map(get_win_conditions)
        .map(|win_conditions| win_conditions.len() as i64)
        .product();

    println!("Result: {}", result);
}

pub fn part2() {
    let input = read_input_lines("input/day6_part2.txt").expect("Could not read input file");
    let mut input_iter = input.iter();
    let time_input = input_iter.next().unwrap();
    let distance_input = input_iter.next().unwrap();

    let time: String = get_numbers_from_line(time_input)
        .iter()
        .map(|time| time.to_string())
        .collect();

    let distance: String = get_numbers_from_line(distance_input)
        .iter()
        .map(|time| time.to_string())
        .collect();

    println!("times: {}", time);

    let race = Race {
        time: time.parse().unwrap(),
        distance_record: distance.parse().unwrap(),
    };

    println!("Race: {:?}", race);

    let result: usize = get_win_conditions(&race).len();

    println!("Result: {}", result);
}
