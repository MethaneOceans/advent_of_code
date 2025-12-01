fn main() {
    println!("{:?}", solve_day_1(include_str!("input_day_1.txt")));
    test_day_1();
}

fn solve_day_1(input: &str) -> (String, String) {
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

fn test_day_1() {
    let (result_a, result_b) = solve_day_1("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82");
    if result_a.as_str() == "3" {
        println!("test part 1: ok");
    } else {
        println!("test part 1: failed\n  expected 3, was {result_a}");
    }

    if result_b.as_str() == "6" {
        println!("test part 2: ok");
    } else {
        println!("test part 2: failed\n  expected 6, was {result_b}");
    }
}