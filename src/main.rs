use std::{env::args, fs::read_to_string};

use advent_of_code::solve;

fn main() {
    const USAGE_MESSAGE: &'static str = include_str!("./usage.txt");

    let args = args().collect::<Vec<_>>();

    if args.len() == 2 {
        match args[1].as_str() {
            "all" => solve_all(false),
            "solvable" => graph_solvable(),
            _ => {
                println!("Error: Invalid arguments");
                println!("{USAGE_MESSAGE}");
            }
        }
        return;
    }

    let path = if args.len() == 4 {
        Some(args[3].to_string())
    } else {
        None
    };

    if args.len() == 3 || args.len() == 4 {
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

        let path = path.unwrap_or(format!("./input/{year}/day_{day}.txt"));
        // let path = format!("./input/{year}/day_{day}.txt");
        
        let input = read_to_string(&path).expect(&format!("Expected an input file at {path}"));
        
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

fn graph_solvable() {
    for year in 2015..=2025 {
        print!("{year} [");

        let days = if year >= 2025 { 12 } else { 25 };

        let iter= (1..=days).into_iter()
            .map(|day| -> Result<(), Box<dyn std::error::Error>> {
                let path = format!("./input/{year}/day_{day}.txt");
                let _input = std::fs::exists(path)?;
                match advent_of_code::get_solver(year, day) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e)?,
                }
            });
            

        for value in iter {
            match value {
                Ok(_) => print!("\x1b[32m."),
                Err(_) => print!("\x1b[31mx"),
            }
        }

        print!("\x1b[0m]\n");
    }
}