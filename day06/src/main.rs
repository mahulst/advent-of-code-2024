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
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
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
fn dir_vector_rev(dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::East => (-1, 0),
        Direction::West => (1, 0),
        Direction::North => (0, 1),
        Direction::South => (0, -1),
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
fn add_reverse_to_map(
    history: &mut HashMap<(i32, i32), Vec<Direction>>,
    map: &HashMap<(i32, i32), Pos>,
    start: (i32, i32),
    direction: Direction,
) {
    let mut pos = start;
    let mut direction = direction;
    let mut count = 0;
    while map.contains_key(&pos) {
        count += 1;
        if count == 1000 {
            return;
        }
        println!("{:?}", pos);
        let direction_vector = dir_vector_rev(&direction);
        let next = (pos.0 + direction_vector.0, pos.1 + direction_vector.1);
        match map.get(&next) {
            Some(pos_in_map) => match pos_in_map {
                Pos::Empty => {
                    pos = next;
                    let entry = history.entry(next).or_default();
                    entry.push(direction);
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
            None => return,
        }
    }
}

fn is_loop(start: (i32, i32), dir: Direction, map: &HashMap<(i32, i32), Pos>) -> bool {
    let mut history = HashSet::new();
    let mut pos = start;
    let mut direction = dir;
    while map.contains_key(&pos) {
        if history.contains(&pos){
            return true
        }
        let direction_vector = dir_vector(&direction);
        let next = (pos.0 + direction_vector.0, pos.1 + direction_vector.1);
        match map.get(&next) {
            Some(pos_in_map) => match pos_in_map {
                Pos::Empty => {
                    pos = next;
                    history.insert(next);
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
// 764 = too low
// 1030 = too low
// 1042  = too low
fn part_2(input: &str) -> usize {
    let (map, start) = parse_input(input);
    let mut history = HashMap::new();
    let mut pos = start;
    let mut direction = Direction::North;
    let mut result = HashSet::new();
    println!("start");
    history.insert(start, vec![direction]);
    add_reverse_to_map(&mut history, &map, start, direction);

    while map.contains_key(&pos) {
        let direction_vector = dir_vector(&direction);
        let next = (pos.0 + direction_vector.0, pos.1 + direction_vector.1);
        match map.get(&next) {
            Some(pos_in_map) => match pos_in_map {
                Pos::Empty => {
                    if let Some(already_been) = history.get(&next) {
                        let prev_direction = match direction {
                            Direction::East => Direction::South,
                            Direction::West => Direction::North,
                            Direction::North => Direction::East,
                            Direction::South => Direction::West,
                        };

                        if already_been.contains(&prev_direction) {
                            let next_next =
                                (next.0 + direction_vector.0, next.1 + direction_vector.1);
                            let not_already_a_device = map.get(&next_next) != Some(&Pos::Device);
                            if not_already_a_device {
                                println!(
                                    "added {:?} because it contains {:?}",
                                    next_next, prev_direction
                                );
                                println!("{:?}", next_next);
                                result.insert(next_next);
                            }
                        }
                    }
                    pos = next;
                    let entry = history.entry(next).or_default();
                    entry.push(direction);
                }
                Pos::Device => {
                    direction = match direction {
                        Direction::East => Direction::South,
                        Direction::West => Direction::North,
                        Direction::North => Direction::East,
                        Direction::South => Direction::West,
                    };
                    println!("adding reverse from , {:?}", pos);
                    add_reverse_to_map(&mut history, &map, pos, direction);
                }
            },
            None => {
                break;
            }
        }
    }
    let mut pos = start;
    let mut direction = Direction::North;

    println!("results:");
    result.len()
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
        assert_eq!(result, 31);
    }
}
