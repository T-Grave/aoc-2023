use std::collections::HashSet;

use crate::input_utils::read_input_lines;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn get_next_beam_coordinate(position: &(i32, i32), direction: &Direction) -> (i32, i32) {
    let (x, y) = position;
    match direction {
        Direction::Right => {
            (x + 1, *y)
        }
        Direction::Left => {
            (x - 1, *y)
        }
        Direction::Up => {
            (*x, y - 1)
        }
        Direction::Down => {
            (*x, y + 1)
        }
    }
}

fn trace_beam(grid: &Vec<Vec<char>>, starting_point: (i32, i32), direction: Direction, beam_trace: &mut HashSet<(i32, i32, Direction)>) {
    let mut beam_pos = starting_point;
    let mut beam_dir = direction;
    loop {
        let (x, y) = beam_pos;
        let tile = grid[y as usize][x as usize];

        beam_trace.insert((x, y, beam_dir));

        match tile {
            '|' => {
                match beam_dir {
                    Direction::Left | Direction::Right => {
                        trace_beam(grid, beam_pos, Direction::Up, beam_trace);
                        beam_dir = Direction::Down
                    }
                    _ => {}
                }
            }
            '-' => {
                match beam_dir {
                    Direction::Up | Direction::Down => {
                        trace_beam(grid, beam_pos, Direction::Left, beam_trace);
                        beam_dir = Direction::Right
                    }
                    _ => {}
                }
            }
            '/' => {
                beam_dir = match beam_dir {
                    Direction::Right => Direction::Up,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left
                }
            }
            '\\' => {
                beam_dir = match beam_dir {
                    Direction::Right => Direction::Down,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right
                }
            }

            _ => {
                // default, move to next position
            }
        }

        let (new_x, new_y) = get_next_beam_coordinate(&beam_pos, &beam_dir);

        if new_x < 0 || new_x >= grid[y as usize].len() as i32 || new_y < 0 || new_y >= grid.len() as i32 || beam_trace.contains(&(new_x, new_y, beam_dir)) {
            // Going out of bounds, beam came to an end OR cycle
            break;
        }

        // if we are within bounds, just move to next
        beam_pos = (new_x, new_y);
    }
}

fn count_energized_tiles(grid: &Vec<Vec<char>>, starting_point: (i32, i32), direction: Direction) -> usize {
    let mut beam_trace = HashSet::new();
    trace_beam(grid, starting_point, direction, &mut beam_trace);
    let result: HashSet<(i32, i32)> = beam_trace.iter().map(|(x, y, _)| (*x, *y)).collect();

    result.len()
}

pub fn part2() {
    let lines = read_input_lines("input/day16_part1.txt").unwrap();
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut largest = 0;

    for x in 0..grid[0].len() {
        // 0
        let temp_result = count_energized_tiles(&grid, (x as i32, 0), Direction::Down);
        if temp_result > largest {
            largest = temp_result;
        }

        let temp_result = count_energized_tiles(&grid, (x as i32, (grid.len() - 1) as i32), Direction::Up);
        if temp_result > largest {
            largest = temp_result;
        }
    }

    for y in 0..grid.len() {
        // 0
        let temp_result = count_energized_tiles(&grid, (0, y as i32), Direction::Right);
        if temp_result > largest {
            largest = temp_result;
        }

        // len
        let temp_result = count_energized_tiles(&grid, ((grid[y].len() - 1) as i32, y as i32), Direction::Left);
        if temp_result > largest {
            largest = temp_result;
        }
    }

    println!("Result {}", largest);
}

pub fn part1() {
    let lines = read_input_lines("input/day16_test.txt").unwrap();
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();


    let result = count_energized_tiles(&grid, (0, 0), Direction::Right);


    // let mut printable_grid = grid.clone();
    //
    // for y in 0..printable_grid.len() {
    //     for x in 0..printable_grid[y].len() {
    //         if result.contains(&(x as i32, y as i32)) {
    //             printable_grid[y][x] = '#';
    //         }
    //     }
    //     println!("{:?}", printable_grid[y]);
    // }


    println!("Result {}", result);
}