use itertools::Itertools;
use std::fs::read_to_string;

pub fn day02() {
    let ranges = read_to_string("src/day02/input.txt")
        .expect("Error reading file")
        .split(',')
        .map(|range_string| {
            let parts: Vec<&str> = range_string.split('-').collect();
            (
                parts[0].parse::<i128>().unwrap(),
                parts[1].parse::<i128>().unwrap(),
            )
        })
        .collect::<Vec<(i128, i128)>>();

    println!("Part 1: {}", part1(&ranges));
    println!("Part 2: {}", part2(&ranges));
}

fn part1(ranges: &Vec<(i128, i128)>) -> i128 {
    ranges
        .iter()
        .map(|(start, end)| {
            (*start..=*end).filter(|id| {
                let id_str = id.to_string();
                let (left, right) = id_str.split_at(id_str.len() / 2);
                left == right
            })
        })
        .flatten()
        .dedup()
        .sum()
}

fn part2(ranges: &Vec<(i128, i128)>) -> i128 {
    ranges
        .iter()
        .map(|(start, end)| {
            (*start..=*end).filter(|id| {
                let id_str = id.to_string();
                for i in 1..=id_str.len() / 2 {
                    let (left, right) = id_str.split_at(i);
                    if right == left.repeat(right.len() / left.len()) {
                        return true;
                    }
                }
                false
            })
        })
        .flatten()
        .dedup()
        .sum()
}
