use std::fs;

use regex::Regex;

#[derive(PartialEq)]
enum Mode {
    On,
    Off,
}

pub fn main() {
    let buff = fs::read_to_string("./inputs/day3.txt").unwrap();
    let reg = Regex::new(r"(mul\([0-9]+,[0-9]+\))|(don't\(\))|(do\(\))").unwrap();
    let matches = reg.find_iter(&buff);
    let mut mode = Mode::On;
    let filtered_instructions = matches
        .map(|insturction| insturction.as_str())
        .filter(|n| match n.chars().nth(2).unwrap() {
            'n' => {
                mode = Mode::Off;
                false
            }
            '(' => {
                mode = Mode::On;
                false
            }
            _ => mode == Mode::On,
        });
    let sum: i32 = filtered_instructions
        .into_iter()
        .map(|mul| {
            let mut chars = mul.chars();
            chars.next();
            chars.next();
            chars.next();
            chars.next();
            chars.next_back();

            let nums = chars
                .collect::<String>()
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            assert_eq!(nums.len(), 2);

            nums[0] * nums[1]
        })
        .sum();
    println!("{sum}");
}
