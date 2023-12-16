use crate::input_utils::read_input_lines;

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

fn find_starting_position(grid: Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut starting_position = None;
    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, cell)| {
            if *cell == 'S' {
                starting_position = Some((x, y));
            }
        });
    });

    starting_position
}

const CONNECT_WEST: [char; 3] = ['L', 'F', '-'];
const CONNECT_NORTH: [char; 3] = ['7', 'F', '|'];
const CONNECT_SOUTH: [char; 3] = ['J', 'L', '|'];
const CONNECT_EAST: [char; 3] = ['J', '7', '-'];

fn find_adjecent_pipes(grid: Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize, char)> {
    let mut adjecent_pipes = Vec::new();

    if x > 0 && CONNECT_WEST.contains(&grid[y][x - 1]) {
        adjecent_pipes.push((x - 1, y, grid[y][x - 1]));
    }

    if x < grid[y].len() - 1 && CONNECT_EAST.contains(&grid[y][x + 1]) {
        adjecent_pipes.push((x + 1, y, grid[y][x + 1]));
    }

    if y > 0 && CONNECT_NORTH.contains(&grid[y - 1][x]) {
        adjecent_pipes.push((x, y - 1, grid[y - 1][x]));
    }

    if y < grid.len() - 1 && CONNECT_SOUTH.contains(&grid[y + 1][x]) {
        adjecent_pipes.push((x, y + 1, grid[y + 1][x]));
    }

    adjecent_pipes
}

fn get_next_pipe(
    grid: Vec<Vec<char>>,
    network: &Vec<(usize, usize, char)>,
) -> Option<(usize, usize, char)> {
    let (x, y, current_pipe) = network.last().unwrap();
    let (previous_x, previous_y, _) = network.get(network.len() - 2).unwrap();
    let mut next_pipe: Option<(usize, usize, char)> = None;

    match current_pipe {
        '|' => {
            if y > previous_y && y + 1 < grid.len() {
                next_pipe = Some((*x, y + 1, grid[y + 1][*x]));
            } else if y - 1 >= 0 {
                next_pipe = Some((*x, y - 1, grid[y - 1][*x]));
            }
        }
        '-' => {
            if x > previous_x && y + 1 < grid.len() {
                next_pipe = Some((x + 1, *y, grid[*y][x + 1]));
            } else if x - 1 >= 0 {
                next_pipe = Some((x - 1, *y, grid[*y][x - 1]));
            }
        }
        'L' => {
            if y > previous_y && x + 1 < grid[*y].len() {
                next_pipe = Some((x + 1, *y, grid[*y][x + 1]));
            } else if y - 1 >= 0 {
                next_pipe = Some((*x, y - 1, grid[y - 1][*x]));
            }
        }
        'J' => {
            if y > previous_y && x - 1 >= 0 {
                next_pipe = Some((x - 1, *y, grid[*y][x - 1]));
            } else if y - 1 >= 0 {
                next_pipe = Some((*x, y - 1, grid[y - 1][*x]));
            }
        }
        '7' => {
            if y < previous_y && x - 1 >= 0 {
                next_pipe = Some((x - 1, *y, grid[*y][x - 1]));
            } else if y + 1 < grid.len() {
                next_pipe = Some((*x, y + 1, grid[y + 1][*x]));
            }
        }
        'F' => {
            if y < previous_y && x + 1 < grid[*y].len() {
                next_pipe = Some((x + 1, *y, grid[*y][x + 1]));
            } else if y + 1 < grid.len() {
                next_pipe = Some((*x, y + 1, grid[y + 1][*x]));
            }
        }
        _ => {}
    }

    next_pipe
}

