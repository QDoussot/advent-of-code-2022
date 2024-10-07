use crate::parse::StaticStr;
use itertools::Itertools;
use strum_macros::EnumString;

use crate::{parse::{separator::CommaSpace, Context, DefStaticStr}, prelude::*};

#[derive(Debug, EnumString, Copy, Clone)]
enum Operator {
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = "*")]
    Times,
}

#[derive(Debug, Copy, Clone)]
enum Operande {
    Old,
    Raw(usize),
}

impl From<either::Either<[usize; 0], usize>> for Operande {
    fn from(value: either::Either<[usize; 0], usize>) -> Self {
        match value {
            either::Either::Left(_) => Operande::Old,
            either::Either::Right(value) => Operande::Raw(value),
        }
    }
}

#[derive(Debug, derive_more::From, Copy, Clone)]
struct Operation {
    operator: Operator,
    right: Operande,
}

impl Operation {
    fn operate_inner(&self, value: usize) -> Option<usize> {
        match (self.operator, self.right) {
            (Operator::Plus, Operande::Old) => value.checked_add(value),
            (Operator::Plus, Operande::Raw(raw)) => value.checked_add(raw),
            (Operator::Times, Operande::Old) => value.checked_mul(value),
            (Operator::Times, Operande::Raw(raw)) => value.checked_mul(raw),
        }
    }

    fn operate(&self, item: Item) -> Option<Item> {
        match item {
            Item::Pure(value) => self.operate_inner(value).map(Item::Pure),

            Item::Reminds(reminds) => reminds
                .into_iter()
                .map(|remind| {
                    self.operate_inner(remind.value).map(|v| Remind {
                        value: v % remind.divider,
                        divider: remind.divider,
                    })
                })
                .collect::<Option<Vec<_>>>()
                .map(Item::Reminds),
        }
    }
}

#[derive(Debug, Clone)]
struct ThrowFetch {
    divider: usize,
    monkey_if_true: usize,
    monkey_if_false: usize,
}

#[derive(Debug, Clone)]
struct Remind {
    divider: usize,
    value: usize,
}

#[derive(Debug, Clone)]
enum Item {
    Pure(usize),
    Reminds(Vec<Remind>),
}

impl Item {
    fn can_be_divided_by(&self, divider: usize) -> bool {
        match self {
            Item::Pure(value) => value % divider == 0,
            Item::Reminds(reminds) => reminds.iter().find(|remind| remind.divider == divider).unwrap().value == 0,
        }
    }
}

