use crate::SolverError;

pub fn solve(input: &str) -> Result<(String, String), SolverError> {
    let instructions = input.split(',')
        .map(|str| str.parse::<i64>())
        .collect::<Result<Vec<_>, _>>();
    
    let Ok(mut program) = instructions else {
        return Err(SolverError::BadInput)
    };

    let mut instruction_ptr = 0;

    loop {
        let op = program[instruction_ptr];
        match op {
            1 => {
                let pa = program[instruction_ptr + 1];
                let pb = program[instruction_ptr + 2];
                let a = program[pa as usize];
                let b = program[pb as usize];

                program[instruction_ptr + 3] = a + b;
            }
            2 => {
                let pa = program[instruction_ptr + 1];
                let pb = program[instruction_ptr + 2];
                let a = program[pa as usize];
                let b = program[pb as usize];

                program[instruction_ptr + 3] = a * b;
            }
            9 => break,
            _ => panic!("iptr: {instruction_ptr}, program: {program:#?}"),
        }

        instruction_ptr += 4;
    }

    Ok((format!("{program:?}") ,"".to_string()))
}

enum IntcodeOp {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}

#[cfg(test)]
mod tests {

}