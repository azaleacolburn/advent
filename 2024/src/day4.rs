use std::fs;

pub fn main() {
    let buff = fs::read_to_string("./inputs/day4.txt").unwrap();
    let regex = regex::Regex::new(r"XMAS").unwrap();
    let mut matches = vec![];
    matches.push(regex.find_iter(&buff));
    let mut matrix = buff
        .trim()
        .split('\n')
        .map(|row| row.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    rotate_90(&mut matrix);
    matches.push(regex.find_iter(&buff));
    rotate_90(&mut matrix);
    matches.push(regex.find_iter(&buff));
    rotate_90(&mut matrix);
    matches.push(regex.find_iter(&buff));
}

fn rotate_45(matrix: &mut Vec<Vec<&str>>) {
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
