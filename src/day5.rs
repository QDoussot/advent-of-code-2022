use std::str::FromStr;

use itertools::Itertools;

use crate::problem::SolvingError::ExpectationUnfulfilled;
use crate::{
    parse::{
        capture::Capture,
        couple::Couple,
        natural::Natural,
        separator::{EmptyLineSep, LineSep, StrSep},
        seq::Seq,
        table::Table,
        ParseExt,
    },
    problem::{ParsingError, Problem, SolvingError},
};

#[derive(Debug)]
struct Move {
    n: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum StackSymbol {
    Empty,
    Crate(char),
    ColumnIndicator(usize),
}

impl StackSymbol {
    fn vertical_group_order(&self) -> usize {
        //
        match self {
            StackSymbol::Empty => 0,
            StackSymbol::Crate(_) => 1,
            StackSymbol::ColumnIndicator(_) => 2,
        }
    }
    fn as_crate_symbol(self) -> Option<char> {
        match self {
            StackSymbol::Empty => None,
            StackSymbol::Crate(c) => Some(c),
            StackSymbol::ColumnIndicator(_) => None,
        }
    }
}

fn is_valid_stack(stack: &[StackSymbol]) -> bool {
    let pattern: Vec<usize> = stack.iter().map(StackSymbol::vertical_group_order).dedup().collect();
    vec![1, 2] == pattern || vec![0, 1, 2] == pattern
}

impl FromStr for StackSymbol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "   " {
            Ok(StackSymbol::Empty)
        } else if s.starts_with("[") && s.ends_with("]") {
            Ok(StackSymbol::Crate(s.chars().nth(1).unwrap()))
        } else if s.chars().nth(1).unwrap().is_ascii_digit() {
            Ok(StackSymbol::ColumnIndicator(usize::from_str(&s[1..=1]).unwrap()))
        } else {
            Err(format!("'{}' is not a valid stack elements", s))
        }
    }
}

#[derive(Debug)]
pub struct RearrangementProcedure {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

impl Problem for RearrangementProcedure {
    fn parse(lines: Vec<String>) -> Result<Self, ParsingError> {
        type StackParser = Seq<Table<3, StrSep<" ">, Natural<StackSymbol>>, LineSep>;
        type ProcedureParser = Seq<Capture<"move % from % to %", 3, Natural<usize>>, LineSep>;
        let res = Couple::<StackParser, EmptyLineSep, ProcedureParser>::parse(lines.join("\n").as_bytes())?;
        let stacks_rows = res.0;

        if !stacks_rows.iter().map(|hor| hor.len()).all_equal() {
            return Err(ParsingError::UnverifiedConstraint(
                "The drawing has lines with different number of stack".into(),
            ));
        }

        let stacks: Vec<Vec<StackSymbol>> = (0..stacks_rows[0].len()).map(|_| Vec::new()).collect();
        let stacks = stacks_rows.into_iter().fold(stacks, |mut acc, row| {
            row.into_iter().enumerate().for_each(|(col, symbol)| {
                acc[col].push(symbol);
            });
            acc
        });

        if !stacks.iter().map(Vec::as_slice).all(is_valid_stack) {
            return Err(ParsingError::UnverifiedConstraint(
                "Feels like there is a crate floating in the air, a stack on the drawing is incorrect".into(),
            ));
        }

        let stacks: Vec<_> = stacks
            .into_iter()
            .map(|stack| {
                stack
                    .into_iter()
                    .filter_map(StackSymbol::as_crate_symbol)
                    .rev()
                    .collect()
            })
            .collect();

        let moves: Vec<_> = res.1.into_iter().map(|[n, from, to]| Move { n, from, to }).collect();

        if let Some(_) = moves.iter().find(|order| !(1..=stacks.len()).contains(&order.from)) {
            return Err(ParsingError::UnverifiedConstraint(
                "Oh noooo! you grabbed an elf which was standing near the crates stacks".into(),
            ));
        }

        if let Some(_) = moves.iter().find(|order| !(1..=stacks.len()).contains(&order.to)) {
            return Err(ParsingError::UnverifiedConstraint(
                "Oh noooo! You throwned a crate away of the ship !".into(),
            ));
        }

        Ok(Self { stacks, moves })
    }

    fn part_one(&self) -> Result<String, SolvingError> {
        let mut stacks = self.stacks.clone();
        for m in self.moves.iter() {
            for _ in 1..=m.n {
                match stacks[m.from - 1].pop() {
                    Some(cra) => stacks[m.to - 1].push(cra),
                    None => {
                        return Err(ExpectationUnfulfilled(
                            "You are trying to pull a crate from an empty stack budy !".into(),
                        ))
                    }
                }
            }
        }
        Ok(stacks.iter().map(|stack| stack.last()).flatten().join(""))
    }

    fn part_two(&self) -> Result<String, SolvingError> {
        let mut stacks = self.stacks.clone();
        for m in self.moves.iter() {
            let remaining_crates = stacks[m.from - 1].len() as isize - m.n as isize;
            if remaining_crates >= 0 {
                let mut moved = stacks[m.from - 1].split_off(remaining_crates as usize);
                stacks[m.to - 1].append(&mut moved);
            } else {
                return Err(ExpectationUnfulfilled(
                    "You are trying to pull a crate from an empty stack budy !".into(),
                ));
            }
        }
        Ok(stacks.iter().map(|stack| stack.last()).flatten().join(""))
    }
}
