use std::{fs, iter::zip};

const INPUT_PATH: &str = "resources/input_1.txt";

fn part_1_process(input: String) -> usize {
    let mut left_list: Vec<usize> = Vec::new();
    let mut right_list: Vec<usize> = Vec::new();
    for line in input.lines() {
        let left: usize = line[..5].parse().unwrap();
        let right: usize = line[8..].parse().unwrap();
        left_list.push(left);
        right_list.push(right);
    }
    left_list.sort();
    right_list.sort();
    let mut result = 0;
    for (left, right) in zip(left_list, right_list) {
        result += left.abs_diff(right);
    }
    result
}

pub fn part_1() -> usize {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    part_1_process(input)
}

fn part_2_process(input: String) -> usize {
    let mut left_list: Vec<usize> = Vec::new();
    let mut right_list: Vec<usize> = Vec::new();
    for line in input.lines() {
        let left: usize = line[..5].parse().unwrap();
        let right: usize = line[8..].parse().unwrap();
        left_list.push(left);
        right_list.push(right);
    }
    let mut result = 0;
    for left in left_list.iter() {
        for right in right_list.iter() {
            if left == right {
                result += left;
            }
        }
    }
    result
}

pub fn part_2() -> usize {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    part_2_process(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = String::from(
            "00003   00004\n00004   00003\n00002   00005\n00001   00003\n00003   00009\n00003   00003\n",
        );
        let result = part_1_process(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn part_2_example() {
        let input = String::from(
            "00003   00004\n00004   00003\n00002   00005\n00001   00003\n00003   00009\n00003   00003\n",
        );
        let result = part_2_process(input);
        assert_eq!(result, 31);
    }
}
