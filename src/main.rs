mod day_1;
mod day_2;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return;
    }
    match (args[1].as_str(), args[2].as_str()) {
        ("1", "1") => {
            let result = day_1::part_1();
            println!("{result}");
        }
        ("1", "2") => {
            let result = day_1::part_2();
            println!("{result}");
        }
        ("2", "1") => {
            let result = day_2::part_1();
            println!("{result}");
        }
        ("2", "2") => {
            let result = day_2::part_2();
            println!("{result}");
        }
        _ => {}
    };
}
