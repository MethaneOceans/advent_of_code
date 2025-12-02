use std::{error::Error, fmt::Display};

pub mod year_2019;
pub mod year_2025;

pub fn solve(year: i32, day: i32, input: &str) -> Result<(String, String), Box<dyn Error>> {
    assert!((2015..=2025).contains(&year), "Expected year to be in the range of [2015, 2025]! Was {year}");
    
    match year {
        2019 => Ok(year_2019::Year2019::solve(day, input)?),
        2025 => Ok(year_2025::Year2025::solve(day, input)?),
        _ => Err(Box::new(SolverError::SolverNotImplemented)),
    }
}

trait YearSolvers {
    fn solve(day: i32, input: &str) -> Result<(String, String), SolverError>;
}

#[derive(Debug)]
pub enum SolverError {
    DayOutOfRangeError,
    SolverNotImplemented,
    BadInput,
}

impl Display for SolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolverError::DayOutOfRangeError => write!(f, "Day out of range!"),
            SolverError::SolverNotImplemented => write!(f, "Solver not implemented!"),
            SolverError::BadInput => write!(f, "Input is wrong"),
        }
    }
}

impl Error for SolverError {}