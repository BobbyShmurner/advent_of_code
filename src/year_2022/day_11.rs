use crate::macros::*;
use crate::BoxedError;
use crate::DayReturnType;

use regex::Regex;

enum Operator {
    Addition,
    Multiplication,
}

struct Monkey {
    items: Vec<u32>,
    operator: Operator,
    operand: u32,
    test: u32,
    throw_monkeys: (usize, usize),
    inspects: u32,
}

impl Monkey {
    fn new(
        starting_items: &str,
        operator: &str,
        operand: &str,
        test: &str,
        true_monkey: &str,
        false_monkey: &str,
    ) -> Result<Self, BoxedError> {
        let mut items = Vec::new();

        for item in starting_items.trim().split(", ") {
            items.push(unwrap_or_return!(
                item.parse::<u32>(),
                "\"{}\" is not a valid starting item!",
                item
            ));
        }

        let operator = match operator.trim() {
            "+" => Operator::Addition,
            "*" => Operator::Multiplication,
            _ => return_err!("\"{}\" is not a valid operator", operator),
        };

        let operand = if operand.trim().to_lowercase() == "old" {
            0
        } else {
            unwrap_or_return!(
                operand.trim().parse::<u32>(),
                "Invalid operand \"{}\"",
                operand
            )
        };

        let test = unwrap_or_return!(test.trim().parse::<u32>(), "Invalid test \"{}\"", test);

        let true_monkey = unwrap_or_return!(
            true_monkey.trim().parse::<usize>(),
            "Invalid monkey for the true case: \"{}\"",
            true_monkey
        );

        let false_monkey = unwrap_or_return!(
            false_monkey.trim().parse::<usize>(),
            "Invalid monkey for the false case: \"{}\"",
            false_monkey
        );

        Ok(Monkey {
            items,
            operator,
            operand,
            test,
            throw_monkeys: (true_monkey, false_monkey),
            inspects: 0,
        })
    }

    fn get_new_worry_level(&self, item: u32) -> u32 {
        let operand = if self.operand == 0 {
            item
        } else {
            self.operand
        };

        let new_worry = match self.operator {
            Operator::Addition => item + operand,
            Operator::Multiplication => item * operand,
        };

        new_worry / 3
    }

    fn throw_all_items(monkey_index: usize, monkeys: &mut [Monkey]) -> Result<(), BoxedError> {
        let mut items_to_move = Vec::new();
        let monkey = &mut monkeys[monkey_index];

        for item in &monkey.items {
            let new_worry = monkey.get_new_worry_level(*item);

            let target_monkey_index = if new_worry % monkey.test == 0 {
                monkey.throw_monkeys.0
            } else {
                monkey.throw_monkeys.1
            };

            items_to_move.push((new_worry, target_monkey_index));
            monkey.inspects += 1;
        }

        for (item, i) in items_to_move {
            let target_monkey =
                unwrap_or_return_option!(monkeys.get_mut(i), "Invalid monkey \"{}\"", i);

            target_monkey.items.push(item);
        }

        monkeys[monkey_index].items.clear();
        Ok(())
    }

    fn complete_round(monkeys: &mut [Monkey]) -> Result<(), BoxedError> {
        for i in 0..monkeys.len() {
            Monkey::throw_all_items(i, monkeys)?;
        }

        Ok(())
    }

    fn get_monkey_business(monkeys: &[Monkey]) -> Result<u32, BoxedError> {
        let mut inspects = Vec::new();

        for monkey in monkeys {
            inspects.push(monkey.inspects);
        }

        if inspects.len() < 2 {
            return_err!(
                "Not enough monkeys! There should be at least 2, but there are only {}",
                inspects.len()
            );
        }

        inspects.sort();
        Ok(inspects.pop().unwrap() * inspects.pop().unwrap())
    }
}

pub fn execute(input: &str) -> DayReturnType {
    let re = Regex::new(r"Monkey (?P<monkey_num>\d+):\n  Starting items: (?P<starting_items>(?:\d+, )*\d+)\n  Operation: new = old (?P<operator>[+*]) (?P<operand>(?:\d+|old))\n  Test: divisible by (?P<test>\d+)\n    If true: throw to monkey (?P<true_monkey>\d+)\n    If false: throw to monkey (?P<false_monkey>\d+)").unwrap();

    let mut monkeys = Vec::new();

    for caps in re.captures_iter(input) {
        monkeys.push(Monkey::new(
            &caps["starting_items"],
            &caps["operator"],
            &caps["operand"],
            &caps["test"],
            &caps["true_monkey"],
            &caps["false_monkey"],
        )?);
    }

    for _i in 0..20 {
        Monkey::complete_round(&mut monkeys)?;
    }

    Ok((
        Monkey::get_monkey_business(&monkeys)?.to_string(),
        "Not Implemented".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let input = r#"Monkey 0:
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
    If false: throw to monkey 1"#;

        let result = super::execute(input).unwrap().0;
        assert_eq!("10605", result);
    }

    #[test]
    fn part2_example() {
        let input = r#"Monkey 0:
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
    If false: throw to monkey 1"#;

        let result = super::execute(input).unwrap().1;
        assert_eq!("Not Implemented", result);
    }
}
