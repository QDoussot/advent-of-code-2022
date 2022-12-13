use derive_more::{Add, From, Sub};
use itertools::Itertools;
use strum_macros::EnumString;

use crate::prelude::*;

#[derive(EnumString, Debug, Copy, Clone)]
enum Direction {
    U,
    D,
    R,
    L,
}

impl Into<Coord> for Direction {
    fn into(self) -> Coord {
        match self {
            Direction::U => (0, 1),
            Direction::D => (0, -1),
            Direction::R => (1, 0),
            Direction::L => (-1, 0),
        }
        .into()
    }
}

#[derive(From, Debug, Clone, Copy)]
pub struct Movement(Direction, usize);

impl IntoIterator for Movement {
    type Item = Coord;

    type IntoIter = std::iter::RepeatN<Coord>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::repeat_n(self.0.into(), self.1)
    }
}

#[derive(Debug)]
pub struct Movements(Vec<Movement>);

#[derive(Add, Sub, From, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Coord {
    x: isize,
    y: isize,
}

fn maxxed(x: isize) -> isize {
    match x {
        x if x <= -1 => -1,
        x if x >= 1 => 1,
        x => x,
    }
}

impl Coord {
    fn is_connexe_with(&self, coord: &Coord) -> bool {
        let delta = *coord - *self;
        isize::abs(delta.x) < 2 && isize::abs(delta.y) < 2
    }

    fn into_maxxed(self) -> Self {
        (maxxed(self.x), maxxed(self.y)).into()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Rope {
    knots: Vec<Coord>,
}

impl Rope {
    fn move_toward(mut self, mov_coord: Coord) -> Self {
        self.knots[0] = self.knots[0] + mov_coord;
        for i in 1..self.knots.len() {
            let heading_knot = self.knots[i - 1];
            let curr = self.knots.get_mut(i).unwrap();
            if !curr.is_connexe_with(&heading_knot) {
                let delta = (heading_knot - *curr).into_maxxed();
                *curr = *curr + delta
            }
        }

        self
    }
}

impl Problem for Movements {
    fn parse(lines: Vec<String>) -> Result<Self, ParsingError> {
        type Parser = Seq<Couple<Natural<Direction>, SpaceSep, Natural<usize>>, LineSep>;
        let movs = Parser::parse(lines.join("\n").as_bytes())?
            .into_iter()
            .map(Movement::from)
            .collect();
        Ok(Self(movs))
    }

    fn part_one(&self) -> Result<String, SolvingError> {
        let mut rope = Rope {
            knots: std::iter::repeat_n((0, 0).into(), 2).collect(),
        };
        let rope_states = self.0.clone().into_iter().flat_map(IntoIterator::into_iter).map(|mov| {
            rope = rope.clone().move_toward(mov);
            rope.clone()
        });
        let res = rope_states
            .map(|rope| rope.knots.last().unwrap().clone())
            .unique()
            .count();
        Ok(res.to_string())
    }

    fn part_two(&self) -> Result<String, SolvingError> {
        let mut rope = Rope {
            knots: std::iter::repeat_n((0, 0).into(), 10).collect(),
        };
        let rope_states = self.0.clone().into_iter().flat_map(IntoIterator::into_iter).map(|mov| {
            rope = rope.clone().move_toward(mov);
            rope.clone()
        });
        let res = rope_states
            .map(|rope| rope.knots.last().unwrap().clone())
            .unique()
            .count();
        Ok(res.to_string())
    }
}
