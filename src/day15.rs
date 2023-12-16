use std::collections::HashMap;

fn hash_sequence(sequence: &str) -> u64 {
    let mut result: u64 = 0;
    sequence.chars().for_each(|c| {
        if c.is_ascii() {
            let ascii = c as u8;
            result += ascii as u64;
            result *= 17;
            result %= 256;
        } else {
            println!("Not ascii: {}", c);
        }
    });

    result
}

pub fn part1() {
    let lines = crate::input_utils::read_input_lines("input/day15_test.txt");
    let first_line = lines.unwrap()[0].clone();
    let steps = first_line.split(',').collect::<Vec<&str>>();

    let result: u64 = steps
        .clone()
        .iter()
        .map(|step| hash_sequence(step))
        .sum();

    println!("Final Result: {:?}", result);
}

pub fn part2() {
    let lines = crate::input_utils::read_input_lines("input/day15_part1.txt");
    let first_line = lines.unwrap()[0].clone();
    let steps = first_line.split(',').collect::<Vec<&str>>();
    let mut map: HashMap<u64, Vec<(&str, u8)>> = HashMap::new();

    steps.iter().for_each(|step| {
        if step.contains('=') {
            let (label, focal_strength) = step.split_once('=').unwrap();
            let box_no = hash_sequence(label);
            let focal_strength: u8 = focal_strength.parse().unwrap();

            match map.get_mut(&box_no) {
                Some(contents) => {
                    match contents.iter_mut().find(|(key, _)| *key == label) {
                        Some(existing) => {
                            existing.1 = focal_strength;
                        }
                        None => {
                            contents.push((label, focal_strength));
                        }
                    }
                }
                None => {
                    let v = vec![(label, focal_strength)];
                    map.insert(box_no, v);
                }
            }
        } else {
            let label: &str = &step[0..(step.len() - 1)];
            let box_no = hash_sequence(label);
            if let Some(contents) = map.get_mut(&box_no) {
                if let Some(index) = contents.iter().position(|(l, _)| *l == label) {
                    contents.remove(index);
                }
            }
        }
    });

    let value: u64 = map.iter().map(|(box_no, lenses)| {
        lenses.iter().enumerate().map(|(index, (_, focal))| {
            (box_no + 1) * (index + 1) as u64 * *focal as u64
        }).sum::<u64>()
    }).sum();

    println!("Result: {:}", value);
}
