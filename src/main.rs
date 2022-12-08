extern crate advent_of_code;

use advent_of_code::cls;

use std::process;

fn main() {
    cls();
    println!("Welcome To Bobby Shmurner's Advent Of Code!");

    match advent_of_code::select_day() {
        Ok((answer_1, answer_2)) => {
            println!("Answer 1: {}\nAnswer 2: {}", answer_1, answer_2);
        }
        Err(error) => {
            eprintln!("An error occured: {}", error);
            process::exit(1);
        }
    };
}