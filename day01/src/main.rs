fn main() {
    let answer_1 = part_1(include_str!("./input.txt"));
    let answer_2 = part_2(include_str!("./input.txt"));

    println!("part_1 {}", answer_1);
    println!("part_2 {}", answer_2);
}

fn part_1(input: &str) -> i32 {
    let (mut first, mut second): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|c| c.parse().ok())
                .collect()
        })
        .map(|l: Vec<i32>| (l[0], l[1]))
        .unzip();

    first.sort();
    second.sort();
    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}
fn part_2(input: &str) -> i32 {
    let (mut first, mut second): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|c| c.parse().ok())
                .collect()
        })
        .map(|l: Vec<i32>| (l[0], l[1]))
        .unzip();

    first.sort();
    second.sort();
    first
        .iter()
        .map(|f| f * second.iter().filter(|s| *s == f).count() as i32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        let result = part_1(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, 11);
    }
    #[test]
    fn test_day_2() {
        let result = part_2(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, 31);
    }
}
