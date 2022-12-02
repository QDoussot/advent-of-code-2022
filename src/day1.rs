use itertools::Itertools;

use crate::problem::{self, ParsingError};

#[derive(Debug)]
pub struct Inventories(Vec<Vec<usize>>);

impl problem::Problem for Inventories {
    fn parse(lines: Vec<String>) -> Result<Self, problem::ParsingError> {
        let mut inventories = vec![];
        let mut curr = vec![];
        for (number, line) in lines.into_iter().enumerate() {
            if line.is_empty() {
                if !curr.is_empty() {
                    inventories.push(curr)
                }
                curr = vec![];
            } else {
                let calories = line.parse::<usize>().map_err(|e| ParsingError::IncorrectLine {
                    description: e.to_string(),
                    number,
                    line,
                })?;
                curr.push(calories)
            }
        }
        if !curr.is_empty() {
            inventories.push(curr);
        }

        Ok(Self(inventories))
    }

    fn part_one(&self) -> Result<usize, problem::SolvingError> {
        let max_inv = self.0.iter().map(|inv| inv.iter().sum::<usize>()).max().unwrap();
        Ok(max_inv)
    }

    fn part_two(&self) -> Result<usize, problem::SolvingError> {
        let max_inv = self
            .0
            .iter()
            .map(|inv| inv.iter().sum::<usize>())
            .sorted()
            .rev()
            .take(3)
            .sum();
        Ok(max_inv)
    }
}
