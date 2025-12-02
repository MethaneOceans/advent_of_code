use std::ops::RangeInclusive;

use crate::SolverError;

pub fn solve(input: &str) -> Result<(String, String), SolverError> {
    let ranges = input.trim()
        .split(',')
        .map(|slice| {
            let Some((a, b)) = slice.split_once('-') else {
                return Err(())
            };

            let a = a.trim().parse::<usize>().unwrap();
            let b = b.trim().parse::<usize>().unwrap();

            Ok((a, b))
        });
    
    let mut part_1_count = 0;
    for range in ranges {
        let Ok((a, b)) = range else {
            return Err(SolverError::BadInput);
        };

        get_invalid_ids(a..=b).into_iter()
            .for_each(|x| part_1_count += x as usize);
    }

    Ok((part_1_count.to_string(), "Not yet implemented!".to_string()))
}

fn is_num_valid(num: usize) -> bool {
    let num_str = num.to_string();
    assert!(num_str.is_ascii(), "expected input to be ascii string");

    if num_str.len() % 2 != 0 {
        return true
    }

    let (a, b) = num_str.split_at(num_str.len() / 2);

    a != b
}

fn get_invalid_ids(range: RangeInclusive<usize>) -> Vec<usize> {
    range.into_iter()
        .filter(|x| !is_num_valid(*x))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn example() {
        let (part_1, _part_2) = solve(EXAMPLE_INPUT).unwrap();

        assert_eq!(&part_1, "1227775554");
    }

    #[test]
    fn determine_validity_of_id() {        
        // 11-22 has two invalid IDs, 11 and 22.
        let range = 11..=22;
        assert_eq!(get_invalid_ids(range), &[11, 22]);
        // 95-115 has one invalid ID, 99.
        let range = 95..=115;
        assert_eq!(get_invalid_ids(range), &[99]);
        // 998-1012 has one invalid ID, 1010.
        let range = 998..=1012;
        assert_eq!(get_invalid_ids(range), &[1010]);
        // 1188511880-1188511890 has one invalid ID, 1188511885.
        let range = 1188511880..=1188511890;
        assert_eq!(get_invalid_ids(range), &[1188511885]);
        // 222220-222224 has one invalid ID, 222222.
        let range = 222220..=222224;
        assert_eq!(get_invalid_ids(range), &[222222]);
        // 1698522-1698528 contains no invalid IDs.
        let range = 1698522..=1698528;
        assert_eq!(get_invalid_ids(range), &[]);
        // 446443-446449 has one invalid ID, 446446.
        let range = 446443..=446449;
        assert_eq!(get_invalid_ids(range), &[446446]);
        // 38593856-38593862 has one invalid ID, 38593859.
        let range = 38593856..=38593862;
        assert_eq!(get_invalid_ids(range), &[38593859]);

        // The rest of the ranges contain no invalid IDs.
        let range = 565653..=565659;
        assert_eq!(get_invalid_ids(range), &[]);
        let range = 824824821..=824824827;
        assert_eq!(get_invalid_ids(range), &[]);
        let range = 2121212118..=2121212124;
        assert_eq!(get_invalid_ids(range), &[]);
    }
}