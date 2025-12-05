use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::ops::RangeInclusive;
use std::time::SystemTime;
use itertools::Itertools;

type IdRanges = Vec<RangeInclusive<usize>>;
type Ids = Vec<usize>;

fn parse_input(path: &str) -> (IdRanges, Ids) {
    let data: Vec<_> = read_to_string(path.to_string())
        .unwrap()
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    let ranges = data[0]
        .iter()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse::<usize>().unwrap()..=end.parse::<usize>().unwrap()
        })
        .collect::<Vec<_>>();

    let ids = data[1]
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (ranges, ids)
}

fn part_one((ranges, ids): &(IdRanges, Ids)) -> usize {
    ids.iter()
        .filter(|&&id| ranges.iter().any(|range| range.contains(&id)))
        .count()
}

fn part_two((ranges, _): &(IdRanges, Ids)) -> usize {
    ranges
        .clone()
        .into_iter()
        .sorted_by_key(|r| *r.start())
        .coalesce(|a, b| {
            if *b.start() <= *a.end() {
                let start = *a.start();
                let end = (*a.end()).max(*b.end());
                Ok(start..=end)
            } else {
                Err((a, b))
            }
        })
        .fold(0, |sum, elem| {
            sum + (elem.end() - elem.start() + 1)
        })
}

fn main() {
    let input = parse_input("input.txt");
    let ans_1 = part_one(&input);
    let ans_2 = part_two(&input);

    println!("Part 1: {}", ans_1);
    println!("Part 2: {}", ans_2);
}
