use std::env;
mod exercises;
mod utils;

use exercises::days;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(value) = args.get(1) {
        match &value[..] {
            "day1" => {
                days::day1::main();
            }
            "day2" => {
                days::day2::main();
            }
            "day3" => {
                days::day3::main();
            }
            "day4" => {
                days::day4::main();
            }
            "day5" => {
                days::day5::main();
            }
            _ => (),
        }
    } else {
        println!("lol that is not valid input dawg (hint: cargo run day2)");
    }
}
