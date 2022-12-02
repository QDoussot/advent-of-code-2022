use derive_more::Display;
use std::fmt::Debug;

#[derive(Display, Debug)]
pub enum ParsingError {
    #[display(fmt = "")]
    IncorrectLine {
        description: String,
        number: usize,
        line: String,
    },
    _UnverifiedConstraint(String),
}

#[derive(Display, Debug)]
pub enum SolvingError {
    _InternError,
    _ExpectationUnfulfilled(String),
}

pub trait Problem: Sized {
    fn parse(lines: Vec<String>) -> Result<Self, ParsingError>;
    fn part_one(&self) -> Result<usize, SolvingError>;
    fn part_two(&self) -> Result<usize, SolvingError>;
}

#[derive(Display, Debug)]
pub enum Error {
    CantOpenInputFile(String),
    _ParsingFailed(ParsingError),
    NoCorrespondingSolver,
    _SolverFailed(SolvingError),
}

pub fn solve<T: Problem + Debug>(lines: Vec<String>, part: usize) -> Result<usize, Error> {
    let problem = T::parse(lines).map_err(Error::_ParsingFailed)?;
    if part == 0 {
        println!("{:?}", problem);
        return Ok(0);
    }
    if part == 1 {
        problem.part_one().map_err(|_| Error::NoCorrespondingSolver)
    } else {
        problem.part_two().map_err(|_| Error::NoCorrespondingSolver)
    }
}
