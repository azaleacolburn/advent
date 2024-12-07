use std::fs;

enum GuardDirection {
    Up,
    Down,
    Right,
    Left
}

struct GuardPosition {
    x: usize,
    y: usize,
    direction: GuardDirection
    next_tile: (usize, usize),
}

pub fn main() {
    let buff = fs::read_to_string("./inputs/day6.txt").unwrap();
    let map: Vec<Vec<char>> = buff
        .split("\n")
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    let start_position = map.iter().enumerate().filter_map(|(y, row)| {
        row.iter().enumerate().find_map(|(x, cell)| match cell {
            '>' => Some(GuardPosition {
                x,
                y,
                next_tile: (x + 1, y),
                direction: GuardDirection::Right
            }),
            '<' => Some(GuardPosition {
                x,
                y,
                next_tile: (x - 1, y),
                direction: GuardDirection::Left
            }),
            '^' => Some(GuardPosition {
                x,
                y,
                next_tile: (x, y - 1),
                direction: GuardDirection::Up
            }),

            'v' => Some(GuardPosition {
                x,
                y,
                next_tile: (x, y + 1),
                direction: GuardDirection::Down
            }),
            _ => None,
        })
    });

    let count = move_guard(&map, start_position);
}

fn move_guard(map: &Vec<Vec<char>>, mut pos: GuardPosition) -> usize {
    let mut count = 0;
    match map[pos.next_tile.1][pos.next_tile.0] {
        '.' => {
            (pos.x, pos.y) = (pos.next_tile.0, pos.next_tile.1);
        }
    }
    move_guard(map, pos)
}
