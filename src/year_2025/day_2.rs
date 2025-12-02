use std::{error::Error, ops::RangeInclusive, time::Instant};
use crate::SolverError;

macro_rules! timed {
    ($e:expr) => {{
        let start = Instant::now();
        let value = $e;
        let end = Instant::now();
        println!("[{}:{}] Took {:?} to evaluate {}", file!(), line!(), (end - start), stringify!($e));
        value
    }};
    ($e:expr, $s:literal) => {{
        let start = Instant::now();
        let value = $e;
        let end = Instant::now();
        println!("[{}:{}] {} took {:?}", file!(), line!(), $s, (end - start));
        value
    }}
}

pub fn solve(input: &str) -> Result<(String, String), SolverError> {
    let ranges = input.trim()
        .split(',')
        .map(|slice| {
            let Some((a, b)) = slice.split_once('-') else {
                return Err(SolverError::BadInput)?
            };

            let a = a.trim().parse::<usize>()?;
            let b = b.trim().parse::<usize>()?;

            Ok((a, b))
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>();

    let Ok(ranges) = ranges else {
        return Err(SolverError::BadInput);
    };

    let part_1 = timed!(solve_part_1(&ranges), "solve_part_1")?;
    let part_2 = timed!(solve_part_2(&ranges), "solve_part_2")?;

    Ok((part_1, part_2))
}

fn solve_part_1(ranges: &[(usize, usize)]) -> Result<String, SolverError> {
    let mut count = 0;
    for (a, b) in ranges {
        get_invalid_ids_part_1(*a..=*b).into_iter()
            .for_each(|x| count += x as usize);
    }

    Ok(count.to_string())
}

fn solve_part_2(ranges: &[(usize, usize)]) -> Result<String, SolverError> {
    let mut count = 0;
    for (a, b) in ranges {
        get_invalid_ids_part_2(*a..=*b).into_iter()
            .for_each(|x| count += x as usize);
    }

    Ok(count.to_string())
}

fn is_num_valid_part_1(num: usize) -> bool {
    let num_str = num.to_string();
    assert!(num_str.is_ascii(), "expected input to be ascii string");

    if num_str.len() % 2 != 0 {
        return true
    }

    let (a, b) = num_str.split_at(num_str.len() / 2);

    a != b
}

fn get_invalid_ids_part_1(range: RangeInclusive<usize>) -> Vec<usize> {
    range.into_iter()
        .filter(|x| !is_num_valid_part_1(*x))
        .collect()
}

fn is_num_valid_part_2(num: usize) -> bool {
    let num_str = num.to_string();

    // Can't have repeated digits if there's only one digit.
    if num_str.chars().count() == 1 {
        return true;
    }
    
    let num_str = num_str.to_string().chars().collect::<Vec<_>>();
    let len = num_str.len();

    // If num cannot be split into equal parts it cannot consist of a repeated pattern
    let len_iter = (1..len).into_iter()
        .filter(|x| len % x == 0);

    for group_len in len_iter {
        let mut groups = num_str.chunks_exact(group_len)
            .map(|s| s);
        
        let first = groups.next().unwrap();
        if groups.all(|ele| ele == first) {
            return false
        }
    }

    true
}

fn get_invalid_ids_part_2(range: RangeInclusive<usize>) -> Vec<usize> {
    timed!(range.into_iter()
        .filter(|x| !is_num_valid_part_2(*x))
        .collect(), "get_invalid_ids_part_2")
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn example() {
        let (part_1, part_2) = solve(EXAMPLE_INPUT).unwrap();

        assert_eq!(&part_1, "1227775554");
        assert_eq!(&part_2, "4174379265");
    }

    #[test]
    fn determine_validity_of_id() {        
        // 11-22 has two invalid IDs, 11 and 22.
        let range = 11..=22;
        assert_eq!(get_invalid_ids_part_1(range), &[11, 22]);
        // 95-115 has one invalid ID, 99.
        let range = 95..=115;
        assert_eq!(get_invalid_ids_part_1(range), &[99]);
        // 998-1012 has one invalid ID, 1010.
        let range = 998..=1012;
        assert_eq!(get_invalid_ids_part_1(range), &[1010]);
        // 1188511880-1188511890 has one invalid ID, 1188511885.
        let range = 1188511880..=1188511890;
        assert_eq!(get_invalid_ids_part_1(range), &[1188511885]);
        // 222220-222224 has one invalid ID, 222222.
        let range = 222220..=222224;
        assert_eq!(get_invalid_ids_part_1(range), &[222222]);
        // 1698522-1698528 contains no invalid IDs.
        let range = 1698522..=1698528;
        assert_eq!(get_invalid_ids_part_1(range), &[]);
        // 446443-446449 has one invalid ID, 446446.
        let range = 446443..=446449;
        assert_eq!(get_invalid_ids_part_1(range), &[446446]);
        // 38593856-38593862 has one invalid ID, 38593859.
        let range = 38593856..=38593862;
        assert_eq!(get_invalid_ids_part_1(range), &[38593859]);

        // The rest of the ranges contain no invalid IDs.
        let range = 565653..=565659;
        assert_eq!(get_invalid_ids_part_1(range), &[]);
        let range = 824824821..=824824827;
        assert_eq!(get_invalid_ids_part_1(range), &[]);
        let range = 2121212118..=2121212124;
        assert_eq!(get_invalid_ids_part_1(range), &[]);
    }
}