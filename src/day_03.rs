use regex::Regex;
use std::fs;

const INPUT_PATH: &str = "resources/input_03.txt";

fn parse(input: &str) -> Vec<(&str, Option<(usize, usize)>)> {
    let pattern = Regex::new(r"(?<action>mul|do|don't)\(((?<x>[0-9]+),(?<y>[0-9]+))?\)").unwrap();
    let mut result = Vec::new();
    for caps in pattern.captures_iter(input) {
        match &caps["action"] {
            "mul" => {
                let x = caps["x"].parse().unwrap();
                let y = caps["y"].parse().unwrap();
                result.push(("mul", Some((x, y))));
            }
            "do" => result.push(("do", None)),
            "don't" => result.push(("don't", None)),
            _ => {}
        }
    }
    result
}

fn part_1_process(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|action| match action {
            ("mul", Some((x, y))) => x * y,
            _ => 0,
        })
        .sum()
}

fn part_2_process(input: &str) -> usize {
    let mut result = 0;
    let mut doing = true;
    for action in parse(input).iter() {
        match action {
            ("mul", Some((x, y))) => {
                if doing {
                    result += x * y
                }
            }
            ("do", None) => doing = true,
            ("don't", None) => doing = false,
            _ => {}
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

    const EXAMPLE_INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part_1_example() {
        let result = part_1_process(EXAMPLE_INPUT);
        assert_eq!(result, 161);
    }

    #[test]
    fn part_2_example() {
        let result = part_2_process(EXAMPLE_INPUT);
        assert_eq!(result, 48);
    }

    #[test]
    fn part_1_answer() {
        let result = part_1();
        assert_eq!(result, 185797128);
    }

    #[test]
    fn part_2_answer() {
        let result = part_2();
        assert_eq!(result, 89798695);
    }
}
