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
    // Select the leftmost highest digit from line[(last_digit + 1)..(len - batteries_left)]
    let digits = parse_line(line).collect::<Result<Vec<_>,SolverError>>()?;
    let mut batteries_left = 12;
    let mut result_digits = vec![];
    let mut index_left = 0;

    while batteries_left > 0 {
        dbg!(batteries_left);
        dbg!(index_left);
        let slice =  &digits[index_left..=(digits.len() - batteries_left)];
        println!("{slice:?}");

        let (i, d) = slice.iter()
            .enumerate()
            .rev()
            .max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();

        dbg!(i);
        dbg!(d);

        result_digits.push(d);
        index_left += i + 1;
        
        batteries_left -= 1;

        println!();
    }


    println!("result_digits: {result_digits:?}");

    let result = result_digits.into_iter()
        .map(|d| *d as u64)
        .reduce(|acc, ele| acc * 10 + ele)
        .unwrap();

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