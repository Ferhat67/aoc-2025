use itertools::Itertools;
use std::fs::read_to_string;
use std::ops::RangeInclusive;

pub fn day05() {
    let (ranges, ids) = parse_input();
    println!("Part 1: {}", part1(&ranges, &ids));
    println!("Part 1: {}", part2(&ranges));
}

fn parse_input() -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    let input = read_to_string("src/day05/input.txt").unwrap();
    let parts: Vec<String> = input.split("\n\n").map(String::from).collect();
    let ranges = parts[0]
        .lines()
        .map(|l| {
            l.split("-")
                .map(|p| p.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|x| x[0]..=x[1])
        .collect::<Vec<RangeInclusive<i64>>>();
    let ids = parts[1]
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    (ranges, ids)
}

fn part1(ranges: &Vec<RangeInclusive<i64>>, ids: &Vec<i64>) -> i64 {
    ids.iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count() as i64
}

fn part2(ranges: &Vec<RangeInclusive<i64>>) -> i64 {
    let sorted_ranges: Vec<RangeInclusive<i64>> = ranges
        .iter()
        .cloned()
        .sorted_unstable_by_key(|r| *r.start())
        .collect();

    let mut non_overlapping_ranges = vec![];
    for range in sorted_ranges.into_iter() {
        if non_overlapping_ranges.is_empty() {
            non_overlapping_ranges.push(range);
            continue;
        }

        let current_end = non_overlapping_ranges.last().unwrap().end();
        if range.start() > current_end {
            non_overlapping_ranges.push(range);
        } else if range.end() > current_end {
            non_overlapping_ranges.push((current_end + 1)..=range.end().clone());
        }
    }
    non_overlapping_ranges
        .iter()
        .map(|r| r.end() - r.start() + 1)
        .sum::<i64>()
}
