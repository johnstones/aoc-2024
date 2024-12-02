mod day_1;

use std::env;

use day_1::{part_1, part_2};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return;
    }
    match (args[1].as_str(), args[2].as_str()) {
        ("1", "1") => {
            let result = part_1();
            println!("{result}");
        }
        ("1", "2") => {
            let result = part_2();
            println!("{result}");
        }
        _ => {}
    };
}
