use std::collections::BTreeMap;
use std::process::id;
use chumsky::chain::Chain;
use chumsky::prelude::todo;
use iter_tools::Itertools;
use lazy_static::lazy_static;
use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser;

pub fn part1(input: Vec<String>) -> Result<String, anyhow::Error> {
    let monkeys = parse_monkeys(&input.join("\n"));
    let mut monkeys : BTreeMap<u32, Monkey> = monkeys.into_iter().map(|m| (m.id, m)).collect();
    let mut rounds = 0;
    let monkeys_after_20_rounds = (0..20).fold(monkeys, |monkeys, round|{
        rounds += 1;
        do_monkey_dance(monkeys)
    });
    let mut most_active_monkeys = monkeys_after_20_rounds.values()
        .sorted_by(|a,b|  b.number_of_inspected_items.cmp(&a.number_of_inspected_items));

    let m1 = most_active_monkeys.next().unwrap();
    let m2 = most_active_monkeys.next().unwrap();
    println!("{}, {}", m1.number_of_inspected_items, m2.number_of_inspected_items);
    let monkey_business = m1.number_of_inspected_items * m2.number_of_inspected_items;

    Ok(format!("Monkey business after {} rounds is {}", rounds,monkey_business))
}

#[derive(pest_derive::Parser)]
#[grammar = "day11/monkey.pest"]
pub struct MonkeyParser;

#[derive(Debug)]
pub enum Operation {
    Add(i64),
    Subtract(i64),
    Multiply(i64),
    Divide(i64),
    Square
}

impl Operation {
    pub fn apply(&self, value: i64) -> i64 {
        match self {
            Operation::Add(op_value) => {
                value + op_value
            }
            Operation::Subtract(op_value) => {
                value - op_value
            }
            Operation::Multiply(op_value) => {
                value * op_value
            }
            Operation::Divide (op_value)=> {
                value / op_value
            }
            Operation::Square => value * value
        }
    }
}
#[derive(Debug)]
pub enum Test{
    DivisibleBy(i64)
}

impl Test {
    pub fn check_item(&self, item: &i64) -> bool {
        match self {
            Test::DivisibleBy(value) => {
                value & item == 0
            }
        }
    }
}

#[derive(Debug)]
pub enum BinOpp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum  Action {
    ThrowToMonkey(u32)
}
#[derive(Debug)]
pub struct Monkey {
    pub id: u32,
    pub number_of_inspected_items : usize,
    pub starting : Vec<i64>,
    pub operation: Operation,
    pub test: Test,
    pub if_true: Action,
    pub if_false: Action
}

