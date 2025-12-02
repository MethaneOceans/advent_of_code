use crate::{SolverError, YearSolvers};

pub mod day_1;

pub struct Year2019;

impl YearSolvers for Year2019 {
    fn solve(day: i32, input: &str) -> Result<(String, String), SolverError> {
        if !(1..=25).contains(&day) {
            return Err(SolverError::DayOutOfRangeError);
        }

        match day {
            1 => day_1::solve(input),
            _ => Err(SolverError::SolverNotImplemented),
        }
    }
}