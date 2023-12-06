use std::sync::{Arc, Mutex};

use rayon::prelude::*;
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

// each line of numbers is [destination range start, source range start, range length]

pub fn part1() {
    let input = read_input_lines("input/day5_part1.txt").expect("Could not read input file");

    // let mut current_map_name: &str = "seeds";

    let mut seeds: Vec<i64> = Vec::new();

    // let mut maps: HashMap<&str, Vec<Vec<i32>>> = HashMap::new();
    let mut maps_vec: Vec<Vec<Vec<i64>>> = Vec::new();

    // Build maps
    for (index, line) in input.iter().enumerate() {
        if line == "" {
            continue;
        }

        if line.contains("seeds:") {
            seeds = get_numbers_from_line(line);
            println!("seeds: {:?}", seeds);
        } else if line.contains("map:") {
            // Set current map
            // current_map_name = line.split_whitespace().collect::<Vec<&str>>()[0];
            maps_vec.push(Vec::new());
            // maps.insert(current_map_name, Vec::new());
            continue;
        } else {
            let numbers = get_numbers_from_line(line);
            let map = maps_vec.last_mut().unwrap();
            map.push(numbers.clone());

            // println!("numbers: {:?}", numbers)
        }
    }

    // Perform recursive mapping
    let mut previous_result = seeds.clone();
    for (map_index, mapping_values) in maps_vec.iter().enumerate() {
        // println!("{}: {:?}", map_index, mapping_values);
        let map_helpers = mapping_values.iter().map(|map_config| {
            let diff = map_config[0] - map_config[1];
            let start = map_config[1];
            let end = map_config[1] + map_config[2];

            return (start, end, diff);
        });

        // println!("{:?}", map_helpers);

        previous_result = previous_result
            .iter()
            .map(|source| {
                let mut result = *source;

                for (start, end, diff) in map_helpers.clone() {
                    if *source >= start && *source < end {
                        result = source + diff;
                    }
                }

                result
            })
            .collect();

        // println!("{}: {:?}", map_index, previous_result)
        // destination: 50, source: 98, range: 2
        // destination: 52, source: 50, range: 48
    }

    let lowest = previous_result.iter().min().unwrap();

    println!("Result: {:?}", lowest);
}

pub fn part2() {
    let input = read_input_lines("input/day5_part2.txt").expect("Could not read input file");

    let mut seed_pairs: Vec<(i64, i64)> = Vec::new();
    let mut maps_vec: Vec<Vec<Vec<i64>>> = Vec::new();

    // Build maps
    for (index, line) in input.iter().enumerate() {
        if line == "" {
            continue;
        }

        if line.contains("seeds:") {
            let seed_input = get_numbers_from_line(line);
            for i in (0..seed_input.len()).step_by(2) {
                seed_pairs.push((seed_input[i], seed_input[i + 1]));
            }
            println!("seed pairs: {:?}", seed_pairs);
        } else if line.contains("map:") {
            maps_vec.push(Vec::new());
            continue;
        } else {
            let numbers = get_numbers_from_line(line);
            let map = maps_vec.last_mut().unwrap();
            map.push(numbers.clone());
        }
    }

    // Perform recursive mapping
    let mut result = Arc::new(Mutex::new(i64::MAX));

    seed_pairs.par_iter().for_each(|&(start, length)| {
        println!("Start crunching seedpair: {} | {}", start, length);
        let mut previous_result: Vec<i64> = (0..length).map(|i| i + start).collect();

        for (map_index, mapping_values) in maps_vec.iter().enumerate() {
            // println!("{}: {:?}", map_index, mapping_values);
            println!(
                "Construct map_helpers: {}, {}",
                map_index,
                mapping_values.len()
            );
            let map_helpers: Vec<(i64, i64, i64)> = mapping_values
                .iter()
                .map(|map_config| {
                    let diff = map_config[0] - map_config[1];
                    let start = map_config[1];
                    let end = map_config[1] + map_config[2];

                    return (start, end, diff);
                })
                .collect();

            // println!("{:?}", map_helpers);

            previous_result = previous_result
                .iter()
                .map(|source| {
                    let mut result = *source;

                    for (start, end, diff) in &map_helpers {
                        if *source >= *start && *source < *end {
                            result = source + diff;
                        }
                    }

                    result
                })
                .collect();

            // println!("{}: {:?}", map_index, previous_result)
            // destination: 50, source: 98, range: 2
            // destination: 52, source: 50, range: 48
        }

        let lowest = previous_result.iter().min().unwrap();

        let mut result_guard = result.lock().unwrap();
        if lowest < &*result_guard {
            *result_guard = *lowest;
        }

        println!("Finished crunching seedpair: {} | {}", start, length);
    });

    let final_result = *result.lock().unwrap();
    println!("Result: {:?}", final_result);
}
