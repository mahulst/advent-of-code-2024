use std::collections::{HashMap, HashSet};

fn main() {
    let answer_1 = part_1(include_str!("./input.txt"));
    let answer_2 = part_2(include_str!("./input.txt"));

    println!("part_1 {}", answer_1);
    println!("part_2 {}", answer_2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Pos {
    Empty,
    Device,
}
fn parse_input(input: &str) -> (HashMap<(i32, i32), Pos>, (i32, i32)) {
    let mut map = HashMap::new();
    let mut start = (0, 0);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map.insert(
                (x as i32, y as i32),
                match c {
                    '#' => Pos::Device,
                    '^' => {
                        start = (x as i32, y as i32);
                        Pos::Empty
                    }
                    _ => Pos::Empty,
                },
            );
        });
    });

    (map, start)
}
#[derive(Clone, Hash, Debug, Copy, PartialEq, Eq)]
enum Direction {
    East,
    West,
    North,
    South,
}

fn dir_vector(dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::East => (1, 0),
        Direction::West => (-1, 0),
        Direction::North => (0, -1),
        Direction::South => (0, 1),
    }
}

fn part_1(input: &str) -> usize {
    let (map, start) = parse_input(input);
    let mut result = HashSet::new();
    result.insert(start);
    let mut pos = start;
    let mut direction = Direction::North;
    while map.contains_key(&pos) {
        let direction_vector = dir_vector(&direction);
        let next = (pos.0 + direction_vector.0, pos.1 + direction_vector.1);
        match map.get(&next) {
            Some(pos_in_map) => match pos_in_map {
                Pos::Empty => {
                    pos = next;
                    result.insert(next);
                }
                Pos::Device => {
                    direction = match direction {
                        Direction::East => Direction::South,
                        Direction::West => Direction::North,
                        Direction::North => Direction::East,
                        Direction::South => Direction::West,
                    };
                }
            },
            None => return result.len(),
        }
    }

    0
}

fn is_loop(start: (i32, i32), dir: Direction, map: &HashMap<(i32, i32), Pos>) -> bool {
    let mut history = HashSet::new();
    let mut pos = start;
    let mut direction = dir;
    while map.contains_key(&pos) {
        let direction_vector = dir_vector(&direction);
        let next = (pos.0 + direction_vector.0, pos.1 + direction_vector.1);
        if history.contains(&(next, direction)) {
            return true;
        }
        match map.get(&next) {
            Some(pos_in_map) => match pos_in_map {
                Pos::Empty => {
                    pos = next;
                    history.insert((next, direction));
                }
                Pos::Device => {
                    direction = match direction {
                        Direction::East => Direction::South,
                        Direction::West => Direction::North,
                        Direction::North => Direction::East,
                        Direction::South => Direction::West,
                    };
                }
            },
            None => {
                break;
            }
        }
    }

    false
}

fn part_2(input: &str) -> usize {
    let (mut map, start) = parse_input(input);
    let mut pos = start;
    let mut direction = Direction::North;

    let mut original_path = HashSet::new();
    original_path.insert(start);
    while map.contains_key(&pos) {
        let direction_vector = dir_vector(&direction);
        let next = (pos.0 + direction_vector.0, pos.1 + direction_vector.1);
        match map.get(&next) {
            Some(pos_in_map) => match pos_in_map {
                Pos::Empty => {
                    pos = next;
                    original_path.insert(next);
                }
                Pos::Device => {
                    direction = match direction {
                        Direction::East => Direction::South,
                        Direction::West => Direction::North,
                        Direction::North => Direction::East,
                        Direction::South => Direction::West,
                    };
                }
            },
            None => {
                break;
            }
        }
    }
    let mut result = 0;
    let len = original_path.len();
    original_path.into_iter().enumerate().for_each(|(i, pos)| {
        println!("{}/{}", i, len);
        map.insert(pos, Pos::Device);
        if is_loop(start, Direction::North, &map) {
            result += 1;
        }
        map.insert(pos, Pos::Empty);
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        let result = part_1(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
        assert_eq!(result, 41);
    }
    #[test]
    fn test_day_2() {
        let result = part_2(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
        assert_eq!(result, 6);
    }
}
