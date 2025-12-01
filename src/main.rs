use std::fs::read_to_string;

fn main() {
    solve_all(false);
}

fn solve_all(fail_silent: bool) {
    for year in 2015..=2025 {
        println!("Year {year}");
        let days = if year >= 2025 {
            12
        } else {
            25
        };

        for day in 1..=days {
            // println!("  Day {day}");
            let path = format!("./input/{year}/day_{day}.txt");
            let input = read_to_string(path);
            let input = match input {
                Ok(v) => v,
                Err(_) => {
                    if !fail_silent {
                        println!("  Day {day:>02}: No input found");
                    }
                    continue;
                },
            };

            let result = advent_of_code::solve(year, day, &input);
            match result {
                Ok((part_1, part_2)) => {
                    println!("  Day {day:>02}");
                    println!("    part 1: {part_1}");
                    println!("    part 2: {part_2}");
                },
                Err(e) => {
                    if !fail_silent {
                        println!("  Day {day:>02}");
                        println!("error: {e}")
                    }
                },
            }
        }
    }
}