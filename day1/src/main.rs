use std::fs::{read_to_string, File};
use std::io;

fn parse_input(path: &str) -> Vec<(char, i32)> {
    read_to_string(path.to_string())
        .expect("Failed to read the file")
        .lines()
        .map(|s| {
            let (direction, steps) = s.split_at(1);
            (direction.chars().next().unwrap(), steps.parse().unwrap())
        }).collect::<Vec<_>>()
}

fn part_one(input: &Vec<(char, i32)>) -> i32 {
    let mut dial = 50;
    let mut zeros = 0;
    for &(direction, steps) in input {
        if direction == 'R' {
            dial = (dial + steps).rem_euclid(100);
        }
        else {
            dial = (dial - steps).rem_euclid(100);
        }

        if dial == 0 {
            zeros += 1;
        }
    }

    zeros
}

fn part_two(input: &Vec<(char, i32)>) -> i32 {
    let mut dial = 50;
    let mut zeros = 0;

    for &(direction, steps) in input {
        if direction == 'R' {
            zeros += (dial + steps) / 100;
            dial = ((dial + steps) % 100 + 100) % 100;
        } else {
            let diff = if dial == 0 {
                steps - 100
            } 
            else {
                steps - dial
            };

            if diff >= 0 {
                zeros += 1 + diff / 100;
            }
            
            dial = ((dial - steps) % 100 + 100) % 100;
        }
    }

    zeros
}

fn main() {
    let input = parse_input("input.txt");
    let answer_2 = part_two(&input);

    println!("Part 2: {}", answer_2);
}
