use std::collections::HashMap;

use crate::input_utils::read_input_lines;

// TODO: Memoize this function
pub fn count(
    springs: Vec<char>,
    groups: &Vec<usize>,
    cache: &mut HashMap<(Vec<char>, Vec<usize>), u64>,
) -> u64 {
    if cache.contains_key(&(springs.clone(), groups.clone())) {
        return *cache.get(&(springs.clone(), groups.clone())).unwrap();
    }
    //
    // println!("count({:?}, {:?})", springs, groups);

    // If no springs are left & no groups are left, we have a valid solution
    if springs.is_empty() {
        return if groups.is_empty() {
            1
        } else {
            // Not all groups were used
            0
        }
    }

    if groups.is_empty() {
        // Broken springs left, but no groups left
        if springs.contains(&'#') {
            return 0;
        }
        // Only dots left
        return 1;
    }

    let mut result = 0;

    if springs[0] == '.' || springs[0] == '?' {
        let vec: Vec<char> = springs.iter().skip(1).cloned().collect();
        // println!(".? JUMP INTO: {:?} {:?}", springs, groups);
        let count = count(vec.clone(), groups, cache);
        // println!(".? JUMP OUT OF {:?} {:?}", springs, groups);
        cache.insert((vec, groups.clone()), count);
        result += count;
    }

    if (springs[0] == '#' || springs[0] == '?') && springs.len() >= groups[0]
            && !springs[0..groups[0]].contains(&'.') && (groups[0] == springs.len() || springs[groups[0]] != '#') {
        let new_spring_index = if groups[0] == springs.len() {
            springs.len()
        } else {
            groups[0] + 1
        };

        // result += count(
        //     springs[new_spring_index..].to_vec(),
        //     &groups[1..].to_vec(),
        //     cache,
        // );

        let vec: Vec<char> = springs.iter().skip(new_spring_index).cloned().collect();
        let new_groups = groups[1..].to_vec();
        let count = count(vec.clone(), &new_groups.clone(), cache);
        cache.insert((vec, new_groups), count);
        result += count;
    }

    result
}

pub fn part1() {
    let lines = read_input_lines("input/day12_part1.txt").expect("Could not read input file");

    let records = lines
        .iter()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').unwrap();

            let parsed_groups: Vec<usize> = groups
                .split(',')
                .map(|number| number.parse::<usize>().unwrap())
                .collect();

            (springs, parsed_groups)
        })
        .collect::<Vec<_>>();
    let mut cache: HashMap<(Vec<char>, Vec<usize>), u64> = HashMap::new();
    let result: u64 = records
        .iter()
        .map(|(springs, groups)| count(springs.chars().collect(), groups, &mut cache))
        .sum();

    println!("Result: {}", result);
}

pub fn part2() {
    let lines = read_input_lines("input/day12_part1.txt").expect("Could not read input file");

    let records = lines
        .iter()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').unwrap();

            let repeated_groups = std::iter::repeat(groups)
                .take(5)
                .collect::<Vec<_>>()
                .join(",");
            let parsed_groups: Vec<usize> = repeated_groups
                .split(',')
                .map(|number| number.parse::<usize>().unwrap())
                .collect();

            let repeated_strings = std::iter::repeat(springs).take(5);
            let new_springs = repeated_strings.collect::<Vec<_>>().join("?");
            (new_springs, parsed_groups)
        })
        .collect::<Vec<_>>();

    let mut items_left = records.len();
    let mut cache: HashMap<(Vec<char>, Vec<usize>), u64> = HashMap::new();
    let result: u64 = records
        .iter()
        .map(|(springs, groups)| {
            let arrangement_count = count(springs.chars().collect(), groups, &mut cache);
            println!(
                "({} -> {:?}) Springs: {} & Groups: {:?}",
                items_left, arrangement_count, springs, groups
            );
            items_left -= 1;
            arrangement_count
        })
        .sum();

    println!("Result: {}", result);
}
