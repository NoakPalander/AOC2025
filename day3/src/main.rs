use std::fs::read_to_string;

fn parse_input(path: &str) -> Vec<Vec<i32>> {
    read_to_string(path.to_string())
        .unwrap()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect()
        }).collect()
}

fn part_one(data: &Vec<Vec<i32>>) -> i32 {
    let index_of_max = |data: &[i32]| {
        let &max = data.iter().max().unwrap();
        data.iter().position(|&x| x == max)
    };
    
    data.iter().fold(0, |sum, bank| {
        let first = index_of_max(bank).unwrap();
        if first != bank.len() - 1 {
            let sub_bank = &bank[first + 1..bank.len()];
            let second = index_of_max(sub_bank).unwrap();
            let energy = bank[first] * 10 + sub_bank[second];
            sum + energy
        }
        else {
            let sub_bank = &bank[0..bank.len() - 1];
            let second = index_of_max(sub_bank).unwrap();
            let energy = sub_bank[second] * 10 + bank[first];
            sum + energy
        }
    })
}

fn main() {
    let input = parse_input("input.txt");
    let ans = part_one(&input);
    println!("{:?}", ans);
}
