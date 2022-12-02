use structopt::StructOpt;
use std::io::{self, BufRead, BufReader};

mod problem;
use problem::Error;

#[derive(StructOpt)]
struct Opt {
    day: usize,
    _part: usize,
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
    let _lines = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    let solution :Result<String, problem::Error>= match opt.day {
        1 => Err(Error::NoCorrespondingSolver),
        _ => Err(Error::NoCorrespondingSolver),
    };
    println!("{}", solution?);

    Ok(())
}

