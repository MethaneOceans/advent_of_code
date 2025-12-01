use std::{env::args, fs::read_to_string};

use advent_of_code::solve;

fn main() {
    const USAGE_MESSAGE: &'static str = include_str!("./usage.txt");

    let args = args().collect::<Vec<_>>();

    if args.len() == 2 && args[1] == "all" {
        solve_all(false);
        return;
    }

    if args.len() == 3 {
        let Ok(year) = args[1].parse::<i32>() else {
            println!("Error: Couldn't parse year");
            println!("{USAGE_MESSAGE}");
            return;
        };
        let Ok(day) = args[2].parse::<i32>() else {
            println!("Error: Couldn't parse day");
            println!("{USAGE_MESSAGE}");
            return;
        };

        let path = format!("./input/{year}/day_{day}.txt");
        let input = read_to_string(&path).expect(&format!("Expected an input file at {path}"));
        
        let result = solve(year, day, &input);
        match result {
            Ok((part_1, part_2)) => {
                println!("Solutions for {year} - day {day}:");
                println!("  part 1: {part_1}");
                println!("  part 2: {part_2}");
            },
            Err(_) => todo!(),
        }

        return;
    }

    if args.len() == 4 {
        let Ok(year) = args[1].parse::<i32>() else {
            println!("Error: Couldn't parse year");
            println!("{USAGE_MESSAGE}");
            return;
        };
        let Ok(day) = args[2].parse::<i32>() else {
            println!("Error: Couldn't parse day");
            println!("{USAGE_MESSAGE}");
            return;
        };
        
        let path = &args[3];
        let input = read_to_string(path).expect(&format!("Expected an input file at {path}"));
        
        let result = solve(year, day, &input);
        match result {
            Ok((part_1, part_2)) => {
                println!("Solutions for {year} - day {day}:");
                println!("  part 1: {part_1}");
                println!("  part 2: {part_2}");
            },
            Err(e) => println!("Error while solving {year} - day {day}: {e}"),
        }

        return;
    }
    
    
    println!("Error: Invalid arguments");
    println!("{USAGE_MESSAGE}");
    return;
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
                        println!("    error: {e}");
                    }
                },
            }
        }
    }
}