#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(is_sorted)]
#![feature(iter_repeat_n)]
#![feature(iter_array_chunks)]

use std::io::{self, BufRead, BufReader};
use structopt::StructOpt;

mod day01;
mod day02;
mod day03;
mod day04;

mod day05;
mod day06;
mod day07;
mod day08;

mod day09;
mod day10;
mod day11;
mod parse;
mod problem;
use problem::Error;

pub mod prelude {
    pub use crate::parse::{
        capture::Capture,
        couple::Couple,
        either::Either,
        natural::Natural,
        separator::{EmptyLineSep, LineSep, SpaceSep, StrSep},
        seq::Seq,
        table::Table,
        ParseExt,
    };
    pub use crate::problem::*;
}

#[derive(StructOpt)]
struct Opt {
    day: usize,
    part: usize,
    #[structopt(long)]
    input: Option<String>,
    #[structopt(long, conflicts_with = "input")]
    example: bool,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let file_name = match opt.input {
        None => {
            let ext = match opt.example {
                false => "",
                true => ".example",
            };
            format!("inputs/{}{}", opt.day, ext)
        }
        Some(file_name) => file_name,
    };
    let file = std::fs::File::open(file_name).map_err(|e| Error::CantOpenInputFile(e.to_string()))?;
    let lines = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    let solution: Result<String, problem::Error> = match opt.day {
        1 => problem::solve::<day01::Inventories>(lines, opt.part),
        2 => problem::solve::<day02::Guide>(lines, opt.part),
        3 => problem::solve::<day03::RuckSacks>(lines, opt.part),
        4 => problem::solve::<day04::AssignmentsPairs>(lines, opt.part),
        5 => problem::solve::<day05::RearrangementProcedure>(lines, opt.part),
        6 => problem::solve::<day06::Signal>(lines, opt.part),
        7 => problem::solve::<day07::FileSystem>(lines, opt.part),
        8 => problem::solve::<day08::Forest>(lines, opt.part),
        9 => problem::solve::<day09::Movements>(lines, opt.part),
        10 => problem::solve::<day10::Program>(lines, opt.part),
        11 => problem::solve::<day11::MonkeyBehaviors>(lines, opt.part),

        _ => Err(Error::NoCorrespondingSolver),
    };
    println!("{}", solution?);

    Ok(())
}
