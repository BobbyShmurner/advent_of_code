extern crate advent_of_code;

use advent_of_code::pause;
use clearscreen::clear;

fn main() {
    loop {
        clear().unwrap();
        println!("Welcome To Bobby Shmurner's Advent Of Code!");

        match advent_of_code::select_day() {
            Ok((answer_1, answer_2, micros)) => {
                println!(
                    "Completed in {:.3} Milliseconds!\n\nPart 1: {answer_1}\nPart 2: {answer_2}",
                    micros as f32 / 1000.0
                );
            }
            Err(error) => {
                eprintln!("An error occured: {error}");
            }
        };

        pause("\nPress Enter To Continue...");
    }
}
