use crate::{
    parse::{
        natural::Natural,
        separator::{Empty, LineSep, StrSep},
        seq::Seq,
    },
    problem::{ParsingError, Problem, SolvingError},
};
use itertools::Itertools;

#[derive(Debug)]
pub struct Forest(Vec<Vec<usize>>);

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Coord { x: value.0, y: value.1 }
    }
}

enum Order {
    Increasing,
    Decreasing,
}
enum Segment {
    Vertical {
        x: usize,
        y_start: usize,
        y_end: usize,
        order: Order,
    },
    Horizontal {
        y: usize,
        x_start: usize,
        x_end: usize,
        order: Order,
    },
}

fn vertical(x: usize, y_start: usize, y_end: usize, order: Order) -> Segment {
    Segment::Vertical {
        x,
        y_start,
        y_end,
        order,
    }
}

fn horizontal(y: usize, x_start: usize, x_end: usize, order: Order) -> Segment {
    Segment::Horizontal {
        y,
        x_start,
        x_end,
        order,
    }
}

impl IntoIterator for Segment {
    type Item = Coord;

    type IntoIter = Box<dyn Iterator<Item = Coord>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Segment::Vertical {
                x,
                y_start,
                y_end,
                order: Order::Increasing,
            } => Box::new((y_start..y_end).map(move |y| (x, y).into())),
            Segment::Vertical {
                x,
                y_start,
                y_end,
                order: Order::Decreasing,
            } => Box::new((y_start..y_end).rev().map(move |y| (x, y).into())),
            Segment::Horizontal {
                y,
                x_start,
                x_end,
                order: Order::Increasing,
            } => Box::new((x_start..x_end).map(move |x| (x, y).into())),
            Segment::Horizontal {
                y,
                x_start,
                x_end,
                order: Order::Decreasing,
            } => Box::new((x_start..x_end).rev().map(move |x| (x, y).into())),
        }
    }
}

impl Forest {
    fn segment_to_edge(&self, edge: Edge, coord: Coord) -> Segment {
        match edge {
            Edge::Top => vertical(coord.x, 0, coord.y, Order::Decreasing),
            Edge::Bottom => vertical(coord.x, coord.y + 1, self.0.len(), Order::Increasing),
            Edge::Left => horizontal(coord.y, 0, coord.x, Order::Decreasing),
            Edge::Right => horizontal(
                coord.y,
                coord.x + 1,
                self.0.get(coord.y).unwrap().len(),
                Order::Increasing,
            ),
        }
    }

    fn tree_height(&self, coord: &Coord) -> usize {
        self.0[coord.y][coord.x]
    }

    fn is_visible_tree(&self, coord: Coord) -> bool {
        use Edge::*;
        [Top, Bottom, Left, Right]
            .map(|edge| {
                self.segment_to_edge(edge, coord)
                    .into_iter()
                    .all(|hightest| self.tree_height(&hightest) < self.tree_height(&coord))
            })
            .into_iter()
            .any(|visible_on_segment| visible_on_segment)
    }

    fn scenic_score(&self, coord: Coord) -> usize {
        use Edge::*;
        [Top, Bottom, Left, Right]
            .map(|edge| {
                self.segment_to_edge(edge, coord)
                    .into_iter()
                    .fold((false, 0), |(reached_taller, count), curr_coord| {
                        match (
                            reached_taller,
                            self.tree_height(&curr_coord) >= self.tree_height(&coord),
                        ) {
                            (true, _) => (true, count),
                            (false, true) => (true, count + 1),
                            (false, false) => (false, count + 1),
                        }
                    })
                    .1
            })
            .iter()
            .product()
    }
}

#[derive(Copy, Clone)]
enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

use crate::parse::{table, ParseExt};
impl Problem for Forest {
    fn parse(lines: Vec<String>) -> Result<Self, ParsingError> {
        type Parser = Seq<table::Table<1, StrSep<Empty>, Natural<usize>>, LineSep>;
        let forest = Parser::parse(lines.join("\n").as_bytes())?;

        if forest.len() == 0 {
            return Err(ParsingError::UnverifiedConstraint("The forest is empty".into()));
        }
        if forest[0].len() == 0 {
            return Err(ParsingError::UnverifiedConstraint("The forest is empty".into()));
        }

        if !forest.iter().map(Vec::len).all_equal() {
            return Err(ParsingError::UnverifiedConstraint("The forest is not a square".into()));
        }
        Ok(Self(forest))
    }

    fn part_one(&self) -> Result<String, SolvingError> {
        let visible_trees_count = (0..self.0.get(0).unwrap().len())
            .cartesian_product(0..self.0.len())
            .filter(|tree_coord| self.is_visible_tree((*tree_coord).into()))
            .count();
        Ok(visible_trees_count.to_string())
    }

    fn part_two(&self) -> Result<String, SolvingError> {
        let max_scenir_score = (0..self.0.get(0).unwrap().len())
            .cartesian_product(0..self.0.len())
            .max_by_key(|tree_coord| self.scenic_score((*tree_coord).into()))
            .map(|tree_coord| self.scenic_score((tree_coord).into()));
        Ok(max_scenir_score.unwrap().to_string())
    }
}
