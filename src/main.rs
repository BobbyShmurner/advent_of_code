extern crate advent_of_code;

use advent_of_code::pause;
use clearscreen::clear;

use std::process;

fn main() {
    clear().unwrap();
    println!("Welcome To Bobby Shmurner's Advent Of Code!");

    match advent_of_code::select_day() {
        Ok((answer_1, answer_2)) => {
            println!("Complete!\n\nPart 1: \"{answer_1}\"\nPart 2: \"{answer_2}\"");
        }
        Err(error) => {
            eprintln!("An error occured: {error}");
            process::exit(1);
        }
    };

    pause("\nPress Enter To Exit...");
}
