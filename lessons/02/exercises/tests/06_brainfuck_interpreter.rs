use std::str;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    UnknownInstruction { location: usize, instruction: char },
    UnmatchedLoop { location: usize },
}

#[derive(Debug, Eq, PartialEq)]
pub enum ExecuteError {
    NoInputLeft,
    InfiniteLoop,
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    IncrementPointer,    // >
    DecrementPointer,    // <
    Increment,           // +
    Decrement,          // -
    Output,             // .
    Input,              // ,
    LoopStart,          // [
    LoopEnd,            // ]
}

#[derive(Debug, Eq, PartialEq)]
pub struct Program {
    instructions: Vec<Instruction>,
}

pub fn parse_program(input: &str) -> Result<Program, ParseError> {
    let mut instructions = Vec::new();
    let mut loop_stack = Vec::new();
    
    for (pos, ch) in input.chars().enumerate() {
        let instruction = match ch {
            '>' => Instruction::IncrementPointer,
            '<' => Instruction::DecrementPointer,
            '+' => Instruction::Increment,
            '-' => Instruction::Decrement,
            '.' => Instruction::Output,
            ',' => Instruction::Input,
            '[' => {
                loop_stack.push(pos);
                Instruction::LoopStart
            }
            ']' => {
                if loop_stack.pop().is_none() {
                    return Err(ParseError::UnmatchedLoop { location: pos });
                }
                Instruction::LoopEnd
            }
            ch if ch.is_whitespace() => continue,
            _ => return Err(ParseError::UnknownInstruction {
                location: pos,
                instruction: ch,
            }),
        };
        instructions.push(instruction);
    }
    
    if let Some(pos) = loop_stack.last() {
        return Err(ParseError::UnmatchedLoop { location: *pos });
    }
    
    Ok(Program { instructions })
}

impl Program {
    pub fn execute(&self, input: Vec<u8>, mut memory: Vec<u8>) -> Result<String, ExecuteError> {
        let mut output = Vec::new();
        let mut data_pointer = 0;
        let mut input_pointer = 0;
        let mut instruction_pointer = 0;
        let mut instruction_count = 0;
        
        while instruction_pointer < self.instructions.len() {
            instruction_count += 1;
            if instruction_count > 10000 {
                return Err(ExecuteError::InfiniteLoop);
            }
            
            match self.instructions[instruction_pointer] {
                Instruction::IncrementPointer => {
                    data_pointer += 1;
                }
                Instruction::DecrementPointer => {
                    data_pointer -= 1;
                }
                Instruction::Increment => {
                    memory[data_pointer] = memory[data_pointer].wrapping_add(1);
                }
                Instruction::Decrement => {
                    memory[data_pointer] = memory[data_pointer].wrapping_sub(1);
                }
                Instruction::Output => {
                    output.push(memory[data_pointer]);
                }
                Instruction::Input => {
                    if input_pointer >= input.len() {
                        return Err(ExecuteError::NoInputLeft);
                    }
                    memory[data_pointer] = input[input_pointer];
                    input_pointer += 1;
                }
                Instruction::LoopStart => {
                    if memory[data_pointer] == 0 {
                        let mut depth = 1;
                        while depth > 0 {
                            instruction_pointer += 1;
                            if instruction_pointer >= self.instructions.len() {
                                return Err(ExecuteError::InfiniteLoop);
                            }
                            match self.instructions[instruction_pointer] {
                                Instruction::LoopStart => depth += 1,
                                Instruction::LoopEnd => depth -= 1,
                                _ => {}
                            }
                        }
                    }
                }
                Instruction::LoopEnd => {
                    if memory[data_pointer] != 0 {
                        let mut depth = 1;
                        while depth > 0 {
                            instruction_pointer -= 1;
                            match self.instructions[instruction_pointer] {
                                Instruction::LoopStart => depth -= 1,
                                Instruction::LoopEnd => depth += 1,
                                _ => {}
                            }
                        }
                        continue;
                    }
                }
            }
            instruction_pointer += 1;
        }
        
        Ok(String::from_utf8_lossy(&output).into_owned())
    }
}
// Run this file with `cargo test --test 06_brainfuck_interpreter`.

// TODO (bonus): Create an interpreter for the [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) language.
// The Brainfuck program will be parsed out of a string and represented as a struct.
//
// Handle both parsing and execution errors using enums representing error conditions,
// see tests for details.
// A parsing error can be either an unknown instruction or an unpaired loop instruction.
// An execution error can be either that the program tries to read input, but there is no more
// input available, or when the program executes more than 10000 instructions (which probably
// signals an infinite loop).
//
// Hint: Put `#[derive(Debug, Eq, PartialEq)]` on top of `ParseError`, `ExecuteError` and `Program`
// (and any other custom types nested inside them) so that asserts in tests work.



/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::{parse_program, ExecuteError, ParseError};

    #[test]
    fn parse_empty() {
        check_output("", "", "");
    }

    #[test]
    fn parse_unknown_instruction() {
        assert!(matches!(
            parse_program(">p"),
            Err(ParseError::UnknownInstruction {
                location: 1,
                instruction: 'p'
            })
        ));
    }

    #[test]
    fn parse_unmatched_loop_start() {
        assert_eq!(
            parse_program(">++[+>][++>"),
            Err(ParseError::UnmatchedLoop { location: 7 })
        );
    }

    #[test]
    fn parse_unmatched_loop_end() {
        assert_eq!(
            parse_program(">++[+>][++>]+]"),
            Err(ParseError::UnmatchedLoop { location: 13 })
        );
    }

    #[test]
    fn missing_input() {
        let program = parse_program(",").unwrap();
        let result = program.execute(vec![], vec![0; 30000]);
        assert_eq!(result, Err(ExecuteError::NoInputLeft));
    }

    #[test]
    fn infinite_loop() {
        let program = parse_program("+[]").unwrap();
        let result = program.execute(vec![], vec![0; 30000]);
        assert_eq!(result, Err(ExecuteError::InfiniteLoop));
    }

    #[test]
    fn copy_input() {
        check_output(",.>,.>,.>,.>,.", "hello", "hello");
    }

    #[test]
    fn output_exclamation_mark() {
        check_output("+++++++++++++++++++++++++++++++++.", "", "!");
    }

    #[test]
    fn three_exclamation_marks() {
        check_output(">+++++++++++++++++++++++++++++++++<+++[>.<-]", "", "!!!");
    }

    #[test]
    fn hello_world() {
        check_output("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.", "", "Hello World!\n");
    }

    fn check_output(program_text: &str, input: &str, expected_output: &str) {
        let program = parse_program(program_text);
        match program {
            Ok(program) => {
                let result = program
                    .execute(input.to_string().into_bytes(), vec![0; 30000])
                    .expect(&format!("Cannot execute program {program_text}"));
                assert_eq!(result, expected_output);
            }
            Err(error) => {
                panic!("Error occurred while parsing program {program_text}: {error:?}");
            }
        }
    }
}