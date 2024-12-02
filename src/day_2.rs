use std::fs;

const INPUT_PATH: &str = "resources/input_02.txt";

fn parse(report: &str) -> Vec<usize> {
    report
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn is_safe(levels: &[usize]) -> bool {
    let is_increasing = levels.iter().is_sorted();
    let is_decreasing = levels.iter().rev().is_sorted();
    let is_stable = levels.windows(2).all(|pair| {
        let dx = pair[0].abs_diff(pair[1]);
        dx >= 1 && dx <= 3
    });
    (is_increasing || is_decreasing) && is_stable
}

fn is_safe_dampened(levels: &[usize]) -> bool {
    (0..levels.len())
        .map(|i| [&levels[..i], &levels[i + 1..levels.len()]].concat())
        .any(|levels_dampened| is_safe(&levels_dampened))
}

fn part_1_process(input: &str) -> usize {
    input
        .lines()
        .map(parse)
        .filter(|levels| is_safe(levels))
        .count()
}

fn part_2_process(input: &str) -> usize {
    input
        .lines()
        .map(parse)
        .filter(|levels| is_safe_dampened(levels))
        .count()
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

    #[test]
    fn part_1_example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let result = part_1_process(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_2_example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let result = part_2_process(&input);
        assert_eq!(result, 4);
    }

    #[test]
    fn part_1_answer() {
        let result = part_1();
        assert_eq!(result, 321);
    }

    #[test]
    fn part_2_answer() {
        let result = part_2();
        assert_eq!(result, 386);
    }
}
