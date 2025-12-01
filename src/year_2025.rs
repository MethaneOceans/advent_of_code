use crate::{SolverError, YearSolvers};

pub mod day_1;

pub struct Year2025;

impl YearSolvers for Year2025 {
    fn solve(day: i32, input: &str) -> Result<(String, String), SolverError> {
        if !(1..=12).contains(&day) {
            return Err(SolverError::DayOutOfRangeError);
        }

        match day {
            1 => day_1::solve_day_1(input),
            _ => Err(SolverError::SolverNotImplemented),
        }
    }
}