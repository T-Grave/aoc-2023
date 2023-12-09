use std::collections::HashMap;

use num_integer::lcm;
use regex::Regex;

use crate::input_utils::read_input_lines;

pub fn part1() {
    let input = read_input_lines("input/day8_part1.txt").expect("Could not read input file");
    let mut input_iter = input.iter();

    let instructions: Vec<char> = input_iter.next().unwrap().chars().collect();

    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();

    input_iter.next(); // Whiteline

    let node_regex = Regex::new(r"[A-Z]{3}").unwrap();
    input_iter.for_each(|line| {
        let mut matches = node_regex.find_iter(line);

        let node = matches.next().unwrap().as_str();
        let left = matches.next().unwrap().as_str();
        let right = matches.next().unwrap().as_str();

        node_map.insert(node, (left, right));
    });

    let mut current_node = "AAA";
    let mut steps = 0;

    while current_node != "ZZZ" {
        let (left, right) = node_map.get(current_node).unwrap();
        println!(
            "Current node: {:?}, left: {:?}, right: {:?}",
            current_node, left, right
        );

        let instruction = instructions[steps % instructions.len()];

        if instruction == 'R' {
            current_node = right;
        } else {
            current_node = left;
        }

        steps += 1;
    }

    println!("Steps: {:?}", steps);
}

pub fn part2() {
    let input = read_input_lines("input/day8_part1.txt").expect("Could not read input file");
    let mut input_iter = input.iter();

    let instructions: Vec<char> = input_iter.next().unwrap().chars().collect();

    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();

    input_iter.next(); // Whiteline

    let node_regex = Regex::new(r"[0-9A-Z]{3}").unwrap();
    input_iter.for_each(|line| {
        let mut matches = node_regex.find_iter(line);

        let node = matches.next().unwrap().as_str();
        let left = matches.next().unwrap().as_str();
        let right = matches.next().unwrap().as_str();

        node_map.insert(node, (left, right));
    });

    let mut current_nodes: Vec<&str> = node_map
        .keys()
        .filter(|node| node.chars().last().unwrap() == 'A')
        .map(|x| *x)
        .collect();

    println!("initial nodes: {:?}", current_nodes);

    let mut steps_per_node: Vec<i64> = vec![0; current_nodes.len()];
    let mut steps = 0;
    let mut all_end_on_z = false;

    while !all_end_on_z {
        all_end_on_z = true;
        let instruction = instructions[steps % instructions.len()];

        // Multithread with rayon?
        for node in 0..current_nodes.len() {
            if current_nodes[node].ends_with("Z") {
                continue;
            }

            all_end_on_z = false;
            steps_per_node[node] += 1;

            if let Some(&(left, right)) = node_map.get(current_nodes[node]) {
                if instruction == 'R' {
                    current_nodes[node] = right;
                } else {
                    current_nodes[node] = left;
                }
            }
        }

        steps += 1;
    }

    let lcm_result = steps_per_node
        .iter()
        .fold(1, |lcm_so_far, &number| lcm(lcm_so_far, number));

    println!("Steps: {:?}", lcm_result);
}
