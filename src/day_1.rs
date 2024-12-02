use std::{fs, iter::zip};

const INPUT_PATH: &str = "resources/input_1.txt";

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            let left: usize = line[..5].parse().unwrap();
            let right: usize = line[8..].parse().unwrap();
            (left, right)
        })
        .unzip()
}

fn part_1_process(input: &str) -> usize {
    let (mut left_list, mut right_list) = parse(input);
    left_list.sort();
    right_list.sort();
    zip(left_list, right_list)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

pub fn part_1() -> usize {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    part_1_process(&input)
}

fn part_2_process(input: &str) -> usize {
    let (left_list, right_list) = parse(input);
    left_list
        .iter()
        .flat_map(|left| {
            right_list
                .iter()
                .filter(move |&right| left == right)
                .map(move |_| left)
        })
        .sum()
}

pub fn part_2() -> usize {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    part_2_process(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = "00003   00004\n00004   00003\n00002   00005\n00001   00003\n00003   00009\n00003   00003\n";
        let result = part_1_process(&input);
        assert_eq!(result, 11);
    }

    #[test]
    fn part_2_example() {
        let input = "00003   00004\n00004   00003\n00002   00005\n00001   00003\n00003   00009\n00003   00003\n";
        let result = part_2_process(&input);
        assert_eq!(result, 31);
    }

    #[test]
    fn part_1_answer() {
        let result = part_1();
        assert_eq!(result, 1646452);
    }

    #[test]
    fn part_2_answer() {
        let result = part_2();
        assert_eq!(result, 23609874);
    }
}
