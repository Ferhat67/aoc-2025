use itertools::Itertools;
use std::fs::read_to_string;

pub fn day03() {
    let battery_banks = parse_input();
    println!("Part 1: {}", part1(&battery_banks));
    println!("Part 2: {}", part2(&battery_banks));
}
fn parse_input() -> Vec<Vec<char>> {
    read_to_string("src/day03/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn part1(battery_banks: &Vec<Vec<char>>) -> usize {
    let mut max_joltages: Vec<usize> = vec![];

    for bank in battery_banks {
        let mut max_joltage = 0;
        for (index, battery_1) in bank.iter().enumerate() {
            for battery_2 in bank.iter().skip(index + 1) {
                let combined_joltage = format!("{}{}", battery_1, battery_2)
                    .parse::<usize>()
                    .unwrap();
                if combined_joltage > max_joltage {
                    max_joltage = combined_joltage;
                }
            }
        }
        max_joltages.push(max_joltage);
    }

    max_joltages.iter().sum()
}

fn part2(battery_banks: &Vec<Vec<char>>) -> i128 {
    battery_banks
        .iter()
        .map(|bank| {
            bank.iter()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .map(|bank| find_largest_total_joltage(bank, 12))
        .sum()
}

fn find_largest_total_joltage(bank: Vec<usize>, total_batteries: usize) -> i128 {
    let mut window_start = 0;
    let mut joltages = Vec::new();

    for batteries_remaining in (0..total_batteries).rev() {
        let window_end = bank.len() - batteries_remaining;
        let (joltage_index, joltage) =
            find_largest_joltage_in_window(&bank[window_start..window_end]);
        joltages.push(*joltage);
        window_start += joltage_index + 1;
    }

    joltages.iter().join("").parse::<i128>().unwrap()
}

fn find_largest_joltage_in_window(window: &[usize]) -> (usize, &usize) {
    let max_joltage = window.iter().max().unwrap();
    window
        .iter()
        .find_position(|&joltage| joltage == max_joltage)
        .unwrap()
}
