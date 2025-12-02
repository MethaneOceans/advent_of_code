use crate::SolverError;

pub fn solve(input: &str) -> Result<(String, String), SolverError> {
    let mut program = parse_input(input)?;
    program[1] = 12;
    program[2] = 2;

    let part_1 = run_intcode_program(program)[0];

    Ok((part_1.to_string(),"".to_string()))
}

fn parse_input(input: &str) -> Result<Vec<i64>, SolverError> {
    let instructions = input.split(',')
        .map(|str| str.parse::<i64>())
        .collect::<Result<Vec<_>, _>>();
    
    let Ok(program) = instructions else {
        return Err(SolverError::BadInput)
    };

    Ok(program)
}

fn run_intcode_program(mut program: Vec<i64>) -> Vec<i64> {
    let mut instruction_ptr = 0;

    loop {
        let op = program[instruction_ptr];
        match op {
            1 => {
                let pa = program[instruction_ptr + 1] as usize;
                let pb = program[instruction_ptr + 2] as usize;
                let pd = program[instruction_ptr + 3] as usize;
                let a = program[pa];
                let b = program[pb];
                program[pd] = a + b;
            }
            2 => {
                let pa = program[instruction_ptr + 1] as usize;
                let pb = program[instruction_ptr + 2] as usize;
                let pd = program[instruction_ptr + 3] as usize;
                let a = program[pa];
                let b = program[pb];
                program[pd] = a * b;

            }
            99 => break,
            _ => panic!("iptr: {instruction_ptr}, program: {program:#?}"),
        }

        instruction_ptr += 4;
    }

    program
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let part_1 = run_intcode_program(parse_input("1,0,0,0,99").unwrap());
        assert_eq!(&part_1, &[2,0,0,0,99]);

        let part_1 = run_intcode_program(parse_input("2,3,0,3,99").unwrap());
        assert_eq!(&part_1, &[2,3,0,6,99]);

        let part_1 = run_intcode_program(parse_input("2,4,4,5,99,0").unwrap());
        assert_eq!(&part_1, &[2,4,4,5,99,9801]);

        let part_1 = run_intcode_program(parse_input("1,1,1,4,99,5,6,0,99").unwrap());
        assert_eq!(&part_1, &[30,1,1,4,2,5,6,0,99]);
    }
}