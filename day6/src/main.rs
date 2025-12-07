use std::fs::read_to_string;
use std::ops::{Add, Mul};

type Monoid = (usize, fn(usize, usize) -> usize);

fn parse_input(path: &str) -> (Vec<Vec<usize>>, Vec<Monoid>) {
    let data = read_to_string(path.to_string())
        .unwrap()
        .split("\n")
        .map(|line| {
            line.to_string()
        }).collect::<Vec<_>>();

    let monoids = data
        .last()
        .unwrap()
        .split_whitespace()
        .map(|op| {
            match op.chars().next().unwrap() {
                '+' => (0usize, usize::add as fn(usize, usize) -> usize),
                '*' => (1usize, usize::mul as fn(usize, usize) -> usize),
                _ => panic!("Invalid operator: {}", op)
            }
        }).collect::<Vec<Monoid>>();

    let rows = data[0..data.len() - 1].iter().map(|line| {
        line.split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    (rows, monoids)
}

fn part_one((rows, monoids): &(Vec<Vec<usize>>, Vec<Monoid>)) -> usize {
    let mut sums = monoids
        .iter()
        .map(|(id, _)| *id)
        .collect::<Vec<_>>();
    
    for row in rows {
        for i in 0..row.len() {
            let (_, op) = monoids[i];
            sums[i] = op(sums[i], row[i]);
        }
    }

    sums.iter().sum()
}

fn main() {
    let input = parse_input("input.txt");
    println!("Part one: {}", part_one(&input));
}
