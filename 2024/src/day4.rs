use std::fs;

pub fn main() {
    // let mut buff = fs::read_to_string("./inputs/day4.txt").unwrap();
    // let mut count = 0;
    // count += count_matches(&buff);
    // let mut matrix: Vec<Vec<char>> = buff
    //     .trim()
    //     .split('\n')
    //     .map(|row| row.trim().chars().collect::<Vec<char>>())
    //     .collect::<Vec<Vec<char>>>();
    // for i in 0..8 {
    //     matrix = rotate_45(&matrix, i);
    //     buff = matrix
    //         .iter()
    //         .map(|row| row.iter().collect::<String>())
    //         .collect::<Vec<String>>()
    //         .join("\n");
    //     count += count_matches(&buff);
    // }
    //
    // println!("{count}");
    let s = vec![
        vec!['A', 'B', 'C', 'D'],
        vec!['E', 'F', 'G', 'H'],
        vec!['I', 'J', 'K', 'L'],
        vec!['M', 'N', 'O', 'P'],
    ];
    let g = rotate_45(&s, 0);
    let k = rotate_45(&g, 1);
    let h = rotate_45(&k, 2);
    let i = rotate_45(&h, 3);
    let l = rotate_45(&i, 4);
    println!("{:?}", s);
    println!("{:?}", matrix_format(&g));
    println!("{:?}", matrix_format(&k));
    println!("{:?}", matrix_format(&h));
    println!("{:?}", matrix_format(&i));
    println!("{:?}", matrix_format(&l));
}

fn rotate_45<'a>(matrix: &Vec<Vec<char>>, rots: usize) -> Vec<Vec<char>> {
    let old_size = matrix.len();
    let mut size = old_size;
    size = size * 2 - 1;
    let mut new_matrix = vec![vec![' '; size]; size];
    println!("{} {}", size, size);

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

fn count_matches(buff: &str) -> usize {
    regex::Regex::new(r"X M A S")
        .unwrap()
        .find_iter(buff)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<String>>()
        .len()
}

fn rotate_90(matrix: &mut Vec<Vec<&str>>) {
    let len = matrix.len();
    matrix.iter_mut().for_each(|row| row.reverse());

    for i in 0..len {
        for j in i + 1..len {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }
}
