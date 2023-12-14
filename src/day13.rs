use crate::input_utils::read_input_lines;

fn reverse_orientation_pattern(pattern: &Vec<String>) -> Vec<String> {
    let mut reversed_pattern: Vec<String> = Vec::new();

    for index in 0..pattern[0].len() {
        let mut reversed_line = String::new();

        for line in pattern {
            reversed_line.push(line.chars().nth(index).unwrap());
        }

        reversed_pattern.push(reversed_line);
    }

    reversed_pattern
}

fn parse_patterns() -> Vec<Vec<String>> {
    let lines = read_input_lines("input/day13_part1.txt").expect("Could not read input file");

    let mut patterns: Vec<Vec<String>> = Vec::new();
    patterns.push(Vec::new());

    for line in lines {
        if line == "" {
            patterns.push(Vec::new());
        } else {
            patterns.last_mut().unwrap().push(line);
        }
    }

    patterns
}

fn find_mirror_indexes(pattern: &Vec<String>) -> Vec<usize> {
    let mut potential_mirrors = Vec::new();

    for (index, line) in pattern.windows(2).enumerate() {
        if *line[0] == *line[1] {
            potential_mirrors.push(index);
        }
    }

    potential_mirrors
}

fn validate_reflection(pattern: &Vec<String>, mirror_index: usize) -> bool {
    // println!(
    //     "Validate reflection start: {:?} - {:?}",
    //     pattern, mirror_index
    // );

    let (mut left, mut right) = pattern.split_at(mirror_index + 1);
    let diff: i32 = left.len() as i32 - right.len() as i32;
    // println!(
    //     "Validate reflection diff: {:?} - {:?} = {}",
    //     left.len(),v
    //     right.len(),
    //     diff
    // );
    if diff > 0 {
        left = &left[(diff as usize)..];
    } else if diff < 0 {
        let index = right.len() - diff.abs() as usize;
        right = &right[..index];
    }

    // println!("Validate reflection: {:?} - {:?}", left, right);

    for (left_line, right_line) in left.iter().zip(right.iter().rev()) {
        if left_line != right_line {
            return false;
        }
    }

    true
}

pub fn get_mirror_score(pattern: &Vec<String>) -> i32 {
    // Check for horizontal reflection
    // println!("Horizontal pattern: {:?}", pattern.len());
    //
    let potential_mirrors = find_mirror_indexes(pattern);
    // println!("Potential horizontal mirrors: {:?}", potential_mirrors);
    for mirror_index in potential_mirrors {
        if validate_reflection(pattern, mirror_index) {
            // println!(
            //     "Found valid horizontal reflection: {} - {:?}",
            //     mirror_index, pattern
            // );
            return (mirror_index as i32 + 1) * 100;
        }
    }

    // Check for vertical reflection
    let vertical_pattern = reverse_orientation_pattern(pattern);
    // println!("Vertical pattern: {:?}", vertical_pattern.len());

    let potential_mirrors = find_mirror_indexes(&vertical_pattern);
    // println!("Potential vertical mirrors: {:?}", potential_mirrors);

    for mirror_index in potential_mirrors {
        if validate_reflection(&vertical_pattern, mirror_index) {
            // println!(
            //     "Found valid vertical reflection: {} - {:?}",
            //     mirror_index, vertical_pattern
            // );
            return mirror_index as i32 + 1;
        }
    }

    return 0;
}

pub fn part1() {
    let patterns = parse_patterns();

    let total = patterns.iter().fold(0, |acc, pattern| {
        println!("Pattern: {:?}", pattern);
        acc + get_mirror_score(pattern)
    });

    println!("Total: {}", total);
}