// Part 2
fn trace_network_inner_outer(grid: Vec<Vec<char>>, network: &Vec<(usize, usize, char)>) {
    let mut a: Vec<(usize, usize, char)> = Vec::new();
    let mut b: Vec<(usize, usize, char)> = Vec::new();

    // Use windows() to get a sliding window of 2 elements
    network.windows(2).for_each(|window| {
        let (x, y, pipe) = window[0];
        let (next_x, next_y, next_pipe) = window[1];

        // FIXME: We accounted for "overlapping outer", but not for corners, which are the difference makers
        // Alternative idea; use the pipe type to create outlines of the grid, and then use that to determine
        if x < next_x {
            // Moving east
            if y > 0 {
                a.push((x, y - 1, grid[y - 1][x]));
            } else {
                a.push((x, y, grid[y][x]))
            }

            if y + 1 < grid.len() {
                b.push((x, y + 1, grid[y + 1][x]));
            } else {
                b.push((x, y, grid[y][x]))
            }
        } else if x > next_x {
            // Moving west
            if y > 0 {
                b.push((x, y - 1, grid[y - 1][x]));
            } else {
                b.push((x, y, grid[y][x]))
            }

            if y + 1 < grid.len() {
                a.push((x, y + 1, grid[y + 1][x]));
            } else {
                a.push((x, y, grid[y][x]))
            }
        } else if y < next_y {
            // Moving south
            if x + 1 < grid[y].len() {
                a.push((x + 1, y, grid[y][x + 1]));
            } else {
                a.push((x, y, grid[y][x]))
            }

            if x > 0 {
                b.push((x - 1, y, grid[y][x - 1]));
            } else {
                b.push((x, y, grid[y][x]))
            }
        } else if y > next_y {
            // Moving north
            if x + 1 < grid[y].len() {
                b.push((x + 1, y, grid[y][x + 1]));
            } else {
                b.push((x, y, grid[y][x]))
            }

            if x > 0 {
                a.push((x - 1, y, grid[y][x - 1]));
            } else {
                a.push((x, y, grid[y][x]))
            }
        }
    });

    // Compare lenght of a and b to determine if we are inner or outer
    if a.len() > b.len() {
        // Inner
        println!("A = Outer ({}), B = Inner ({})", a.len(), b.len());
    } else {
        println!("B = Outer");
    }
}

pub fn part1() {
    let lines = read_input_lines("input/day10_part1.txt").expect("Could not read input file");

    let grid = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("Grid size: {}x{}", grid.len(), grid[0].len());

    let starting_position =
        find_starting_position(grid.clone()).expect("Could not find starting position");

    let adjecent_pipes =
        find_adjecent_pipes(grid.clone(), starting_position.0, starting_position.1);

    let networks: Vec<Vec<(usize, usize, char)>> = adjecent_pipes
        .iter()
        .map(|(x, y, adjacent_pipe)| {
            println!("Starting pipe: {:?}", grid[*y][*x]);
            let mut network = Vec::new();

            network.push((
                starting_position.0,
                starting_position.1,
                grid[starting_position.1][starting_position.0],
            ));
            network.push((*x, *y, *adjacent_pipe));

            loop {
                let next_pipe = get_next_pipe(grid.clone(), &network);

                if network.len() % 1000 == 0 {
                    println!("Network size: {}", network.len());
                }

                if let Some(pipe) = next_pipe {
                    if pipe.2 == '.' {
                        println!("We encountered a ., this is not a closed loop");
                        break;
                    }

                    if pipe.2 == 'S' {
                        println!("We closed the loop");
                        break;
                    }

                    if network.contains(&pipe) {
                        println!(
                            "We encountered a pipe we already visited, cycle detected, aborting ({:?}) inside {:?}", pipe, network
                        );
                        break;
                    }

                    network.push(pipe);
                } else {
                    break;
                }
            }

            network
        })
        .collect();

    let result: usize = networks
        .iter()
        .map(|network| {
            println!("Network: {:?}", network.len());

            trace_network_inner_outer(grid.clone(), network);

            let half = network.len() / 2;
            println!("half {}", half);

            half
        })
        .max()
        .unwrap();

    println!("Result: {}", result);
}

// pub fn part2() {
//     let input = read_input_lines("input/day10_part2.txt").expect("Could not read input file");

//     println!("Result: {}", result);
// }
