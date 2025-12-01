use crate::SolverError;

pub fn solve_day_1(input: &str) -> Result<(String, String), SolverError> {
    let mut err = None;
    let instructions = input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            // Skip calculations if there's an error.
            if err.is_some() { return 0 }

            let (direction, clicks) = line.split_at(1);
            
            let clicks: i32 = match clicks.parse() {
                Ok(value) => value,
                Err(_) => {
                    err = Some(SolverError::BadInput);
                    return 0;
                },
            };

            match direction {
                "L" => -clicks,
                "R" => clicks,
                _ => {
                    err = Some(SolverError::BadInput);
                    0
                },
            }
        }).collect::<Vec<_>>();

    if let Some(err) = err {
        return Err(err);
    }
    
    let mut position: i32 = 50;
    let mut part1_count = 0;
    let mut part2_count = 0;

    for clicks in instructions {
        let step = clicks.signum();
        
        for _ in 0..(clicks.abs()) {
            position += step;

            if position % 100 == 0 {
                part2_count += 1;
            }
        }

        position = position.rem_euclid(100);

        
        if position == 0 {
            part1_count += 1;
        }
    }

    Ok((part1_count.to_string(), part2_count.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &'static str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn example() {
        let (part_1, part_2) = solve_day_1(EXAMPLE_INPUT).unwrap();
        assert_eq!(&part_1, "3");
        assert_eq!(&part_2, "6");
    }
}