use std::fs::read_to_string;

fn read_input(path: &str) -> Vec<(usize, usize)> {
    read_to_string(path.to_string())
        .expect("Failed to read input file")
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|line| {
            let (start, end) = line.split_at(line.find("-").unwrap());
            (start.parse().unwrap(), end[1..].parse().unwrap())
        })
        .collect()
}

fn part_one(data: &Vec<(usize, usize)>) -> usize {
    let mut sum = 0;
    for &(start, end) in data {
        for id in start..=end {
            let str = id.to_string();
            if str.len() % 2 == 0 {
                let (left, right) = str.split_at(str.len() / 2);
                if left == right {
                    sum += id;
                }
            }
        }
    }

    sum
}

fn part_two(data: &Vec<(usize, usize)>) -> usize {
    let mut sum = 0;
    for &(start, end) in data {
        for id in start..=end {
            let str = id.to_string();
            if str.len() % 2 == 0 {
                let (left, right) = str.split_at(str.len() / 2);
                if left == right {
                    sum += id;
                }
            }
        }
    }

    sum
}

fn main() {
    let input = read_input("input.txt");
    let ans = part_two(&input);
    println!("{}", ans);
}
