#![allow(unused_variables)]
mod day01;
mod day02;
mod day03;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = if args.len() > 1 {
        args[1].parse::<i32>().unwrap_or_else(|_| {
            println!("Invalid day number");
            std::process::exit(1);
        })
    } else {
        1
    };

    match day {
        1 => day01::day01(),
        2 => day02::day02(),
        3 => day03::day03(),
        4..=25 => println!("Day {} not implemented yet", day),
        _ => println!("Invalid day: {}", day),
    }
}
