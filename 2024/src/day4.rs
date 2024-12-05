use std::fs;

pub fn main() {
    let buff = fs::read_to_string("./inputs/day4.txt").unwrap();
    let matrix = buff
        .split('\n')
        .map(|row| row.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
}
