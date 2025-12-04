use std::fs::read_to_string;

pub fn day04() {
    let paper_rolls = parse_input();
    println!("Part 1: {}", part1(&paper_rolls));
    println!("Part 2: {}", part2(&paper_rolls));
}
fn parse_input() -> Vec<Vec<char>> {
    read_to_string("src/day04/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn part1(paper_rolls: &Vec<Vec<char>>) -> usize {
    get_accessible_rolls(paper_rolls).len()
}

fn get_accessible_rolls(paper_rolls: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut accessible_rolls = Vec::new();
    for y in 0..paper_rolls.len() {
        for x in 0..paper_rolls[y].len() {
            if paper_rolls[y][x] == '@' && is_accessible(paper_rolls, x, y) {
                accessible_rolls.push((x, y));
            }
        }
    }
    accessible_rolls
}

fn is_accessible(paper_rolls: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let (x,y) = (x as i32, y as i32);
    let x_bounds = 0..paper_rolls.get(0).unwrap().len() as i32;
    let y_bounds = 0..paper_rolls.len() as i32;
    let adjacent_positions = [
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
        (x - 1, y - 1),
        (x + 1, y + 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
    ];

    adjacent_positions
        .iter()
        .filter(|(x2, y2)| x_bounds.contains(x2) && y_bounds.contains(y2))
        .filter(|(x2, y2)| paper_rolls[*y2 as usize][*x2 as usize] == '@')
        .count() < 4
}

fn part2(paper_rolls: &Vec<Vec<char>>) -> usize {
    let mut paper_rolls = paper_rolls.clone();
    loop {
        let accessible_rolls = get_accessible_rolls(&paper_rolls);
        accessible_rolls.iter().for_each(|&(x, y)| paper_rolls[y][x] = 'x');
        if accessible_rolls.len() == 0 {
            break;
        }
    }
    paper_rolls.iter().flatten().filter(|&&c| c == 'x').count()
}
