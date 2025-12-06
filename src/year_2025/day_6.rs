use crate::SolverError;

pub fn solve(input: &str) -> Result<(String, String), SolverError> {
    let part_1 = solve_part_1(input)?.to_string();
    let part_2 = solve_part_2(input)?.to_string();

    Ok((part_1, part_2))
}

fn solve_part_1(input: &str) -> Result<usize, SolverError> {
    let mut input = input.lines()
        .map(|line| line.trim().split(' ').filter(|l| !l.is_empty()).collect::<Vec<_>>())
        .inspect(|l| println!("{l:?}"))
        .collect::<Vec<_>>();
    
    let operators = input.pop()
        .unwrap()
        .into_iter()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<_>>();

    let numbers = input.into_iter()
        .map(|i| {
            i.into_iter()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total = 0;
    for (col, op) in operators.into_iter().enumerate() {
        let mut result = numbers[0][col];
        println!("start: {result}");

        for i in 1..numbers.len() {
            let num = numbers[i][col];
            match op {
                '+' => result += num,
                '*' => result *= num,
                _ => panic!()
            }
            println!("{result}");
        }

        println!("result: {result}");

        total += result;
    }

    Ok(total)
}

fn solve_part_2(input: &str) -> Result<usize, SolverError> {
    let mut lines = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let operators = lines.pop().unwrap();

    let mut index = 0;
    let mut total = 0;

    while index < operators.len() {
        let operator = operators[index];

        let mut nums_len = 1;
        while index + nums_len < operators.len() && !['*','+'].contains(&operators[index + nums_len]) {
            nums_len += 1;
        }
        
        if index + nums_len < operators.len() {
            nums_len -= 1;
        }

        // Collect the characters from each number into new strings.
        let mut nums = vec![String::default(); nums_len];

        for i in 0..(nums_len) {
            for s in lines.iter() {
                let n = &mut nums[i];
                n.push(s[index + i]);
            }
        }

        // Create an iterator that trims and then parses the strings
        let nums = nums.into_iter()
            .map(|num| num.trim().parse::<usize>().unwrap());

        // Apply operation on the numbers
        let result = match operator {
            '*' => nums.reduce(|acc, ele| acc * ele),
            '+' => nums.reduce(|acc, ele| acc + ele),
            _ => panic!(),
        }.unwrap();

        total += result;
        index += nums_len + 1;
    }

    Ok(total)
}