impl Monkey {
    pub fn do_monkey_things(&mut self) -> Vec<(Action, i64)> {
        let current_worry_levels : Vec<i64>= self.starting.drain(0..)
            .into_iter().collect();
        self.number_of_inspected_items += current_worry_levels.len();
        current_worry_levels.
            into_iter()
            .map(|level| self.inspect(level))
            .map(|new_level| (self.decide_next_action(&new_level), new_level))
            .collect()
    }
    pub fn inspect(&self, item: i64) -> i64 {
        let new_level = self.operation.apply(item);
        let because_it_is_boring = new_level / 3;
        println!(r###"Monkey {}:
    Monkey inspects an item with a worry level of  {}
        New worry level is {}.
        Monkey gets bored with item. Worry level is divided by 3 to {}."###, self.id, item,  &new_level, because_it_is_boring);
        because_it_is_boring
    }

    pub fn decide_next_action(&self, item: &i64) -> Action {
        if self.test.check_item(item) {
            println!("\t\tItem is thrown to {:?}",&self.if_true);
            self.if_true.clone()
        } else {
            println!("\t\tItem is thrown to {:?}",&self.if_false);
            self.if_false.clone()
        }
    }
}
pub fn parse_monkeys(input: &str) -> Vec<Monkey> {
    use pest::iterators::Pair;

    fn parse_operation(pair: Pair<Rule>) -> Operation {
        let mut cursoer = pair.into_inner();
        let op = cursoer.next().unwrap();
        match op.as_rule() {
            Rule::add => Operation::Add(cursoer.next().unwrap().as_str().parse().unwrap()),
            Rule::multiply => Operation::Multiply(cursoer.next().unwrap().as_str().parse().unwrap()),
            Rule::subtract => Operation::Subtract(cursoer.next().unwrap().as_str().parse().unwrap()),
            Rule::divide => Operation::Divide(cursoer.next().unwrap().as_str().parse().unwrap()),
            Rule::square => Operation::Square,
            _ => panic!("Unknown binary op")
        }
    }

    fn parse_test(test_rule: Pair<Rule>) -> Test {
        Test::DivisibleBy(test_rule.into_inner().next().unwrap().as_str().parse().unwrap())
    }

    fn parse_if(test_rule: Pair<Rule>) -> Action {
        let mut cursor = test_rule.into_inner();
        let action = cursor.next().unwrap();
        let target = action.into_inner().next().unwrap();
        Action::ThrowToMonkey(target.as_str().parse().unwrap())
    }
    fn parse_monkey(monkey_rules: Pair<Rule>) -> Monkey {
        let mut cursor = monkey_rules.into_inner();
        let id_rule = cursor.next().unwrap();
        let id = id_rule.into_inner().next().unwrap().as_str().parse().unwrap();
        let starting: Vec<i64> = cursor.next().unwrap().into_inner().map(|s| s.as_str().parse().unwrap()).collect();
        let operation = parse_operation(cursor.next().unwrap());
        let test = parse_test(cursor.next().unwrap());
        let if_true = parse_if(cursor.next().unwrap());
        let if_false = parse_if(cursor.next().unwrap());
        Monkey {
            id,
            starting,
            operation,
            test,
            if_true,
            if_false,
            number_of_inspected_items: 0
        }
    }
    fn parse_value(pairs: Pairs<Rule>) -> Vec<Monkey> {
        pairs.into_iter()
            .filter_map(|pair| {
                match pair.as_rule() {
                    Rule::monkey => {
                        Some(parse_monkey(pair))
                    }
                    Rule::EOI => {
                        None
                    }
                    a => {
                        println!("I did not expect to find rule {:?} here!", &a);
                        None
                    }
                }
            })
            .collect()
    }
    parse_value(MonkeyParser::parse(Rule::monkeys, input).unwrap())
}
pub fn do_monkey_dance(mut monkeys: BTreeMap<u32, Monkey>) -> BTreeMap<u32, Monkey> {

    let ids : Vec<u32> = monkeys.keys().cloned().collect();
    for monkey_id in ids {
        let current_monkey = monkeys.get_mut(&monkey_id).unwrap();
        let actions = current_monkey.do_monkey_things();
        for (Action::ThrowToMonkey(target), item) in actions {
            let target = monkeys.get_mut(&target).unwrap();
            target.starting.push(item);
        }
    }
    monkeys

}
#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use pest::iterators::Pair;
    use pest::Parser;
    use super::*;

    const TEST_INPUT: &'static str = r###"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"###;

    #[test]
    pub fn test_parser() {
        let monkeys = parse_monkeys(TEST_INPUT);
        let mut monkeys : BTreeMap<u32, Monkey> = monkeys.into_iter().map(|m| (m.id, m)).collect();

        // round one
        monkeys = do_monkey_dance(monkeys);
        let empty: Vec<i64> = vec![];
        assert_eq!(&vec![20, 23, 27, 26], &monkeys[&0].starting);
        assert_eq!(&empty, &monkeys[&2].starting);
        assert_eq!(&empty, &monkeys[&3].starting);
        assert_eq!(&vec![2080, 25, 167, 207, 401, 1046], &monkeys[&1].starting);

    }
}
