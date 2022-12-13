use std::collections::HashSet;

use itertools::Itertools;

use crate::problem::{self, ParsingError, Problem, SolvingError};

#[derive(Debug)]
pub struct RuckSacks(Vec<RuckSack>);

#[derive(Debug)]
struct RuckSack(Vec<char>, Vec<char>);

impl RuckSack {
    fn common_item(&self) -> Result<char, SolvingError> {
        let (l, r) = (
            HashSet::<&char>::from_iter(self.0.iter()),
            HashSet::<&char>::from_iter(self.1.iter()),
        );
        let commons = l.intersection(&r).map(|c| **c).collect::<Vec<_>>();
        if let &[common] = commons.as_slice() {
            Ok(common)
        } else {
            let msg = format!("Not exactly one common item : '{:?}'", commons);
            Err(SolvingError::ExpectationUnfulfilled(msg))
        }
    }

    fn item_set(&self) -> HashSet<char> {
        self.0.clone().into_iter().chain(self.1.clone().into_iter()).collect()
    }
}

#[derive(Debug)]
struct Group<'s>([&'s RuckSack; 3]);

impl<'s> Group<'s> {
    fn find_badge(&self) -> Result<char, SolvingError> {
        let occurences = self
            .0
            .iter()
            .map(|sack| sack.item_set().into_iter().collect::<Vec<char>>())
            .flatten()
            .counts();
        let badge = occurences
            .into_iter()
            .filter_map(|(v, c)| (c == 3).then(|| v))
            .collect::<Vec<_>>();
        match badge.as_slice() {
            &[badge] => Ok(badge),
            _ => Err(SolvingError::ExpectationUnfulfilled(format!(
                "Did not find exactly one possible badge: '{:?}'",
                badge
            ))),
        }
    }
}

impl<'s> TryFrom<Vec<&'s RuckSack>> for Group<'s> {
    type Error = SolvingError;

    fn try_from(value: Vec<&'s RuckSack>) -> Result<Self, Self::Error> {
        let group = value.try_into().map_err(|e: Vec<_>| {
            SolvingError::ExpectationUnfulfilled(format!("Expected group of 3 and found: '{}'", e.len()))
        })?;
        Ok(Self(group))
    }
}

fn priority(c: char) -> Result<usize, SolvingError> {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .find(c)
        .ok_or(SolvingError::ExpectationUnfulfilled("Not an alphabetic".into()))
        .map(|v| v + 1)
}

impl Problem for RuckSacks {
    fn parse(lines: Vec<String>) -> Result<Self, ParsingError> {
        let ruck_sacks = lines
            .into_iter()
            .map(|mut line| {
                if line.len() % 2 == 0 {
                    let snd = line.split_off(line.len() / 2);
                    Ok(RuckSack(line.chars().collect(), snd.chars().collect()))
                } else {
                    Err(ParsingError::UnverifiedConstraint(
                        "line '{}' has not even number of items".to_string(),
                    ))
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(ruck_sacks))
    }

    fn part_one(&self) -> Result<String, problem::SolvingError> {
        let res: usize = self
            .0
            .iter()
            .map(|rs| rs.common_item().and_then(priority))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .sum();
        Ok(res.to_string())
    }

    fn part_two(&self) -> Result<String, problem::SolvingError> {
        let res: usize = self
            .0
            .iter()
            .chunks(3)
            .into_iter()
            .map(|group| {
                let group = Group::try_from(group.collect::<Vec<_>>())?;
                let badge = Group::find_badge(&group);
                match &badge {
                    Ok(_) => (),
                    Err(g) => println!("Group {:?} has err : '{:?}'", group, g),
                }
                priority(badge?)
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .sum();
        Ok(res.to_string())
    }
}
