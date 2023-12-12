use crate::input_utils::read_input_lines;

#[derive(Debug)]
struct Universe {
    elements: Vec<char>,
    size_x: usize,
    size_y: usize,
    expanded_cols: Vec<usize>,
    expanded_rows: Vec<usize>,
}

impl Universe {
    fn new(elements: Vec<char>, size_x: usize, size_y: usize) -> Universe {
        Universe {
            elements,
            size_x,
            size_y,
            expanded_cols: Vec::new(),
            expanded_rows: Vec::new(),
        }
    }

    fn get_element(&self, x: usize, y: usize) -> char {
        self.elements[x + y * self.size_x]
    }

    // fn get_index(&self, x: usize, y: usize) -> usize {
    //     x + y * self.size_x
    // }

    // fn get_xy(&self, index: usize) -> (usize, usize) {
    //     (index % self.size_x, index / self.size_x)
    // }

    pub fn mark_expanded_cols(&mut self) {
        for x in 0..self.size_x {
            let mut empty_col = true;
            for y in 0..self.size_y {
                if self.get_element(x, y) != '.' {
                    empty_col = false;
                    break;
                }
            }

            if empty_col {
                self.expanded_cols.push(x);
            }
        }
    }

    pub fn mark_expanded_rows(&mut self) {
        for y in 0..self.size_y {
            let mut empty_row = true;
            for x in 0..self.size_x {
                if self.get_element(x, y) != '.' {
                    empty_row = false;
                    break;
                }
            }

            if empty_row {
                self.expanded_rows.push(y);
            }
        }
    }

    pub fn find_galaxies(&self) -> Vec<(usize, usize)> {
        let mut galaxies: Vec<(usize, usize)> = Vec::new();

        for x in 0..self.size_x {
            for y in 0..self.size_y {
                if self.get_element(x, y) == '#' {
                    galaxies.push((x, y));
                }
            }
        }

        galaxies
    }

    pub fn calculate_manhattan_distance(
        &self,
        galaxy_a: (usize, usize),
        galaxy_b: (usize, usize),
    ) -> usize {
        let (x1, y1) = galaxy_a;
        let (x2, y2) = galaxy_b;

        let x_modifier = self
            .expanded_cols
            .iter()
            .filter(|&x| {
                if x1 < x2 {
                    x1 < *x && *x < x2
                } else {
                    x2 < *x && *x < x1
                }
            })
            .count();

        let x_distance = x1.abs_diff(x2);

        let y_modifiers = self
            .expanded_rows
            .iter()
            .filter(|&y| {
                if y1 < y2 {
                    y1 < *y && *y < y2
                } else {
                    y2 < *y && *y < y1
                }
            })
            .count();
        let y_distance = y1.abs_diff(y2);

        x_distance + y_distance + (x_modifier * 999_999) + (y_modifiers * 999_999)
    }
}

pub fn part1() {
    let lines = read_input_lines("input/day11_part1.txt").expect("Could not read input file");
    let lines_len = lines.len();
    let mut characters: Vec<char> = Vec::new();
    for line in lines {
        for character in line.chars() {
            characters.push(character);
        }
    }

    let total_elements = characters.len();

    let mut universe = Universe::new(characters, total_elements / lines_len, lines_len);
    universe.mark_expanded_cols();
    universe.mark_expanded_rows();
    let galaxies = universe.find_galaxies();

    let mut total = 0;
    let mut pairs = 0;
    for galaxy_a in &galaxies {
        for galaxy_b in &galaxies {
            if galaxy_a == galaxy_b {
                continue;
            }

            pairs += 1;

            let distance = universe.calculate_manhattan_distance(*galaxy_a, *galaxy_b);

            total += distance;
        }
    }

    println!("Universe: {:?} / {}", total / 2, pairs);
}
