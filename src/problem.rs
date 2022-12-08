use derive_more::Display;
use std::fmt::Debug;

#[derive(Display, Debug)]
pub enum ParsingError {
    #[display(fmt = "")]
    #[allow(dead_code)]
    IncorrectLine {
        description: String,
        number: usize,
        line: String,
    },
    Parse(crate::parse::Error),
    UnverifiedConstraint(String),
}
impl From<crate::parse::Error> for ParsingError {
    fn from(e: crate::parse::Error) -> Self {
        Self::Parse(e)
    }
}

#[derive(Display, Debug)]
pub enum SolvingError {
    _InternError,
    ExpectationUnfulfilled(String),
}

pub trait Problem: Sized {
    fn parse(lines: Vec<String>) -> Result<Self, ParsingError>;
    fn part_one(&self) -> Result<String, SolvingError>;
    fn part_two(&self) -> Result<String, SolvingError>;
}

#[derive(Display, Debug)]
pub enum Error {
    CantOpenInputFile(String),
    _ParsingFailed(ParsingError),
    NoCorrespondingSolver,
    SolverFailed(SolvingError),
}

pub fn solve<T: Problem + Debug>(lines: Vec<String>, part: usize) -> Result<String, Error> {
    let problem = T::parse(lines).map_err(Error::_ParsingFailed)?;
    if part == 0 {
        Ok(format!("{:?}", problem))
    } else if part == 1 {
        problem.part_one().map_err(Error::SolverFailed)
    } else {
        problem.part_two().map_err(Error::SolverFailed)
    }
}
