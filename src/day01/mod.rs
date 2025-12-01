use std::fs::read_to_string;

pub fn day01() {
    let rotations = read_to_string("src/day01/input.txt")
        .expect("Error reading file")
        .replace('L', "-")
        .replace('R', "")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("Part 1: {}", count_zero_positions(&rotations, 1));
    println!("Part 2: {}", count_zero_positions(&rotations, 2));
}

fn count_zero_positions(rotations: &Vec<i32>, part: usize) -> usize {
    let mut dial_position: i32 = 50;
    let mut zero_positions: usize = 0;

    for rotation in rotations {
        if part == 2 {
            for clicks in 1..rotation.abs() {
                if (dial_position + clicks * rotation.signum()).rem_euclid(100) == 0 {
                    zero_positions += 1;
                }
            }
        }

        dial_position = (dial_position + rotation).rem_euclid(100);
        if dial_position == 0 {
            zero_positions += 1;
        }
    }

    zero_positions
}
