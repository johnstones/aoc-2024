mod day_01;
mod day_02;
mod day_03;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return;
    }
    match (args[1].as_str(), args[2].as_str()) {
        ("1", "1") => {
            let result = day_01::part_1();
            println!("{result}");
        }
        ("1", "2") => {
            let result = day_01::part_2();
            println!("{result}");
        }
        ("2", "1") => {
            let result = day_02::part_1();
            println!("{result}");
        }
        ("2", "2") => {
            let result = day_02::part_2();
            println!("{result}");
        }
        ("3", "1") => {
            let result = day_03::part_1();
            println!("{result}");
        }
        ("3", "2") => {
            let result = day_03::part_2();
            println!("{result}");
        }
        _ => {}
    };
}
