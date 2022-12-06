use std::fmt::{Display, Formatter};
use advent_of_code_2022::core::{get_data, get_lines};

#[derive(Eq, PartialEq, Debug)]
enum Value {
    Int(usize),
}
#[derive(Eq, PartialEq, Debug)]
struct MoveInstruction(Value, Value, Value);

#[derive(Eq, PartialEq, Debug)]
enum Instruction {
    MoveInstruction(MoveInstruction),
}

#[derive(Eq, PartialEq, Debug)]
enum ProgramNode {
    Instruction(Instruction),
    Command(Value),
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
            if !c.is_ascii_digit() {
                break;
            }
            self.eat_next();

            digit_buffer.push(c)
        }

        digit_buffer
            .parse()
            .map_err(|_| format!("Failed to parse digit {}", digit_buffer))
    }

    fn parse_int_value(&mut self, var_name: &str) -> Result<Value, String> {
        self.skip_whitespace();

        for c in var_name.chars() {
            self.eat(c).ok_or(format!(
                "Expected {} at {}, but found {:?}",
                c,
                self.cursor,
                self.current()
            ));
        }

        Ok(Value::Int(self.parse_digit()?))
    }

    fn parse_instruction(&mut self) -> Result<ProgramNode, String> {
        Ok(ProgramNode::Instruction(Instruction::MoveInstruction(
            MoveInstruction(
                self.parse_int_value("move")?,
                self.parse_int_value("from")?,
                self.parse_int_value("to")?),
        )))
    }

    pub fn parse_program(&mut self) -> Result<Vec<ProgramNode>, String> {
        let mut program: Vec<ProgramNode> = vec![];
        while self.current().is_some() {
            program.push(self.parse_instruction()?)
        }

        Ok(program)
    }
}

struct Vm {
    registers: Vec<Vec<char>>,
}

impl Display for Vm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_register_len = self.registers.iter().map(|x| x.len()).max().expect("");
        let mut registers = String::new();
        let register_list = (1..=self.registers.len()).map(|x| format!(" {}  ", x)).collect::<String>();
        for i in 0..=max_register_len {
            let i = max_register_len-i;
            for register in self.registers.iter() {
                match register.get(i){
                    Some(content) => registers.push_str(&format!("[{}] ", content)),
                    _ => registers.push_str("    ")
                }
            }
            registers.push('\n');
        }

        write!(
            f,
            "VM State -----\n{}{}\n------------",
            registers,
            register_list
        )
    }
}

impl Vm {
    fn exec_move(&mut self, m: usize, f: usize, t: usize) -> Result<(), String> {
        let from_length = self.registers[f].len();
        let drain_range = from_length-m..from_length;
        let mut to_append: Vec<char> = self.registers[f].drain(drain_range).collect();
        self.registers[t].append(&mut to_append);

        Ok(())
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), String> {
        match instruction {
            Instruction::MoveInstruction(MoveInstruction(
                                             Value::Int(m),
                                             Value::Int(f),
                                             Value::Int(t),
            )) => self.exec_move(*m, f - 1, t - 1),
            _ => Err(format!("Got an unknown instruction {:?}", instruction)),
        }
    }

    pub fn run_program(&mut self, program: &[ProgramNode]) -> Result<(), String> {
        for node in program {
            match node {
                ProgramNode::Instruction(instruction) => self.execute_instruction(instruction),
                _ => return Err(format!("Encountered an invalid ProgramNode: {:?}", node)),
            };
        }

        Ok(())
    }

    pub fn get_tops_of_stacks(&self) -> String {
        self.registers.iter().map(|reg| match reg.last() {
            Some(c) => c.to_string(),
            None => "".to_string()
        })
            .collect()
    }
}

struct StateParser {}

impl StateParser {
    fn get_max_line_len(lines: &[String]) -> usize {
        lines.iter().map(|x| x.len()).max().expect("Got something unexpected for max_line_len")
    }

