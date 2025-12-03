use itertools::Itertools;

use crate::SolverError;

pub fn solve(input: &str) -> Result<(String, String), SolverError> {
    let part_1 = solve_part_1(input)?;
    let part_2 = solve_part_2(input)?;
    Ok((part_1, part_2))
}

fn solve_part_1(input: &str) -> Result<String, SolverError> {
    let result = input.lines()
        .map(|line| solve_line_part_1(line))
        .fold(Ok(0), |acc, ele| {
            let Ok(acc) = acc else {
                return acc
            };
            let Ok(ele) = ele else {
                return ele
            };
            Ok(acc + ele)
        })?;
    Ok(result.to_string())
}

fn solve_line_part_1(line: &str) -> Result<u32, SolverError> {
    // First digit is the highest digit in the input that is NOT the last digit in the input.
    let digits = parse_line(line).collect::<Result<Vec<_>,SolverError>>()?;
    let (first_index, first) = digits[0..(digits.len() - 1)].iter()
        .enumerate()
        .rev()
        .max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();

    let second = digits[(first_index + 1)..digits.len()].iter().max().unwrap();

    Ok(*first * 10 + *second)
}

fn solve_part_2(input: &str) -> Result<String, SolverError> {
    let result = input.lines()
        .map(|line| solve_line_part_2(line))
        .fold(Ok(0), |acc, ele| {
            let Ok(acc) = acc else {
                return acc
            };
            let Ok(ele) = ele else {
                return ele
            };
            Ok(acc + ele)
        })?;

    Ok(result.to_string())
}

fn solve_line_part_2(line: &str) -> Result<u64, SolverError> {
    let mut digits = parse_line(line).collect::<Result<Vec<_>, _>>()?;
    let num_digits = digits.len();

    let lowest = digits.iter()
        .enumerate()
        .sorted_by(|(_, a), (_, b)| a.cmp(b))
        .take(num_digits - 12)
        .map(|t| t.0)
        .sorted()
        .rev()
        .collect::<Vec<_>>();

    for (index, digit) in digits.iter().enumerate() {
        if lowest.contains(&index) {
            print!("\x1b[31m{digit}");
        } else {
            print!("\x1b[32m{digit}");
        }
    }

    for i in lowest {
        digits.remove(i);
    }

    let result = digits.into_iter()
        .map(|d| d.to_string())
        .collect::<String>()
        .parse::<u64>().unwrap();

    println!("\x1b[0m -> {result}");
    
    Ok(result)
}

fn parse_line(line: &str) -> impl Iterator<Item = Result<u32, SolverError>> {
    line.chars()
        .map(|c| match c {
            d @ '1'..='9' => Ok(d.to_digit(10).unwrap()),
            _ => Err(SolverError::BadInput),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_lines_part_1() {
        assert_eq!(solve_line_part_1("987654321111111").unwrap(), 98);
        assert_eq!(solve_line_part_1("811111111111119").unwrap(), 89);
        assert_eq!(solve_line_part_1("234234234234278").unwrap(), 78);
        assert_eq!(solve_line_part_1("818181911112111").unwrap(), 92);
    }

    #[test]
    fn example_lines_part_2() {
        assert_eq!(solve_line_part_2("987654321111111").unwrap(), 987654321111);
        assert_eq!(solve_line_part_2("811111111111119").unwrap(), 811111111119);
        assert_eq!(solve_line_part_2("234234234234278").unwrap(), 434234234278);
        assert_eq!(solve_line_part_2("818181911112111").unwrap(), 888911112111);
    }
}