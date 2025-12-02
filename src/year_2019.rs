use crate::{SolverError, YearSolvers};

mod day_1;
mod day_2;

pub struct Year2019;

impl YearSolvers for Year2019 {
    fn get_solver(day: i32) -> Result<Box<crate::DaySolverFunction>, SolverError> {
        if !(1..=25).contains(&day) {
            return Err(SolverError::DayOutOfRangeError);
        }

        match day {
            1 => Ok(Box::new(day_1::solve)),
            2 => Ok(Box::new(day_2::solve)),
            _ => Err(SolverError::SolverNotImplemented),
        }
    }
}