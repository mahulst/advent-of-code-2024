use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::digit1,
    combinator::{map_res, recognize},
    sequence::{preceded, tuple},
    IResult,
};

fn number_with_length(input: &str) -> IResult<&str, u32> {
    map_res(
        take_while_m_n(1, 3, |c: char| c.is_ascii_digit()),
        |num_str: &str| num_str.parse::<u32>(),
    )(input)
}

fn mul_with_numbers(input: &str) -> IResult<&str, (u32, u32)> {
    let (rest, (_mul, _par, first, _comma, secon, _par_close)) = tuple((
        tag("mul"),
        tag("("),
        number_with_length,
        tag(","),
        number_with_length,
        tag(")"),
    ))(input)?;

    Ok((rest, (first, secon)))
}

fn find_all_mul_with_numbers(input: &str) -> Vec<(u32, u32)> {
    let mut results = Vec::new();
    let mut offset = 0;
    let input_len = input.len();

    while offset < input_len {
        let current_input = &input[offset..];
        match mul_with_numbers(current_input) {
            Ok((rest, (n, m))) => {
                results.push((n, m));
                let nomnom = current_input.len() - rest.len();
                offset += nomnom;
            }
            Err(_) => {
                offset += 1;
            }
        }
    }
    results
}
fn main() {
    let answer_1 = part_1(include_str!("./input.txt"));
    let answer_2 = part_2(include_str!("./input.txt"));

    println!("part_1 {}", answer_1);
    println!("part_2 {}", answer_2);
}

fn part_1(input: &str) -> u32 {
    let a = find_all_mul_with_numbers(input);
    a.iter().map(|(v1, v2)| v1 * v2).sum()
}

fn remove_dont_untill_do(s: &str) -> String {
    let mut result = String::new();
    let mut pos = 0;

    loop {
        if let Some(dont_pos) = s[pos..].find("don't()") {
            result.push_str(&s[pos..pos + dont_pos]);
            pos = pos + dont_pos + 7;

            if let Some(do_pos) = s[pos..].find("do()") {
                pos += do_pos;
            } else {
                break;
            }
        } else {
            result.push_str(&s[pos..]);
            break;
        }
    }

    result
}
fn part_2(input: &str) -> u32 {
    let a = remove_dont_untill_do(input);
    dbg!(&a);
    dbg!(&input);

    part_1(&a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        let result =
            part_1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, 161);
    }
    #[test]
    fn test_day_2() {
        let result =
            part_2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, 48);
    }
}
