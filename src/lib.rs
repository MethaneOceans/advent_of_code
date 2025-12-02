use std::{error::Error, fmt::Display};

pub mod year_2019;
pub mod year_2025;

type DaySolverFunction = dyn Fn(&str) -> Result<(String, String), SolverError>;

pub fn solve(year: i32, day: i32, input: &str) -> Result<(String, String), Box<dyn Error>> {
    if !(2015..=2025).contains(&year) {
        return Err(SolverError::YearOutOfRangeError)?;
    }
    
    match year {
        2019 => Ok(year_2019::Year2019::solve(day, input)?),
        2025 => Ok(year_2025::Year2025::solve(day, input)?),
        _ => Err(Box::new(SolverError::SolverNotImplemented)),
    }
}

pub fn get_solver(year: i32, day: i32) -> Result<Box<DaySolverFunction>, SolverError> {
    if !(2015..=2025).contains(&year) {
        return Err(SolverError::YearOutOfRangeError)?;
    }
    
    match year {
        2019 => Ok(year_2019::Year2019::get_solver(day)?),
        2025 => Ok(year_2025::Year2025::get_solver(day)?),
        _ => Err(SolverError::SolverNotImplemented),
    }
}

trait YearSolvers {
    fn solve(day: i32, input: &str) -> Result<(String, String), SolverError> {
        Self::get_solver(day)?(input)
    }

    fn get_solver(day: i32) -> Result<Box<DaySolverFunction>, SolverError>;
}

#[derive(Debug)]
pub enum SolverError {
    DayOutOfRangeError,
    YearOutOfRangeError,
    SolverNotImplemented,
    BadInput,
}

impl Display for SolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolverError::DayOutOfRangeError => write!(f, "Day out of range!"),
            SolverError::YearOutOfRangeError => write!(f, "Year out of range! Should be in the range of [2015, 2025]"),
            SolverError::SolverNotImplemented => write!(f, "Solver not implemented!"),
            SolverError::BadInput => write!(f, "Input is wrong"),
        }
    }
}

impl Error for SolverError {}