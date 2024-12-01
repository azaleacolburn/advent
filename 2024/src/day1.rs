use itertools::Itertools;
use std::fs;

#[allow(unused_mut)]
pub fn main() {
    let buff = fs::read_to_string("./inputs/day1.txt").unwrap();
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = buff
        .trim()
        .split_whitespace()
        .tuples::<(&str, &str)>()
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .unzip();

    // p.1
    // left.sort();
    // right.sort();
    //
    // let sum: i32 = left
    //     .iter()
    //     .zip(right.iter())
    //     .map(|(a, b)| (a - b).abs())
    //     .sum();
    // println!("{sum}");

    // p.2
    let sim: i32 = left
        .iter()
        .map(|left_num| {
            left_num
                * right
                    .iter()
                    .filter(|right_num| *left_num == **right_num)
                    .collect::<Vec<&i32>>()
                    .len() as i32
        })
        .sum();
    println!("{sim}");
}
