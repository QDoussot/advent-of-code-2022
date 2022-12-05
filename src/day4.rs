use std::ops::RangeInclusive;

use crate::{
    parse::{
        couple::Couple,
        natural::Natural,
        separator::{LineSep, StrSep},
        seq::Seq,
        ParseExt,
    },
    problem::{ParsingError, Problem, SolvingError},
};

#[derive(Debug)]
pub struct AssignmentsPairs(Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>);

type RangeParser = Couple<Natural<usize>, StrSep<"-">, Natural<usize>>;
type Parser = Seq<Couple<RangeParser, StrSep<",">, RangeParser>, LineSep>;

fn range_from_couple((start, end): (usize, usize)) -> RangeInclusive<usize> {
    RangeInclusive::new(start, end)
}

impl Problem for AssignmentsPairs {
    fn parse(lines: Vec<String>) -> Result<Self, ParsingError> {
        let ap = Parser::parse(lines.join("\n").as_bytes())?
            .into_iter()
            .map(|(left_range, right_range)| (range_from_couple(left_range), range_from_couple(right_range)))
            .collect();
        Ok(Self(ap))
    }

    fn part_one(&self) -> Result<usize, SolvingError> {
        let overlaps = self.0.iter().filter(|(left, right)| {
            (left.contains(&right.start()) && left.contains(&right.end()))
                || (right.contains(&left.start()) && right.contains(&left.end()))
        });
        Ok(overlaps.count())
    }

    fn part_two(&self) -> Result<usize, SolvingError> {
        let overlaps = self.0.iter().filter(|(left, right)| {
            (left.contains(&right.start()) || left.contains(&right.end()))
                || (right.contains(&left.start()) || right.contains(&left.end()))
        });
        Ok(overlaps.count())
    }
}
