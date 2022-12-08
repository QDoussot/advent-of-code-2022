use itertools::Itertools;

use crate::parse::natural::Natural;
use crate::parse::separator::{EmptyLineSep, LineSep};
use crate::parse::seq::Seq;
use crate::parse::ParseExt;
use crate::problem::{self};

#[derive(Debug)]
pub struct Inventories(Vec<Vec<usize>>);
type Parser = Seq<Seq<Natural<usize>, LineSep>, EmptyLineSep>;
impl problem::Problem for Inventories {
    fn parse(lines: Vec<String>) -> Result<Self, problem::ParsingError> {
        let bytes = lines.join("\n");
        let inventories = Parser::parse(bytes.as_bytes());
        inventories.map(Self).map_err(Into::into)
    }

    fn part_one(&self) -> Result<String, problem::SolvingError> {
        let max_inv = self.0.iter().map(|inv| inv.iter().sum::<usize>()).max().unwrap();
        Ok(max_inv.to_string())
    }

    fn part_two(&self) -> Result<String, problem::SolvingError> {
        let max_inv:usize = self
            .0
            .iter()
            .map(|inv| inv.iter().sum::<usize>())
            .sorted()
            .rev()
            .take(3)
            .sum();
        Ok(max_inv.to_string())
    }
}
