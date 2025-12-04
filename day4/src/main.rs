use std::fs::read_to_string;

type Cell = (i32, i32, char);

fn parse_input(path: &str) -> Vec<Vec<Cell>> {
    read_to_string(path.to_string())
        .unwrap()
        .split("\n")
        .map(|line| line.chars())
        .enumerate().map(|(row, line)| {
            line.enumerate()
                .map(|(col, cell)| (row as i32, col as i32, cell))
                .collect()
        }).collect()
}

fn adjacents(data: &Vec<Vec<Cell>>, position: (i32, i32)) -> Vec<Cell> {
    let cols = data.len() as i32;
    let rows = data[0].len() as i32;

    let (y, x) = position;
    let mut out: Vec<Cell> = Vec::new();

    let offsets = [(0, 1), (1, 0), (0, -1), (-1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    for (dy, dx) in offsets.iter() {
        let new_y = y + dy;
        let new_x = x + dx;

        if (new_y >= 0 && new_y < rows) && (new_x >= 0 && new_x < cols) {
            out.push(data[(y + dy) as usize][(x + dx) as usize]);
        }
    }

    out
}

fn part_one(data: &Vec<Vec<Cell>>) -> i32 {
    let mut out = 0;
    for row in data {
        for (r, c, current) in row {
            let count = adjacents(&data, (*r, *c))
                .iter()
                .filter(|&&(_, _, key)| key == '@')
                .count();

            if count < 4 && *current == '@' {
                out += 1;
            }
        }
    }

    out
}

fn part_two(input: &Vec<Vec<Cell>>) -> i32 {
    let mut out = 0;
    let mut data = input.clone();

    loop {
        let mut rolls = 0;
        let mut cleared: Vec<(i32, i32)> = Vec::new();

        for row in &data {
            for &(r, c, current) in row {
                let count = adjacents(&data, (r, c))
                    .iter()
                    .filter(|&&(_, _, key)| key == '@')
                    .count();

                if count < 4 && current == '@' {
                    cleared.push((r, c));
                    rolls += 1;
                }
            }
        }

        if (rolls == 0) {
            break
        }
        else {
            out += rolls;
            for (r, c) in cleared {
                data[r as usize][c as usize] = (r, c, '.');
            }
        }
    }

    out
}

fn main() {
    let input = parse_input("input.txt");
    let ans = part_two(&input);
    println!("Part one: {}", ans);
}
