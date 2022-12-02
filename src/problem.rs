use std::fmt::Debug;
use derive_more::Display;

#[derive(Display,Debug)]
pub enum ParsingError {
    #[display(fmt="")]
    _IncorrectLine {
        description: String,
        number: usize,
        line: String,
    },
    _UnverifiedConstraint(String),
}

#[derive(Display,Debug)]
pub enum SolvingError {
    _InternError,
    _ExpectationUnfulfilled(String),
}

pub trait Problem: Sized {
    fn parse(lines: &[String]) -> Result<Self, ParsingError>;
    fn part_one(&self) -> Result<usize, SolvingError>;
    fn part_two(&self) -> Result<usize, SolvingError>;
}

#[derive(Display,Debug)]
pub enum Error {
    CantOpenInputFile(String),
    _ParsingFailed(ParsingError),
    NoCorrespondingSolver,
    _SolverFailed(SolvingError),
}

pub fn _solve<T: Problem + Debug>(lines: &[String], part: usize) -> Result<usize, Error> {
    let problem = T::parse(lines).map_err(Error::_ParsingFailed)?;
    if part == 0 {
        println!("{:?}", problem);
    }
    if part == 1 {
        problem.part_one().map_err(|_| Error::NoCorrespondingSolver)
    } else {
        problem.part_two().map_err(|_| Error::NoCorrespondingSolver)
    }
}
