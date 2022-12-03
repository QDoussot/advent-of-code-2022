use itertools::Itertools;

use crate::parse::natural::Natural;
use crate::parse::seq::{EmptyLineSep, LineSep, Seq};
use crate::parse::{Context, ParseExt};
use crate::problem::{self};

#[derive(Debug)]
pub struct Inventories(Vec<Vec<usize>>);
type Parser = Seq<Seq<Natural<usize>, LineSep>, EmptyLineSep>;
impl problem::Problem for Inventories {
    fn parse(lines: Vec<String>) -> Result<Self, problem::ParsingError> {
        let bytes = lines.join("\n");
        let inventories = Parser::parse_with_context(bytes.as_bytes(), Context::default());
        match inventories {
            Ok(inv) => Ok(Self(inv)),
            Err(e) => {
                println!("{:?}", e);
                Ok(Self(vec![]))
            }
        }
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
