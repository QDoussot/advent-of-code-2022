#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(is_sorted)]

use std::io::{self, BufRead, BufReader};
use structopt::StructOpt;

mod day1;
mod day2;
mod day3;
mod day4;

mod day5;
mod parse;
mod problem;
use problem::Error;

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
        1 => problem::solve::<day1::Inventories>(lines, opt.part),
        2 => problem::solve::<day2::Guide>(lines, opt.part),
        3 => problem::solve::<day3::RuckSacks>(lines, opt.part),
        4 => problem::solve::<day4::AssignmentsPairs>(lines, opt.part),
        5 => problem::solve::<day5::RearrangementProcedure>(lines, opt.part),

        _ => Err(Error::NoCorrespondingSolver),
    };
    println!("{}", solution?);

    Ok(())
}
