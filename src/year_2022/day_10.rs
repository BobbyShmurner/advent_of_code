use crate::macros::*;
use crate::BoxedError;
use crate::DayReturnType;

trait Operation {
    fn tick(&mut self, cpu: &mut Cpu) -> bool;
    fn new(operand: &str) -> Result<Box<Self>, BoxedError>
    where
        Self: Sized;
}

struct NoOp {}

impl Operation for NoOp {
    fn tick(&mut self, _cpu: &mut Cpu) -> bool {
        true
    }

    fn new(_operand: &str) -> Result<Box<Self>, BoxedError>
    where
        Self: Sized,
    {
        Ok(Box::new(Self {}))
    }
}

struct Add {
    cycles_remaining: u32,
    operand: i32,
}

impl Operation for Add {
    fn tick(&mut self, cpu: &mut Cpu) -> bool {
        self.cycles_remaining -= 1;

        if self.cycles_remaining != 0 {
            return false;
        }

        cpu.register += self.operand;
        true
    }

    fn new(operand: &str) -> Result<Box<Self>, BoxedError>
    where
        Self: Sized,
    {
        let operand: i32 = unwrap_or_return!(
            operand.parse(),
            "Invalid operand \"{}\" for opcode \"addx\"",
            operand
        );

        Ok(Box::new(Self {
            operand,
            cycles_remaining: 2,
        }))
    }
}

struct Cpu {
    operations: Option<Vec<Box<dyn Operation>>>,
    cycle: i32,
    cycle_until_next_strength_update: u32,
    register: i32,
    total_signal_strength: i32,
    display: String,
}

impl Cpu {
    fn new(operations: Vec<Box<dyn Operation>>) -> Self {
        Cpu {
            operations: Some(operations),
            cycle: 0,
            cycle_until_next_strength_update: 20,
            total_signal_strength: 0,
            register: 1,
            display: String::new(),
        }
    }

    fn update_signal_strength(&mut self) {
        self.total_signal_strength += self.cycle * self.register;
    }

    fn tick(&mut self) -> Result<(), BoxedError> {
        self.cycle += 1;
        self.cycle_until_next_strength_update -= 1;

        if (self.register - ((self.cycle - 1) % 40)).abs() <= 1 {
            self.display += "#";
        } else {
            self.display += ".";
        }

        if self.cycle % 40 == 0 {
            self.display += "\n";
        }

        if self.cycle_until_next_strength_update == 0 {
            self.update_signal_strength();
            self.cycle_until_next_strength_update = 40;
        }

        let mut operations = self.operations.take().unwrap();

        if operations.is_empty() {
            return_err!("Not enough operations!!!");
        }

        if operations[0].tick(self) {
            operations.remove(0);
        }

        self.operations = Some(operations);
        Ok(())
    }
}

fn parse_operation(input: &str) -> Result<Box<dyn Operation>, BoxedError> {
    let input = input.trim().to_lowercase();

    Ok(if !input.contains(' ') {
        match input.as_str() {
            "noop" => NoOp::new("")?,
            _ => return_err!("Invalid operation \"{}\"", input),
        }
    } else {
        let split: Vec<&str> = input.split(' ').collect();

        if split.len() != 2 {
            return_err!("Invalid Operation \"{}\"", input);
        }

        let opcode = split[0].trim();
        let operand = split[1].trim();

        match opcode {
            "addx" => Add::new(operand)?,
            _ => return_err!(
                "Invalid operation \"{}\" with operand \"{}\"",
                opcode,
                operand
            ),
        }
    })
}

fn parse_operations(input: &str) -> Result<Vec<Box<dyn Operation>>, BoxedError> {
    let mut operations = Vec::new();

    for line in input.lines() {
        operations.push(parse_operation(line)?);
    }

    Ok(operations)
}

pub fn execute(input: &str) -> DayReturnType {
    let operations = parse_operations(input)?;
    let mut cpu = Cpu::new(operations);

    for _i in 0..220 {
        cpu.tick()?;
    }

    let signal_strnegth = cpu.total_signal_strength;

    for _i in 0..20 {
        cpu.tick()?;
    }

    let display = cpu.display.trim();

    Ok((signal_strnegth.to_string(), "\n\n".to_string() + display))
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_example() {
        let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

        let result = super::execute(input).unwrap().0;
        assert_eq!("13140", result);
    }

    #[test]
    fn part2_example() {
        let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

        let result = super::execute(input).unwrap().1;
        assert_eq!(
            r#"

##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#,
            result
        );
    }
}
