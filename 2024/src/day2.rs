use itertools::Itertools;
use std::fs;

pub fn main() {
    let buff = fs::read_to_string("./inputs/day2.txt").unwrap();
    // p.1
    // let safe_levels =
    //     buff.lines()
    //         .map(|l| l.split_whitespace().filter_map(|n| n.parse::<i32>().ok()))
    //         .filter(|level| {
    //             level
    //                 .clone()
    //                 .tuple_windows()
    //                 .all(|(n, next)| (n - next).abs() >= 1 && (n - next).abs() <= 3 && (n < next))
    //                 || level.clone().tuple_windows().all(|(n, next)| {
    //                     (n - next).abs() >= 1 && (n - next).abs() <= 3 && (n > next)
    //                 })
    //         })
    //         .count();
    // println!("{safe_levels}");
    let safe_levels =
        buff.lines()
            .map(|l| l.split_whitespace().filter_map(|n| n.parse::<i32>().ok()))
            .filter(|level| {
                let is_safe = |level: Vec<i32>| -> bool {
                    level.clone().into_iter().tuple_windows().all(|(n, next)| {
                        (n - next).abs() >= 1 && (n - next).abs() <= 3 && (n < next)
                    }) || level.clone().into_iter().tuple_windows().all(|(n, next)| {
                        (n - next).abs() >= 1 && (n - next).abs() <= 3 && (n > next)
                    })
                };

                // let mut problem_indexes =
                //     level
                //         .clone()
                //         .tuple_windows()
                //         .enumerate()
                //         .filter(|(_i, (n, next))| {
                //             (n - next).abs() < 1 || (n - next).abs() > 3 || !(n < next)
                //         })
                //         .chain(level.clone().tuple_windows().enumerate().filter(
                //             |(_i, (n, next))| {
                //                 (n - next).abs() < 1 || (n - next).abs() > 3 || !(n > next)
                //             },
                //         ))
                //         .map(|(i, (_n, _next))| i);
                level.clone().enumerate().any(|(i, _n)| {
                    let mut cloned_level = level.clone().collect::<Vec<i32>>();
                    cloned_level.remove(i);
                    is_safe(cloned_level)
                })
            })
            .count();
    println!("{safe_levels}")
}