    fn parse_line(line: &String) -> Result<Vec<(usize, Option<char>)>, String> {
        Ok(line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|register| match register.get(1) {
                Some(value) if *value == ' ' => Ok(None),
                Some(value) => Ok(Some(*value)),
                None => Err(format!(
                    "Expected value in register, but got nothing. Input line: {}",
                    line
                )),
            })
            .collect::<Result<Vec<Option<char>>, String>>()?
            .into_iter()
            .enumerate()
            .collect())
    }

    pub fn create_vm_registers(input: &[String]) -> Result<Vec<Vec<char>>, String> {
        let n_registers = (StateParser::get_max_line_len(input)+1) / 4;
        let mut registers = vec![Vec::new(); n_registers];

        for line in input.iter().take(input.len()-1) {
            for (register, maybe_val) in StateParser::parse_line(line)? {
                match maybe_val {
                    Some(c) => registers[register].insert(0, c),
                    None => ()
                }
            }
        }

        Ok(registers)
    }
}

fn parse_input(input: &str) -> Result<(Vm, Vec<ProgramNode>), String> {
    let split = input.split("\n\n").map(String::from).collect::<Vec<String>>();
    let registers_string = split.get(0).ok_or("Unexpected, couldn't find registers!")?;
    let mut program_parser = ProgramParser::new(split.get(1).ok_or("Unexpected, couldn't find program!")?);

    return Ok((
        Vm {
            registers: StateParser::create_vm_registers(
                &registers_string.lines().map(String::from).collect::<Vec<String>>(),
            )?,
        },
        program_parser.parse_program()?,
    ));
}

fn part_one(input: &String) -> Result<String, String> {
    let (mut vm, program) = parse_input(&input)?;

    vm.run_program(&program);

    Ok(vm.get_tops_of_stacks())
}

fn main() -> std::io::Result<()> {
    let input = get_data("day-5")?;

    println!("Part One: {}", part_one(&input).expect("Could not solve Part One"));
    Ok(())
}

#[cfg(test)]
mod day_5_tests {
    use crate::*;

    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

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

        assert_eq!(registers[0].len(), 3);
        assert_eq!(registers[0][0], 'Z');
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
                ProgramNode::Instruction(Instruction::MoveInstruction(MoveInstruction(
                    Value::Int(1),
                    Value::Int(2),
                    Value::Int(1),
                ))),
                ProgramNode::Instruction(Instruction::MoveInstruction(MoveInstruction(
                    Value::Int(3),
                    Value::Int(1),
                    Value::Int(3),
                ))),
                ProgramNode::Instruction(Instruction::MoveInstruction(MoveInstruction(
                    Value::Int(2),
                    Value::Int(2),
                    Value::Int(1),
                ))),
            ]
        )
    }

    #[test]
    fn vm_smoketest() {
        let initial_state = "[D]
[N] [C]
[Z] [M] [P]
 1   2   3 "
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let mut vm = Vm {
            registers: StateParser::create_vm_registers(&initial_state)
                .expect("Could not create VM Registers"),
        };

        let instruction = Instruction::MoveInstruction(MoveInstruction(
            Value::Int(1),
            Value::Int(1),
            Value::Int(2),
        ));


        vm.execute_instruction(&instruction)
            .expect("Could not execute instruction");

    }

    // #[test]
    // fn part_one_example(){
    //     let (mut vm, program) = parse_input(INPUT).expect("Could not create input");
    //     vm.run_program(&program).expect("Failed to run program");
    //
    //     assert_eq!(vm.get_tops_of_stacks(), "CMZ");
    // }

    #[test]
    fn part_two_example(){
        let (mut vm, program) = parse_input(INPUT).expect("Could not create input");
        vm.run_program(&program).expect("Failed to run program");

        assert_eq!(vm.get_tops_of_stacks(), "MCD");
    }
}
