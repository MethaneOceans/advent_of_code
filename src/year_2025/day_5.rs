use std::ops::RangeInclusive;

use crate::SolverError;

pub fn solve(input: &str) -> Result<(String, String), SolverError> {
    let (ranges, food_items) = parse_input(input)?;

    let part_1 = solve_part_1(&ranges, &food_items)?.to_string();
    let part_2 = solve_part_2(&ranges)?.to_string();

    Ok((part_1, part_2))
}

fn parse_input(input: &str) -> Result<(Vec<RangeInclusive<usize>>, Vec<usize>), SolverError> {
    let mut lines = input.lines()
        .map(|line| line.trim());

    let mut ranges = vec![];
    loop {
        let Some(line) = lines.next() else {
            // If the input stops before the line break something is wrong.
            return Err(SolverError::BadInput);
        };

        // Stop processing the lines as ranges
        if line.is_empty() {
            break;
        }

        let mut split = line.split('-');
        let lo: usize = split.next().unwrap().parse().unwrap();
        let hi: usize = split.next().unwrap().parse().unwrap();
        let range = lo..=hi;

        ranges.push(range);

    }

    let mut food_items = vec![];
    while let Some(line) = lines.next() {
        food_items.push(line.parse::<usize>().unwrap());
    }

    Ok((ranges, food_items))
}

fn solve_part_1(ranges: &[RangeInclusive<usize>], food_items: &[usize]) -> Result<usize, SolverError> {
    let count = food_items.iter()
        .filter(|item| ranges.iter().any(|range| range.contains(item)))
        .count();
    Ok(count)
}

fn solve_part_2(ranges: &[RangeInclusive<usize>]) -> Result<usize, SolverError> {
    let mut ranges = ranges.iter()
        .cloned()
        .collect::<Vec<_>>();
    
    let mut iterations = 0;
    // Sort the ranges so they can be merged
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut i = 0;
    iterations += 1;

    loop {
        if i >= ranges.len() - 1 {
            break;
        }

        let a = &ranges[i];
        let b = &ranges[i+1];
        
        println!("i: {i}");
        println!("a: {a:?}");
        println!("b: {b:?}");
        
        if a.contains(b.start()) || a.contains(b.end()) {
            println!("ranges overlap, merging");
            let start = *a.start().min(b.start());
            let end = *a.end().max(b.end());
            println!("new range: {:?}", start..=end);
            ranges.remove(i);
            ranges[i] = start..=end;
        } else {
            i += 1;
        }
        println!();
    }

    println!("{ranges:#?}");
    println!("merge iterations: {iterations}");

    let count: usize = ranges.into_iter()
        .map(|range| range.count())
        .sum();

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let (ranges, food_items) = 
            parse_input(include_str!("../../input/examples/2025_day_5.txt")).unwrap();
        assert_eq!(solve_part_1(&ranges, &food_items).unwrap(), 3);
    }

    #[test]
    fn example_part_2() {
        let (ranges, _) = 
            parse_input(include_str!("../../input/examples/2025_day_5.txt")).unwrap();
        assert_eq!(solve_part_2(&ranges).unwrap(), 14);
    }
}