use std::{collections::HashMap, fs};

use itertools::Itertools;

pub fn main() {
    let buff = fs::read_to_string("./inputs/day5.txt").unwrap();

    let (rules_buff, updates_buff) = buff
        .trim()
        .split("\n\n")
        .into_iter()
        .map(|section| section.trim().split("\n").collect::<Vec<&str>>())
        .collect_tuple::<(Vec<&str>, Vec<&str>)>()
        .unwrap();

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    rules_buff
        .into_iter()
        .map(|rule| rule.split("|").collect_tuple::<(&str, &str)>().unwrap())
        .map(|(first, second)| {
            (
                first.parse::<i32>().unwrap(),
                second.parse::<i32>().unwrap(),
            )
        })
        .for_each(|(first, second)| {
            rules
                .entry(first)
                .and_modify(|rule| rule.push(second))
                .or_insert(vec![second]);
        });

    let updates: Vec<Vec<i32>> = updates_buff
        .into_iter()
        .map(|update| {
            update
                .trim()
                .split(",")
                .into_iter()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    // p. 2
    let middle_corrected_sum: i32 = updates
        .into_iter()
        .filter_map(|update| {
            let mut pre_curr: Vec<i32> = vec![];
            let mut was_invalid = false;
            update.iter().for_each(|num| {
                if let Some(these_rules) = rules.get(&num) {
                    let correct_position = these_rules
                        .iter()
                        .filter_map(|rule| pre_curr.iter().position(|n| n == rule))
                        .min()
                        .unwrap_or(pre_curr.len());
                    pre_curr.insert(correct_position, *num);
                    was_invalid |= correct_position != pre_curr.len() - 1;
                } else {
                    pre_curr.push(*num);
                    was_invalid |= false
                }
            });
            match was_invalid {
                true => Some(pre_curr[(pre_curr.len() - 1) / 2]),
                false => None,
            }
        })
        .sum();

    // p. 1
    // let middle_correct_sum: i32 = updates
    //     .into_iter()
    //     .filter_map(|update| {
    //         let mut pre_curr: Vec<i32> = vec![];
    //         let is_invalid = update.iter().any(|num| {
    //             let is_invalid = if let Some(these_rules) = rules.get(&num) {
    //                 these_rules.iter().any(|rule| pre_curr.contains(&rule))
    //             } else {
    //                 false
    //             };
    //             pre_curr.push(*num);
    //             is_invalid
    //         });
    //         match is_invalid {
    //             true => None,
    //             false => Some(update[(update.len() - 1) / 2]),
    //         }
    //     })
    //     .sum();
    println!("{middle_corrected_sum}");
}
