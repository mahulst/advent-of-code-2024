use std::collections::HashMap;

fn main() {
    let answer_1 = part_1(include_str!("./input.txt"));
    let answer_2 = part_2(include_str!("./input.txt"));

    println!("part_1 {}", answer_1);
    println!("part_2 {}", answer_2);
}

fn parse_input(input: &str) -> HashMap<(i32, i32), char> {
    let mut map = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            map.insert((x as i32, y as i32), c);
        });
    });

    map
}
fn directions() -> [[(i32, i32); 4]; 8] {
    let directions = [
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    directions.map(|(dx, dy)| {
        (0..=3)
            .map(|step| (step * dx, step * dy))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    })
}
fn part_1(input: &str) -> usize {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let directions = directions();
    let word = ['X', 'M', 'A', 'S'];
    let map = parse_input(input);
    let mut matches = 0;
    for y in 0..=height {
        for x in 0..=width {
            let count = directions
                .iter()
                .filter(|steps| {
                    steps.iter().enumerate().all(|(i, step)| {
                        let coord = (step.0 + x, step.1 + y);
                        let expected_letter = word[i];
                        if let Some(letter) = map.get(&coord) {
                            *letter == expected_letter
                        } else {
                            false
                        }
                    })
                })
                .count();
            matches += count;
        }
    }
    matches
}

fn check(
    letters: &[char],
    coords: &[(i32, i32)],
    map: &HashMap<(i32, i32), char>,
    width: i32,
    height: i32,
) -> i32 {
    let mut matches = 0;

    for y in 0..=height {
        for x in 0..=width {
            let is_match = coords.iter().enumerate().all(|(i, step)| {
                let coord = (step.0 + x, step.1 + y);
                let expected_letter = letters[i];
                if let Some(letter) = map.get(&coord) {
                    *letter == expected_letter
                } else {
                    false
                }
            });
            if is_match {
                matches += 1;
            }
        }
    }
    matches
}

fn part_2(input: &str) -> i32 {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let mut matches = 0;
    let map = parse_input(input);

    let word = ['M', 'S', 'A', 'M', 'S'];
    let dirs = [(0, 0), (2, 0), (1, 1), (0, 2), (2, 2)];
    matches += check(&word, &dirs, &map, width, height);
    let word = ['M', 'M', 'A', 'S', 'S'];
    matches += check(&word, &dirs, &map, width, height);
    let word = ['S', 'S', 'A', 'M', 'M'];
    matches += check(&word, &dirs, &map, width, height);
    let word = ['S', 'M', 'A', 'S', 'M'];
    matches += check(&word, &dirs, &map, width, height);
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        let result = part_1(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(result, 18);
    }
    #[test]
    fn test_day_2() {
        let result = part_2(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(result, 9);
    }
}