impl From<usize> for Item {
    fn from(value: usize) -> Self {
        Item::Pure(value)
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: std::collections::VecDeque<Item>,
    operation: Operation,
    throw_fetch: ThrowFetch,
}

enum Error {
    WorryLevelTooHigh,
}

impl Into<SolvingError> for Error {
    fn into(self) -> SolvingError {
        SolvingError::ExpectationUnfulfilled("Worry level too high".to_string())
    }
}

impl Monkey {
    /// worry level, monkey id
    fn inspect_item(&mut self, divider: usize, _substracter: usize) -> Result<Option<(Item, usize)>, Error> {
        self.items
            .pop_front()
            .map(|item| {
                println!("Monkey {} inspect item {:?}", self.id, item);
                let mut new_worry = self.operation.operate(item).ok_or(Error::WorryLevelTooHigh)?;
                if divider != 1 {
                    if let Item::Pure(value) = new_worry {
                        new_worry = Item::Pure(value / divider)
                    }
                }
                if new_worry.can_be_divided_by(self.throw_fetch.divider) {
                    Ok((new_worry, self.throw_fetch.monkey_if_true))
                } else {
                    Ok((new_worry, self.throw_fetch.monkey_if_false))
                }
            })
            .transpose()
    }
}

impl Monkey {
    fn parse_from_capture(captured: [(Vec<u8>, Context, Context); 6]) -> Result<Self, ParsingError> {
        let id = Natural::<usize>::parse_with_context(&captured[0].0, captured[0].1)?;
        let items = Seq::<Natural<usize>, StrSep<CommaSpace>>::parse_with_context(&captured[1].0, captured[1].1)?;
        DefStaticStr!(Old,"old");

        // A bit of a hack here to check 'old' token
        type OperandeParser = Either<Capture<Old, 0, Natural<usize>>, Natural<usize>>;
        type OperationParser = Couple<Natural<Operator>, SpaceSep, OperandeParser>;
        let operation = OperationParser::parse_with_context(&captured[2].0, captured[2].1)?;
        let operation = (operation.0, Operande::from(operation.1)).into();

        let divider = Natural::<usize>::parse_with_context(&captured[3].0, captured[3].1)?;
        let monkey_if_true = Natural::<usize>::parse_with_context(&captured[4].0, captured[4].1)?;
        let monkey_if_false = Natural::<usize>::parse_with_context(&captured[5].0, captured[5].1)?;
        let throw_fetch = ThrowFetch {
            divider,
            monkey_if_true,
            monkey_if_false,
        };

        Ok(Self {
            id,
            items: items.into_iter().map(|value| Item::Pure(value)).collect(),
            operation,
            throw_fetch,
        })
    }
}

#[derive(Debug, Clone)]
pub struct MonkeyBehaviors(Vec<Monkey>);

DefStaticStr!(MonkeyFmt, "Monkey %:
  Starting items: %
  Operation: new = old %
  Test: divisible by %
    If true: throw to monkey %
    If false: throw to monkey %"
    );
impl Problem for MonkeyBehaviors {
    fn parse(lines: Vec<String>) -> Result<Self, ParsingError> {
        
        type MonkeyParser = Capture<MonkeyFmt, 6, crate::parse::keep::Keep>;
        type NoteParser = Seq<MonkeyParser, EmptyLineSep>;
        let monkey_behaviors = NoteParser::parse(lines.join("\n").as_bytes())?;
        let monkey_behaviors = monkey_behaviors
            .into_iter()
            .map(Monkey::parse_from_capture)
            .collect::<Result<_, ParsingError>>()?;

        Ok(Self(monkey_behaviors))
    }

    fn part_one(&self) -> Result<String, SolvingError> {
        let mut state = self.clone();

        let mut inspections: Vec<_> = self.0.iter().map(|_| 0).collect();
        for _ in 0..20 {
            for monkey in 0..self.0.len() {
                while let Some(throw) = state.0[monkey].inspect_item(3, 0).map_err(Into::into)? {
                    inspections[monkey] += 1;
                    println!("monkey {} thow {:?} at {}", monkey, throw.0, throw.1);
                    state.0[throw.1].items.push_back(throw.0);
                }
            }
        }
        let two_bests = inspections.into_iter().sorted().rev().take(2).collect::<Vec<_>>();
        Ok((two_bests[0] * two_bests[1]).to_string())
    }

    fn part_two(&self) -> Result<String, SolvingError> {
        let divider: Vec<_> = self.0.iter().map(|monkey| monkey.throw_fetch.divider).collect();
        let mut monkeys = self.0.clone();
        for monkey in monkeys.iter_mut() {
            let items = monkey
                .items
                .iter()
                .map(|item| match item {
                    Item::Pure(v) => Item::Reminds(
                        divider
                            .iter()
                            .map(|d| Remind {
                                value: v % d,
                                divider: *d,
                            })
                            .collect(),
                    ),
                    i => i.clone(),
                })
                .collect();
            monkey.items = items;
        }

        let mut inspections: Vec<_> = self.0.iter().map(|_| 0).collect();
        for _ in 0..10000 {
            for monkey in 0..self.0.len() {
                while let Some(throw) = monkeys[monkey].inspect_item(1, 0).map_err(Into::into)? {
                    inspections[monkey] += 1;
                    println!("monkey {} thow {:?} at {}", monkey, throw.0, throw.1);
                    monkeys[throw.1].items.push_back(throw.0);
                }
            }
        }
        println!("{:?}", inspections);
        let two_bests = inspections.into_iter().sorted().rev().take(2).collect::<Vec<_>>();
        Ok((two_bests[0] as u64 * two_bests[1] as u64).to_string())
    }
}
