use std::fs;

#[allow(unused_mut)]
pub fn main() {
    let buff = fs::read_to_string("./inputs/day4.txt").unwrap();
    let mut matrix: Vec<Vec<char>> = buff
        .trim()
        .split('\n')
        .map(|row| {
            row.trim()
                .chars()
                .map(|c| c.to_ascii_lowercase())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let mut count = 0;
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix.len() - 1 {
            //            println!(" {}", matrix[i - 1][j - 1]);
            if matrix[i][j] != 'a' {
                continue;
            }
            match matrix[i - 1][j - 1] {
                'm' => {
                    if (matrix[i - 1][j + 1] == 'm'
                        && matrix[i + 1][j + 1] == 's'
                        && matrix[i + 1][j - 1] == 's')
                        || (matrix[i + 1][j - 1] == 'm'
                            && matrix[i - 1][j + 1] == 's'
                            && matrix[i + 1][j + 1] == 's')
                    {
                        count += 1;
                        continue;
                    }
                }
                's' => {
                    if (matrix[i - 1][j + 1] == 's'
                        && matrix[i + 1][j + 1] == 'm'
                        && matrix[i + 1][j - 1] == 'm')
                        || (matrix[i + 1][j - 1] == 's'
                            && matrix[i - 1][j + 1] == 'm'
                            && matrix[i + 1][j + 1] == 'm')
                    {
                        count += 1;
                        continue;
                    }
                }
                _ => {}
            }
            match matrix[i + 1][j + 1] {
                'm' => {
                    if (matrix[i - 1][j + 1] == 'm'
                        && matrix[i + 1][j + 1] == 's'
                        && matrix[i + 1][j - 1] == 's')
                        || (matrix[i + 1][j - 1] == 'm'
                            && matrix[i - 1][j + 1] == 's'
                            && matrix[i + 1][j + 1] == 's')
                    {
                        count += 1;
                        continue;
                    }
                }
                's' => {
                    if (matrix[i - 1][j + 1] == 's'
                        && matrix[i + 1][j + 1] == 'm'
                        && matrix[i + 1][j - 1] == 'm')
                        || (matrix[i + 1][j - 1] == 's'
                            && matrix[i - 1][j + 1] == 'm'
                            && matrix[i + 1][j + 1] == 'm')
                    {
                        count += 1;
                        continue;
                    }
                }
                _ => {}
            }
        }
    }
    println!("{count}");

    // p. 1
    // let buff = matrix_format(&matrix).join("\n");
    // println!("{buff}");
    // let mut count = count_matches(
    //     matrix
    //         .iter()
    //         .map(|row| row.iter().collect::<String>())
    //         .collect::<Vec<String>>(),
    // );
    // for i in 0..7 {
    //     matrix = rotate_45(&matrix, i);
    //     let buff = matrix_format(&matrix);
    //     println!("{}", buff.join("\n"));
    //     count += count_matches(buff);
    // }
    //
    // println!("{count}");
}

fn rotate_45(matrix: &Vec<Vec<char>>, rots: usize) -> Vec<Vec<char>> {
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

    match rots % 2 == 1 {
        true => new_matrix
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| *c)
                    .filter(|c| *c != ' ')
                    .collect::<Vec<char>>()
            })
            .filter(|row| !row.is_empty())
            .collect(),
        false => new_matrix,
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
            .collect::<Vec<_>>()
            .len()
    })
}
