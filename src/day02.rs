use std::str::FromStr;

use crate::parse::couple::Couple;
use crate::parse::natural::Natural;
use crate::parse::separator::{LineSep, SpaceSep};
use crate::parse::seq::Seq;
use crate::parse::ParseExt;
use crate::problem::{ParsingError, Problem};

#[derive(Debug)]
enum Concept {
    Rock,
    Papr,
    Scis,
}

impl AsRef<usize> for Concept {
    fn as_ref(&self) -> &usize {
        match self {
            Concept::Rock => &0,
            Concept::Papr => &1,
            Concept::Scis => &2,
        }
    }
}

impl Concept {
    fn score(&self) -> usize {
        match self {
            Concept::Rock => 1,
            Concept::Papr => 2,
            Concept::Scis => 3,
        }
    }
    fn fight(&self, other: &Concept) -> MatchResult {
        let (win, drw, lst) = (MatchResult::Win, MatchResult::Drw, MatchResult::Lst);
        #[rustfmt::skip]
        let result = [
            [drw, lst, win],
            [win, drw, lst],
            [lst, win, drw]
        ];
        result[*self.as_ref()][*other.as_ref()]
    }

    fn has_outcome(&self, outcome: MatchResult) -> Self {
        match (self, outcome) {
            (Concept::Rock, MatchResult::Win) => Concept::Papr,
            (Concept::Rock, MatchResult::Drw) => Concept::Rock,
            (Concept::Rock, MatchResult::Lst) => Concept::Scis,
            (Concept::Papr, MatchResult::Win) => Concept::Scis,
            (Concept::Papr, MatchResult::Drw) => Concept::Papr,
            (Concept::Papr, MatchResult::Lst) => Concept::Rock,
            (Concept::Scis, MatchResult::Win) => Concept::Rock,
            (Concept::Scis, MatchResult::Drw) => Concept::Scis,
            (Concept::Scis, MatchResult::Lst) => Concept::Papr,
        }
    }
}

#[derive(Debug)]
struct Play(Concept);

impl FromStr for Play {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Play(Concept::Rock)),
            "B" => Ok(Play(Concept::Papr)),
            "C" => Ok(Play(Concept::Scis)),
            _ => Err(format!("'{}' is not a valid play", s)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Indicator {
    X,
    Y,
    Z,
}

impl FromStr for Indicator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Indicator::X),
            "Y" => Ok(Indicator::Y),
            "Z" => Ok(Indicator::Z),
            _ => Err(format!("'{}' is not a valid indicator", s)),
        }
    }
}

#[derive(Copy, Clone)]
enum MatchResult {
    Win,
    Drw,
    Lst,
}

impl MatchResult {
    fn score(&self) -> usize {
        match self {
            MatchResult::Win => 6,
            MatchResult::Drw => 3,
            MatchResult::Lst => 0,
        }
    }
}

#[derive(Debug)]
pub struct Guide(Vec<(Play, Indicator)>);

fn as_play(indicator: Indicator) -> Play {
    match indicator {
        Indicator::X => Play(Concept::Rock),
        Indicator::Y => Play(Concept::Papr),
        Indicator::Z => Play(Concept::Scis),
    }
}

fn as_outcome(indicator: Indicator) -> MatchResult {
    match indicator {
        Indicator::X => MatchResult::Lst,
        Indicator::Y => MatchResult::Drw,
        Indicator::Z => MatchResult::Win,
    }
}

impl Problem for Guide {
    fn parse(lines: Vec<String>) -> Result<Self, ParsingError> {
        let guide =
            Seq::<Couple<Natural<Play>, SpaceSep, Natural<Indicator>>, LineSep>::parse(lines.join("\n").as_bytes())?;
        Ok(Self(guide))
    }

    fn part_one(&self) -> Result<String, crate::problem::SolvingError> {
        let score: usize = self
            .0
            .iter()
            .map(|(o, m)| (o, as_play(*m)))
            .map(|(o, m)| m.0.fight(&o.0).score() + m.0.score())
            .sum();
        Ok(score.to_string())
    }

    fn part_two(&self) -> Result<String, crate::problem::SolvingError> {
        let score: usize = self
            .0
            .iter()
            .map(|(o, m)| (o, as_outcome(*m)))
            .map(|(o, m)| o.0.has_outcome(m).score() + m.score())
            .sum();
        Ok(score.to_string())
    }
    //
}
