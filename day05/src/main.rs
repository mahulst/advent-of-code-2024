use std::{cmp::Ordering, collections::{HashMap, HashSet}};

fn main() {
    let answer_1 = part_1(include_str!("./input.txt"), include_str!("./input2.txt"));
    let answer_2 = part_2(include_str!("./input.txt"), include_str!("./input2.txt"));

    println!("part_1 {}", answer_1);
    println!("part_2 {}", answer_2);
}

fn part_1(input: &str, input2: &str) -> usize {
    let mut before: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut after: HashMap<i32, HashSet<i32>> = HashMap::new();
    input
        .lines()
        .map(|line| line.split("|").filter_map(|c| c.parse().ok()).collect())
        .map(|l: Vec<i32>| (l[0], l[1]))
        .for_each(|(page, before_page)| {
            let pages = before.entry(page).or_default();
            pages.insert(before_page);

            let pages = after.entry(before_page).or_default();
            pages.insert(page);
        });
    input2
        .lines()
        .filter(|line| {
            let pages: Vec<i32> = line.split(",").filter_map(|c| c.parse().ok()).collect();
            pages.iter().enumerate().all(|(index, page)| {
                if let Some(must_be_after) = after.get(page) {
                    let a = pages[index + 1..].iter().any(|pages_after_current| {
                        let page_is_before_where_it_shouldnt =
                            must_be_after.contains(pages_after_current);

                        page_is_before_where_it_shouldnt
                    });
                    !a
                } else {
                    true
                }
            })
        })
        .map(|line| {
            let pages: Vec<i32> = line.split(",").filter_map(|c| c.parse().ok()).collect();
            let len = (pages.len() as f32 / 2.0).floor();

            let a = pages[len as usize] as usize;
            a
        })
        .sum()
}

fn part_2(input: &str, input2: &str) -> usize {
    let mut before: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut after: HashMap<i32, HashSet<i32>> = HashMap::new();
    input
        .lines()
        .map(|line| line.split("|").filter_map(|c| c.parse().ok()).collect())
        .map(|l: Vec<i32>| (l[0], l[1]))
        .for_each(|(page, before_page)| {
            let pages = before.entry(page).or_default();
            pages.insert(before_page);

            let pages = after.entry(before_page).or_default();
            pages.insert(page);
        });
    before.iter().fold(Vec::new(), |mut acc, (page, befores)| {
        acc.push(page);
        acc
    });
    input2
        .lines()
        .filter(|line| {
            let pages: Vec<i32> = line.split(",").filter_map(|c| c.parse().ok()).collect();
            !pages.iter().enumerate().all(|(index, page)| {
                if let Some(must_be_after) = after.get(page) {
                    let a = pages[index + 1..].iter().any(|pages_after_current| {
                        let page_is_before_where_it_shouldnt =
                            must_be_after.contains(pages_after_current);

                        page_is_before_where_it_shouldnt
                    });
                    !a
                } else {
                    true
                }
            })
        })
        .map(|line| {
            let mut pages: Vec<i32> = line.split(",").filter_map(|c| c.parse().ok()).collect();
            pages.sort_by(|a, b| {
                if let Some(should_be_before) = before.get(b) {
                    match should_be_before.contains(a) {
                        true => Ordering::Less,
                        false => Ordering::Greater,
                    }
                } else {
                    Ordering::Greater
                }
            });

            let len = (pages.len() as f32 / 2.0).floor();

            let a = pages[len as usize] as usize;
            a
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        let result = part_1(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13",
            "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(result, 143);
    }
    #[test]
    fn test_day_2() {
        let result = part_2(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13",
            "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(result, 123);
    }
}
