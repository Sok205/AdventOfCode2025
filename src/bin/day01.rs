use advent_of_code::read_lines;
use std::io;

fn solve_part1(lines: &[String]) -> i32 {
    let mut password = 0;
    let mut number: i32 = 50;

    for line in lines {
        let dir = line.chars().next().unwrap();
        let value: i32 = line[1..].parse().unwrap();

        match dir {
            'R' => {
                number = (number + value) % 100;
            }
            'L' => {
                number = (number - value).rem_euclid(100);
            }
            _ => panic!("bomboclat"),
        }

        if number == 0 {
            password += 1;
        }
    }

    password
}

fn solve_part2(lines: &[String]) -> i32 {
    0
}

fn main() -> io::Result<()> {
    let lines = read_lines("input/day01.txt")?;

    println!("Day 1");
    println!("Part 1: {}", solve_part1(&lines));
    println!("Part 2: {}", solve_part2(&lines));

    Ok(())
}
