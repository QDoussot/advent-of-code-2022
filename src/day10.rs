use crate::prelude::*;
use itertools::Itertools;
use strum_macros::EnumString;

#[derive(EnumString, Debug, Copy, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum CodeOp {
    NoOp,
    AddX,
}

#[derive(EnumString, Debug, Copy, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum NoArgCodeOp {
    NoOp,
}

#[derive(EnumString, Debug, Copy, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum OneArgCodeOp {
    AddX,
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    NoArgCodeOp(NoArgCodeOp),
    OneArgCodeOp(OneArgCodeOp, isize),
}

impl Instruction {
    fn into_event_vec(self) -> Vec<ProcEvent> {
        match self {
            Instruction::NoArgCodeOp(NoArgCodeOp::NoOp) => vec![ProcEvent::NothingHappens],
            Instruction::OneArgCodeOp(OneArgCodeOp::AddX, op) => vec![ProcEvent::NothingHappens, ProcEvent::AddX(op)],
        }
    }
}

enum ProcEvent {
    NothingHappens,
    AddX(isize),
}

impl From<either::Either<NoArgCodeOp, (OneArgCodeOp, isize)>> for Instruction {
    fn from(value: either::Either<NoArgCodeOp, (OneArgCodeOp, isize)>) -> Self {
        match value {
            itertools::Either::Left(no_arg) => Instruction::NoArgCodeOp(no_arg),
            itertools::Either::Right((one_arg, arg)) => Instruction::OneArgCodeOp(one_arg, arg),
        }
    }
}

#[derive(Debug)]
pub struct Program(Vec<Instruction>);

impl Problem for Program {
    fn parse(lines: Vec<String>) -> Result<Self, ParsingError> {
        type Parser =
            Seq<Either<Natural<NoArgCodeOp>, Couple<Natural<OneArgCodeOp>, SpaceSep, Natural<isize>>>, LineSep>;
        let pb = Parser::parse(lines.join("\n").as_bytes())?
            .into_iter()
            .map(Instruction::from)
            .collect();
        Ok(Self(pb))
    }

    fn part_one(&self) -> Result<String, SolvingError> {
        let mut x: isize = 1;
        let res = self
            .0
            .iter()
            .flat_map(|ins| Instruction::into_event_vec(*ins).into_iter())
            .map(|ev| {
                let x_start = x;
                match ev {
                    ProcEvent::NothingHappens => (),
                    ProcEvent::AddX(op) => x += op,
                }
                x_start
            })
            .enumerate()
            .map(|(zeroed_cycle, v)| (zeroed_cycle + 1, v))
            .filter(|(cycle, _)| ((*cycle + 20) % 40 == 0 && *cycle <= 220))
            .map(|(cycle, value)| cycle as isize * value)
            .sum::<isize>();
        Ok(res.to_string())
    }

    fn part_two(&self) -> Result<String, SolvingError> {
        let mut x = 1;
        let res = self
            .0
            .iter()
            .flat_map(|ins| Instruction::into_event_vec(*ins).into_iter())
            .map(|ev| {
                let x_start = x;
                match ev {
                    ProcEvent::NothingHappens => (),
                    ProcEvent::AddX(op) => x += op,
                }
                x_start
            })
            .enumerate()
            .map(|(zeroed_cycle, v)| (zeroed_cycle + 1, v))
            .map(|(cycle, sprite_x)| {
                let x = (cycle - 1) % 40;
                if isize::abs(sprite_x - (x as isize)) <= 1 {
                    '#'
                } else {
                    '.'
                }
            })
            .collect::<Vec<_>>();
        let lines = res
            .iter()
            .array_chunks()
            .map(|v: [_; 40]| Vec::from(v).into_iter().join(""))
            .join("\n");
        Ok(lines)
    }
}
