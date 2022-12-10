use std::str::FromStr;

use crate::parse::couple::SplitFirst;
use crate::parse::natural::Natural;
use crate::parse::seq::{Skip, SkipFinal};
use crate::{
    parse::{
        couple::Couple,
        separator::{LineSep, StrSep},
        seq::Seq,
        ParseExt,
    },
    problem::{ParsingError, Problem, SolvingError},
};

use itertools::Itertools;

#[allow(dead_code)]
#[derive(Debug)]
struct File {
    size: usize,
    name: String,
}

#[derive(Debug)]
struct Directory {
    name: String,
    browsed: bool,
    subdirectories: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {
    fn root() -> Self {
        Directory {
            name: "/".to_string(),
            browsed: false,
            subdirectories: vec![],
            files: vec![],
        }
    }
    fn unbrowsed(name: String) -> Self {
        Self {
            name,
            browsed: false,
            subdirectories: vec![],
            files: vec![],
        }
    }

    fn total_size(&self) -> usize {
        self.files.iter().map(|file| file.size).sum::<usize>()
            + self.subdirectories.iter().map(|dir| dir.total_size()).sum::<usize>()
    }

    fn total_sum_of_subdirectories(&self, maximal_size: usize) -> usize {
        let my = if self.total_size() < maximal_size {
            self.total_size()
        } else {
            0
        };
        my + self
            .subdirectories
            .iter()
            .map(|dir| dir.total_sum_of_subdirectories(maximal_size))
            .sum::<usize>()
    }

    fn smallest_dir_with_size_over(&self, minimal_size: usize) -> Option<usize> {
        let my = (self.total_size() > minimal_size).then_some(self.total_size());
        let others = self
            .subdirectories
            .iter()
            .map(|dir| dir.smallest_dir_with_size_over(minimal_size))
            .flatten()
            .min();
        [my, others].iter().flatten().min().map(|e| *e)
    }
}

#[derive(Debug)]
pub struct FileSystem(Directory);

struct FileSystemBuilder {
    current_path: Vec<usize>,
    root: Directory,
}

impl FileSystemBuilder {
    fn new() -> Self {
        Self {
            current_path: vec![],
            root: Directory::root(),
        }
    }

    fn interpret_command(&mut self, (command, cmd_answers): (Command, Vec<CommandAnswerItem>)) -> Result<(), String> {
        use Command::*;
        use CommandAnswerItem::*;

        match (command, cmd_answers.as_slice()) {
            (Cd(dir), &[]) if &dir == "/" => Ok(self.current_path.clear()),
            (Cd(dir), &[]) if &dir == ".." => {
                self.current_path.pop();
                Ok(())
            }
            (Cd(dir), &[]) => {
                let (new_pos, _) = self
                    .get_dir(&self.current_path)?
                    .subdirectories
                    .iter()
                    .find_position(|d| d.name == dir)
                    .ok_or_else(|| format!("'{}' :wrong dir", dir))?;
                self.current_path.push(new_pos);
                Ok(())
            }
            (Ls, &[]) => {
                self.current_dir()?.browsed = true;
                self.current_dir()?.subdirectories.clear();
                self.current_dir()?.files.clear();
                Ok(())
            }

            (Ls, file_entries) => {
                let (files, dirs) = file_entries
                    .into_iter()
                    .fold((vec![], vec![]), |(mut files, mut dirs), fl| {
                        match fl {
                            FileDesc { size, name } => files.push(File {
                                size: *size,
                                name: name.to_owned(),
                            }),
                            DirDesc(name) => dirs.push(Directory::unbrowsed(name.to_owned())),
                        }
                        (files, dirs)
                    });
                self.current_dir()?.browsed = true;
                self.current_dir()?.subdirectories = dirs;
                self.current_dir()?.files = files;

                Ok(())
            }
            _ => Err("Invalid cmd".to_string()),
        }
    }

    fn get_dir_mut(&mut self, path: &[usize]) -> Result<&mut Directory, String> {
        path.into_iter().fold(Ok(&mut self.root), |curr_dir, path_item| {
            curr_dir?
                .subdirectories
                .get_mut(*path_item)
                .ok_or_else(|| format!("Did not find"))
        })
    }

    fn get_dir(&self, path: &[usize]) -> Result<&Directory, String> {
        path.into_iter().fold(Ok(&self.root), |curr_dir, path_item| {
            curr_dir?.subdirectories.get(*path_item).ok_or_else(|| "".to_string())
        })
    }

    fn current_dir(&mut self) -> Result<&mut Directory, String> {
        let slice = self.current_path.clone();
        self.get_dir_mut(&slice)
    }
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split(" ").collect::<Vec<_>>().as_slice() {
            &["cd", name] => Command::Cd(name.to_string()),
            &["ls"] => Command::Ls,
            _ => Err(format!("'{}' is not a valid command", s))?,
        })
    }
}

#[derive(Debug)]
enum CommandAnswerItem {
    FileDesc { size: usize, name: String },
    DirDesc(String),
    //ImplicitOk,
}

impl FromStr for CommandAnswerItem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split(" ").collect::<Vec<_>>().as_slice() {
            &["dir", name] => CommandAnswerItem::DirDesc(name.to_string()),
            &[size, name] => CommandAnswerItem::FileDesc {
                size: size.parse().map_err(|_| "Invalid file size".to_string())?,
                name: name.to_string(),
            },
            //       &[""] => CommandAnswerItem::ImplicitOk,
            _ => Err(format!("'{}' is not a valid command answer", s))?,
        })
    }
}

impl Problem for FileSystem {
    fn parse(lines: Vec<String>) -> Result<Self, ParsingError> {
        type CommandParser =
            Couple<Natural<Command>, LineSep, Seq<Natural<CommandAnswerItem>, LineSep, SkipFinal>, SplitFirst>;
        let cmds = Seq::<CommandParser, StrSep<"$ ">, Skip>::parse(lines.join("\n").as_bytes())?;

        let mut builder = FileSystemBuilder::new();

        for cmd in cmds {
            builder
                .interpret_command(cmd)
                .map_err(|e| ParsingError::UnverifiedConstraint(e))?;
        }
        Ok(Self(builder.root))
    }

    fn part_one(&self) -> Result<String, SolvingError> {
        Ok(self.0.total_sum_of_subdirectories(100000).to_string())
    }

    fn part_two(&self) -> Result<String, SolvingError> {
        let available = 70000000;
        let required = 30000000;
        let current = self.0.total_size();
        let free = available - current;
        let to_free = required - free;
        self.0
            .smallest_dir_with_size_over(to_free)
            .as_ref()
            .map(ToString::to_string)
            .ok_or(SolvingError::ExpectationUnfulfilled("No sol".into()))
    }
}
