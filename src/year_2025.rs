use crate::{SolverError, YearSolvers};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

pub struct Year2025;

impl YearSolvers for Year2025 {
    fn get_solver(day: i32) -> Result<Box<crate::DaySolverFunction>, SolverError> {
        if !(1..=12).contains(&day) {
            return Err(SolverError::DayOutOfRangeError);
        }

        match day {
            1 => Ok(Box::new(day_1::solve)),
            2 => Ok(Box::new(day_2::solve)),
            3 => Ok(Box::new(day_3::solve)),
            4 => Ok(Box::new(day_4::solve)),
            5 => Ok(Box::new(day_5::solve)),
            6 => Ok(Box::new(day_6::solve)),
            _ => Err(SolverError::SolverNotImplemented),
        }
    }
}