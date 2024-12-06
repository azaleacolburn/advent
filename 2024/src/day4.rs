use std::fs;

pub fn main() {
    let buff = fs::read_to_string("./inputs/day4.txt").unwrap();
    let mut count = 0;
    let mut matrix: Vec<Vec<char>> = buff
        .trim()
        .split('\n')
        .map(|row| row.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    count += count_matches(
        matrix
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>(),
    );
    for i in 0..8 {
        matrix = rotate_45(&matrix, i);
        let buff = matrix_format(&matrix);
        count += count_matches(buff);
    }

    println!("{count}");
}

fn rotate_45<'a>(matrix: &Vec<Vec<char>>, rots: usize) -> Vec<Vec<char>> {
    let old_size = matrix.len();
    let mut size = old_size;
    size = size * 2 - 1;
    let mut new_matrix = vec![vec![' '; size]; size];

    for i in 0..old_size {
        for j in 0..old_size {
            let new_row = i + j;
            let new_col = old_size - 1 - i + j;
            new_matrix[new_row][new_col] = matrix[i][j];
        }
    }

    if rots % 2 == 1 {
        new_matrix
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| *c)
                    .filter(|c| *c != ' ')
                    .collect::<Vec<char>>()
            })
            .filter(|row| !row.is_empty())
            .collect()
    } else {
        new_matrix
    }
}

fn matrix_format(matrix: &Vec<Vec<char>>) -> Vec<String> {
    matrix
        .iter()
        .map(|row| row.iter().filter(|c| **c != ' ').collect::<String>())
        .filter(|row| row != "")
        .collect::<Vec<String>>()
}

fn count_matches(buff: Vec<String>) -> usize {
    buff.iter().fold(0, |acc, row| {
        acc + regex::Regex::new(r"XMAS")
            .unwrap()
            .find_iter(row)
            .map(|m| m.as_str().to_string())
            .collect::<Vec<String>>()
            .len()
    })
}
