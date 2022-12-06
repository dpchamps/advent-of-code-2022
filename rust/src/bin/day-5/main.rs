use std::str::from_boxed_utf8_unchecked;

struct StateParser {}

impl StateParser {
    fn parse_line(line: &String) -> Result<Vec<char>, String> {
        line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|register| match register.get(1) {
                Some(value) => Ok(value.clone()),
                None => Err(format!(
                    "Expected value in register, but got nothing. Input line: {}",
                    line
                )),
            })
            .collect()
    }

    pub fn create_vm_registers(input: &[String]) -> Result<Vec<Vec<char>>, String> {
        input
            .iter()
            .take(input.len() - 1)
            .map(StateParser::parse_line)
            .collect::<Result<Vec<Vec<char>>, String>>()
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Command {
    Move(usize),
    From(usize),
    To(usize),
}

#[derive(Eq, PartialEq, Debug)]
struct Instruction(Box<Program>, Box<Program>, Box<Program>);

#[derive(Eq, PartialEq, Debug)]
enum Program {
    Instruction(Instruction),
    Command(Command),
}

struct ProgramParser {
    buffer: Vec<char>,
    cursor: usize,
}

impl ProgramParser {
    fn new(input: &str) -> Self {
        Self {
            buffer: input.chars().collect(),
            cursor: 0,
        }
    }

    fn current(&self) -> Option<char> {
        if self.cursor == self.buffer.len() {
            return None;
        }

        Some(self.buffer[self.cursor])
    }

    fn eat_next(&mut self) -> Option<char> {
        match self.current() {
            Some(val) => {
                self.cursor += 1;
                Some(val)
            }
            None => None,
        }
    }

    fn eat(&mut self, symbol: char) -> Option<char> {
        match self.current() {
            Some(val) if val == symbol => {
                self.cursor += 1;
                Some(val)
            }
            _ => None,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(current) = self.current() {
            if !current.is_whitespace() {
                break;
            }
            self.cursor += 1;
        }
    }

    fn parse_digit(&mut self) -> Result<usize, String> {
        self.skip_whitespace();
        let mut digit_buffer = String::from("");

        while let Some(c) = self.current() {
            if !c.is_digit(10) {
                break;
            }
            self.eat_next();

            digit_buffer.push(c)
        }

        digit_buffer
            .parse()
            .map_err(|_| format!("Failed to parse digit {}", digit_buffer))
    }

    fn parse_digit_directive(&mut self, command: &str) -> Result<usize, String> {
        self.skip_whitespace();

        for c in command.chars() {
            self.eat(c).ok_or(format!(
                "Expected {} at {}, but found {:?}",
                c,
                self.cursor,
                self.current()
            ));
        }

        self.parse_digit()
    }

    fn parse_move(&mut self) -> Result<Program, String> {
        println!("parsing next move");
        Ok(Program::Command(Command::Move(
            self.parse_digit_directive("move")?,
        )))
    }

    fn parse_from(&mut self) -> Result<Program, String> {
        println!("parsing next from");
        Ok(Program::Command(Command::From(
            self.parse_digit_directive("from")?,
        )))
    }

    fn parse_to(&mut self) -> Result<Program, String> {
        println!("parsing next to");
        Ok(Program::Command(Command::To(
            self.parse_digit_directive("to")?,
        )))
    }

    fn parse_instruction(&mut self) -> Result<Program, String> {
        Ok(Program::Instruction(Instruction(
            Box::new(self.parse_move()?),
            Box::new(self.parse_from()?),
            Box::new(self.parse_to()?),
        )))
    }

    pub fn parse_program(&mut self) -> Result<Vec<Program>, String> {
        let mut program: Vec<Program> = vec![];
        while let Some(_) = self.current() {
            println!("parsing next instruction");
            program.push(self.parse_instruction()?)
        }

        Ok(program)
    }
}

struct Vm {
    registers: Option<Vec<Vec<char>>>,
    program: Option<Vec<Program>>,
}

fn main() -> std::io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod day_5_tests {
    use crate::*;

    #[test]
    fn state_parser_smoke_test() {
        let input = "[D]
[N] [C]
[Z] [M] [P]
 1   2   3 "
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let registers = StateParser::create_vm_registers(&input).expect("Could not create VM");

        assert_eq!(registers[0].len(), 1);
        assert_eq!(registers[0][0], 'D');
        assert_eq!(registers.len(), 3);
    }

    #[test]
    fn program_parser_smoke_test() {
        let input = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1";

        let mut parser = ProgramParser::new(input);
        let program = parser.parse_program().expect("Could not parse program");

        assert_eq!(
            &program,
            &[
                Program::Instruction(Instruction(
                    Box::new(Program::Command(Command::Move(1))),
                    Box::new(Program::Command(Command::From(2))),
                    Box::new(Program::Command(Command::To(1)))
                )),
                Program::Instruction(Instruction(
                    Box::new(Program::Command(Command::Move(3))),
                    Box::new(Program::Command(Command::From(1))),
                    Box::new(Program::Command(Command::To(3)))
                )),
                Program::Instruction(Instruction(
                    Box::new(Program::Command(Command::Move(2))),
                    Box::new(Program::Command(Command::From(2))),
                    Box::new(Program::Command(Command::To(1)))
                ))
            ]
        )
    }
}
