use itertools::Itertools;
use regex::Regex;
use std::fs::read_to_string;
use std::usize::MAX;

pub fn day06() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> i64 {
    let whitespace_pattern = Regex::new(r"\s+").unwrap();
    let worksheet: Vec<Vec<String>> = read_to_string("src/day06/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            whitespace_pattern
                .replace_all(line.trim(), " ")
                .split(" ")
                .map(String::from)
                .collect()
        })
        .collect();

    worksheet
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, operator)| {
            let numbers: Vec<String> = worksheet[0..worksheet.len() - 1]
                .iter()
                .map(|row| row[i].clone())
                .collect();
            do_the_math(numbers, &operator)
        })
        .sum()
}

fn do_the_math(numbers: Vec<String>, operator: &String) -> i64 {
    match operator.as_str() {
        "+" => numbers
            .iter()
            .map(|s| s.trim().parse::<i64>().unwrap())
            .sum::<i64>(),
        "*" => numbers
            .iter()
            .map(|s| s.trim().parse::<i64>().unwrap())
            .product::<i64>(),
        _ => panic!("Invalid operator: {}", operator),
    }
}

fn part2() -> i64 {
    let worksheet: Vec<Vec<String>> = read_to_string("src/day06/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(String::from).collect())
        .collect();

    let mut sum = 0;
    for (index, operator) in worksheet.last().unwrap().iter().enumerate() {
        if operator == "+" || operator == "*" {
            let mut numbers = vec![read_number_vertically(&worksheet, index)];
            for next_index in (index + 1)..MAX {
                let next_number = read_number_vertically(&worksheet, next_index);
                if next_number.trim().is_empty() {
                    break;
                }
                numbers.push(next_number);
            }
            sum += do_the_math(numbers, &operator)
        }
    }
    sum
}

fn read_number_vertically(worksheet: &Vec<Vec<String>>, column: usize) -> String {
    worksheet[0..worksheet.len() - 1]
        .iter()
        .map(|row| row.get(column).map_or("", |s| s.as_str()))
        .join("")
}
