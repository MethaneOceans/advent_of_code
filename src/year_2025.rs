use crate::{SolverError, YearSolvers};

pub mod day_1;
pub mod day_2;

pub struct Year2025;

impl YearSolvers for Year2025 {
    fn solve(day: i32, input: &str) -> Result<(String, String), SolverError> {
        if !(1..=12).contains(&day) {
            return Err(SolverError::DayOutOfRangeError);
        }

        match day {
            1 => day_1::solve(input),
            2 => day_2::solve(input),
            _ => Err(SolverError::SolverNotImplemented),
        }
    }
}