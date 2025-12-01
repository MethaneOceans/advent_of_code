pub fn solve_day_1(input: &str) -> (String, String) {
    let instructions = input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (direction, clicks) = line.split_at(1);
            let clicks: i32 = clicks.parse().unwrap();
            match direction {
                "L" => -clicks,
                "R" => clicks,
                _ => panic!(),
            }
        });
    
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

    (part1_count.to_string(), part2_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &'static str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn example() {
        let (part_1, part_2) = solve_day_1(EXAMPLE_INPUT);
        assert_eq!(&part_1, "3");
        assert_eq!(&part_2, "6");
    }
}