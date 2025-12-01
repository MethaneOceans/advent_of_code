// day 1 - part 1 => 1150

fn main() {
    dbg!(test_day_1().is_ok());
    // let result = solve_day_1(include_str!("input_day_1.txt"));
    // println!("{result:?}");
}

fn solve_day_1(input: &str) -> (String, String) {
    let instructions = input.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (direction, ticks) = line.split_at(1);
            let ticks: i32 = ticks.parse().unwrap();
            let direction = match direction {
                "L" => TurnDirection::Left,
                "R" => TurnDirection::Right,
                _ => panic!(),
            };

            (direction, ticks)
        });
    
    let mut position = 50;
    let mut part1_count = 0;
    let mut part2_count = 0;
    for (direction, ticks) in instructions {
        let ticks_rem = ticks % 100;
        let full_turns = (ticks - ticks_rem) / 100;

        part2_count += full_turns;

        match direction {
            TurnDirection::Left => position -= ticks_rem,
            TurnDirection::Right => position += ticks_rem,
        }

        if !(0..100).contains(&position) {
            part2_count += 1;
        }

        position = position.rem_euclid(100);

        if position == 0 {
            part1_count += 1;
        }
    }

    (part1_count.to_string(), part2_count.to_string())
}

#[derive(Debug, Clone, Copy)]
enum TurnDirection {
    Left,
    Right,
}

fn test_day_1() -> Result<(),()> {
    let (result_a, result_b) = solve_day_1("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82");
    if result_a.as_str() == "3" && result_b.as_str() == "6" {
        Ok(())
    } else {
        Err(())
    }
}