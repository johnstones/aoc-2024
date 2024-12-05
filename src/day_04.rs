use std::{fs, ops::Add};

const INPUT_PATH: &str = "resources/input_04.txt";

#[derive(Debug, PartialEq, Eq)]
struct Vector2D {
    x: isize,
    y: isize,
}

impl Add for &Vector2D {
    type Output = Vector2D;

    fn add(self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Vector2D> for &Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn get_cha(grid: &Vec<&str>, coords: &Vector2D) -> char {
    println!("{:?}", coords);
    grid.get(coords.y as usize)
        .unwrap()
        .chars()
        .nth(coords.x as usize)
        .unwrap()
}

fn is_word(grid: &Vec<&str>, coords: &Vector2D, word: &str, direction: &Vector2D) -> bool {
    if word.is_empty() {
        return true;
    }
    let is_x_in_bounds = coords.x >= 0 && coords.x < grid.first().unwrap().len() as isize;
    let is_y_in_bounds = coords.y >= 0 && coords.y < grid.len() as isize;
    if !is_x_in_bounds || !is_y_in_bounds {
        return false;
    }
    let cha = get_cha(grid, coords);
    if cha == word.chars().next().unwrap() {
        let coords_next = coords + direction;
        return is_word(grid, &coords_next, &word[1..], direction);
    }
    false
}

fn word_count(grid: &Vec<&str>, coords: &Vector2D, word: &str) -> usize {
    [
        Vector2D { x: 0, y: -1 },  // up
        Vector2D { x: 1, y: -1 },  // up, right
        Vector2D { x: 1, y: 0 },   // right
        Vector2D { x: 1, y: 1 },   // down, right
        Vector2D { x: 0, y: 1 },   // down
        Vector2D { x: -1, y: 1 },  // down, left
        Vector2D { x: -1, y: 0 },  // left
        Vector2D { x: -1, y: -1 }, // up, left
    ]
    .iter()
    .filter(|dir| is_word(grid, coords, word, dir))
    .count()
}

fn part_1_process(input: &str) -> usize {
    let grid: Vec<&str> = input.lines().collect();
    let mut result = 0;
    for (row, line) in grid.iter().enumerate() {
        for (col, cha) in line.chars().enumerate() {
            if cha == 'X' {
                let start_coord = Vector2D {
                    x: col as isize,
                    y: row as isize,
                };
                result += word_count(&grid, &start_coord, "XMAS");
            }
        }
    }
    result
}

fn part_2_process(input: &str) -> usize {
    let grid: Vec<&str> = input.lines().collect();
    let mut result = 0;
    for (row, line) in grid[..grid.len() - 1].iter().enumerate().skip(1) {
        for (col, cha) in line[..line.len() - 1].chars().enumerate().skip(1) {
            if cha == 'A' {
                let coords = Vector2D {
                    x: col as isize,
                    y: row as isize,
                };
                let top_right = &coords + Vector2D { x: 1, y: -1 };
                let bottom_right = &coords + Vector2D { x: 1, y: 1 };
                let bottom_left = &coords + Vector2D { x: -1, y: 1 };
                let top_left = &coords + Vector2D { x: -1, y: -1 };
                let top_right_cha = get_cha(&grid, &top_right);
                let bottom_right_cha = get_cha(&grid, &bottom_right);
                let bottom_left_cha = get_cha(&grid, &bottom_left);
                let top_left_cha = get_cha(&grid, &top_left);
                let cha_vec = [
                    top_right_cha,
                    bottom_right_cha,
                    bottom_left_cha,
                    top_left_cha,
                ];
                println!("{:?}", cha_vec);
                if cha_vec == ['M', 'M', 'S', 'S']
                    || cha_vec == ['S', 'M', 'M', 'S']
                    || cha_vec == ['S', 'S', 'M', 'M']
                    || cha_vec == ['M', 'S', 'S', 'M']
                {
                    result += 1;
                }
            }
        }
    }
    result
}

pub fn part_1() -> usize {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    part_1_process(&input)
}

pub fn part_2() -> usize {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    part_2_process(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn part_1_example() {
        let result = part_1_process(EXAMPLE_INPUT);
        assert_eq!(result, 18);
    }

    #[test]
    fn part_2_example() {
        let result = part_2_process(EXAMPLE_INPUT);
        assert_eq!(result, 9);
    }

    #[test]
    fn part_1_answer() {
        let result = part_1();
        assert_eq!(result, 2613);
    }

    #[test]
    fn part_2_answer() {
        let result = part_2();
        assert_eq!(result, 1905);
    }
}
