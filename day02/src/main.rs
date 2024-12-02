fn main() {
    let answer_1 = part_1(include_str!("./input.txt"));
    let answer_2 = part_2(include_str!("./input.txt"));

    println!("part_1 {}", answer_1);
    println!("part_2 {}", answer_2);
}

fn permutations_with_one_missing<T: Clone>(vec: Vec<T>) -> Vec<Vec<T>> {
    let mut permutations = Vec::new();

    for i in 0..vec.len() {
        let mut perm = vec.clone();
        perm.remove(i);
        permutations.push(perm);
    }

    permutations
}
fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|c| c.parse().ok())
                .collect()
        })
        .filter(|l: &Vec<i32>| {
                Some(l)
                    .map(|l| l.windows(2).map(|a| a[0] - a[1]).collect::<Vec<i32>>())
                    .filter(|l| l.iter().all(|i| *i > 0) || l.iter().all(|i| *i < 0))
                    .filter(|l| l.iter().all(|i| i.abs() >= 1))
                    .filter(|l| l.iter().all(|i| i.abs() <= 3))
                    .is_some()
        })
        .count()
}
fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|c| c.parse().ok())
                .collect()
        })
        .filter(|l: &Vec<i32>| {
            let mut all_posibilities = permutations_with_one_missing(l.clone());
            all_posibilities.push(l.clone());

            all_posibilities.iter().any(|l| {
                Some(l)
                    .map(|l| l.windows(2).map(|a| a[0] - a[1]).collect::<Vec<i32>>())
                    .filter(|l| l.iter().all(|i| *i > 0) || l.iter().all(|i| *i < 0))
                    .filter(|l| l.iter().all(|i| i.abs() >= 1))
                    .filter(|l| l.iter().all(|i| i.abs() <= 3))
                    .is_some()
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        let result = part_1(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
        );
        assert_eq!(result, 2);
    }
    #[test]
    fn test_day_2() {
        let result = part_2(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
        );
        assert_eq!(result, 4);
    }
}
