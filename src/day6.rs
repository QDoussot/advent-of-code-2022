use itertools::Itertools;

use crate::problem::{ParsingError, Problem, SolvingError};

#[derive(Debug)]
pub struct Signal(String);

impl Signal {
    fn find_start_marker_pos(&self, required_nbr_of_unique_char: usize) -> Option<usize> {
        self.0
            .chars()
            .collect::<Vec<char>>()
            .as_slice()
            .windows(required_nbr_of_unique_char)
            .find_position(|packet_pot_start| packet_pot_start.iter().all_unique())
            .map(|(p, _)| p)
    }
}

impl Problem for Signal {
    fn parse(mut lines: Vec<String>) -> Result<Self, ParsingError> {
        lines
            .pop()
            .ok_or_else(|| ParsingError::UnverifiedConstraint("No signal at all received (empty file) !".into()))
            .map(Self)
    }

    fn part_one(&self) -> Result<String, SolvingError> {
        match self.find_start_marker_pos(4) {
            Some(ind) => Ok((ind + 4).to_string()),
            None => Err(SolvingError::ExpectationUnfulfilled("No packet start detected".into())),
        }
    }

    fn part_two(&self) -> Result<String, SolvingError> {
        match self.find_start_marker_pos(14) {
            Some(ind) => Ok((ind + 14).to_string()),
            None => Err(SolvingError::ExpectationUnfulfilled("No packet start detected".into())),
        }
    }
}
