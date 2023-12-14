use std::{collections::HashSet, time::Instant};

use crate::input_utils;

fn parse_input(lines: Vec<String>) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    for line in lines {
        let mut chars = Vec::new();
        for c in line.chars() {
            chars.push(c);
        }
        result.push(chars);
    }

    result
}

fn retrieve_boulders(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'O' {
                result.push((x, y));
            }
        }
    }
    result
}

enum Direction {
    North,
    South,
    East,
    West,
}

fn move_boulders(grid: &mut Vec<Vec<char>>, direction: Direction) -> bool {
    let mut moved_at_all = false;
    loop {
        let mut moved = false;
        for (y, line) in grid.clone().iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                match direction {
                    Direction::North => {
                        if y > 0 && *c == 'O' && grid[y - 1][x] == '.' {
                            grid[y - 1][x] = 'O';
                            grid[y][x] = '.';
                            moved = true;
                            moved_at_all = true;
                        }
                    }
                    Direction::South => {
                        if y < grid.len() - 1 && *c == 'O' && grid[y + 1][x] == '.' {
                            grid[y + 1][x] = 'O';
                            grid[y][x] = '.';
                            moved = true;
                            moved_at_all = true;
                        }
                    }
                    Direction::East => {
                        if x < line.len() - 1 && *c == 'O' && grid[y][x + 1] == '.' {
                            grid[y][x + 1] = 'O';
                            grid[y][x] = '.';
                            moved = true;
                            moved_at_all = true;
                        }
                    }
                    Direction::West => {
                        if x > 0 && *c == 'O' && grid[y][x - 1] == '.' {
                            grid[y][x - 1] = 'O';
                            grid[y][x] = '.';
                            moved = true;
                        }
                    }
                }
            }
        }

        if !moved {
            break;
        }
    }

    moved_at_all
}

fn move_boulders_fast(
    grid: &mut Vec<Vec<char>>,
    boulders: &mut Vec<(usize, usize)>,
    direction: Direction,
) {
    loop {
        let mut moved = false;

        boulders.iter_mut().for_each(|(x, y)| match direction {
            Direction::North => {
                if *y > 0 && grid[*y - 1][*x] == '.' {
                    grid[*y - 1][*x] = 'O';
                    grid[*y][*x] = '.';
                    *y -= 1;
                    moved = true;
                }
            }
            Direction::South => {
                if *y < grid.len() - 1 && grid[*y + 1][*x] == '.' {
                    grid[*y + 1][*x] = 'O';
                    grid[*y][*x] = '.';
                    *y += 1;
                    moved = true;
                }
            }
            Direction::East => {
                if *x < grid[*y].len() - 1 && grid[*y][*x + 1] == '.' {
                    grid[*y][*x + 1] = 'O';
                    grid[*y][*x] = '.';
                    *x += 1;
                    moved = true;
                }
            }
            Direction::West => {
                if *x > 0 && grid[*y][*x - 1] == '.' {
                    grid[*y][*x - 1] = 'O';
                    grid[*y][*x] = '.';
                    *x -= 1;
                    moved = true;
                }
            }
        });

        if !moved {
            break;
        }
    }
}

fn count_weight(grid: &Vec<Vec<char>>) -> u64 {
    grid.iter().enumerate().fold(0, |acc, (index, line)| {
        acc + line.iter().fold(0, |acc, c| {
            if *c == 'O' {
                acc + (grid.len() - index) as u64
            } else {
                acc
            }
        })
    })
}

pub fn part1() {
    let lines = input_utils::read_input_lines("input/day14_part1.txt");
    let mut grid = parse_input(lines.unwrap());

    move_boulders(&mut grid, Direction::North);

    for line in &grid {
        println!("{:?}", line);
    }

    let weight = count_weight(&grid);

    println!("result: {}", weight)
}

pub fn part2_old() {
    let lines = input_utils::read_input_lines("input/day14_part1.txt");
    let mut grid = parse_input(lines.unwrap());

    let mut boulders = retrieve_boulders(&grid);

    let start = Instant::now();

    let mut cycle_history = HashSet::new();
    cycle_history.insert(boulders.clone());

    let target_index = 1_000_000_000;
    let mut index = 0;

    while index < target_index {
        move_boulders_fast(&mut grid, &mut boulders, Direction::North);
        move_boulders_fast(&mut grid, &mut boulders, Direction::West);
        move_boulders_fast(&mut grid, &mut boulders, Direction::South);
        move_boulders_fast(&mut grid, &mut boulders, Direction::East);

        if cycle_history.contains(&boulders) && index < 1_000 {
            let cycles_left = 1_000_000_000 % index;
            println!("Cycle found at {}, jump to: {}", index, cycles_left);
            index = 1_000_000_000 - cycles_left;
        }
        index += 1;
        cycle_history.insert(boulders.clone());

        if (index + 1) % 100 == 0 {
            let elapsed = start.elapsed();
            let rate = (index + 1) as f64 / elapsed.as_secs_f64();
            let remaining = (1_000_000_000 - index - 1) as f64 / rate;
            println!(
                "Cycle {} ({}) ({}m remaining)",
                index + 1,
                rate,
                remaining / 60.0
            );
        }
    }

    let weight = count_weight(&grid);

    println!("result: {}", weight)
}

pub fn part2() {
    let lines = input_utils::read_input_lines("input/day14_part1.txt");
    let mut grid = parse_input(lines.unwrap());

    let mut cycle_history = Vec::new();

    let target_index = 1_000_000_000;
    let mut index = 0;
    let mut cycle_start = 0;
    let cycle_end;
    let mut final_grid = grid.clone();

    while index < target_index {
        if cycle_history.contains(&grid) && cycle_start == 0 {
            cycle_start = cycle_history.iter().position(|item| item == &grid).unwrap();
            cycle_end = index;
            println!("Cycle from {} to {}", cycle_start, cycle_end);

            let remaining_cycles = target_index - cycle_end;
            println!("Remaining cycles {}", remaining_cycles);

            final_grid =
                cycle_history[cycle_start + (remaining_cycles % (cycle_end - cycle_start))].clone();

            break;
        } else {
            cycle_history.push(grid.clone());

            move_boulders(&mut grid, Direction::North);
            move_boulders(&mut grid, Direction::West);
            move_boulders(&mut grid, Direction::South);
            move_boulders(&mut grid, Direction::East);

            index += 1;
        }
    }

    let weight = count_weight(&final_grid);
    println!("result: {}", weight);
    for line in &final_grid {
        println!("{:?}", line);
    }
}
