use std::{collections::LinkedList, io};

use thiserror::Error;

const MEMORY_SIZE: usize = 30_000;

#[derive(PartialEq, Eq, Debug)]
enum Token {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    LoopBegin,
    LoopEnd,
    Unknown,
}

#[derive(PartialEq, Eq, Debug)]
enum Operation {
    MoveRight(usize),
    MoveLeft(usize),
    Increment(u8),
    Decrement(u8),
    Output,
    Input,
    Loop(Vec<Operation>),
}

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("Error parsing source: `{0}`")]
    ParseError(String),
    #[error("Memory overflow")]
    MemoryOverflow,
    #[error("Pointer is out of memory bounds")]
    PointerOverflow,
    #[error("Error reading from stdin: `{0}`")]
    StdinError(io::Error),
}

struct Program {
    memory: [u8; MEMORY_SIZE],
    pointer: usize,
    stdin: Box<dyn io::Read>,
    stdout: String,
}

impl Program {
    fn new(stdin: Box<dyn io::Read>) -> Self {
        Self {
            memory: [0u8; MEMORY_SIZE],
            pointer: 0,
            stdin,
            stdout: String::new(),
        }
    }

    fn execute(mut self, operations: &[Operation]) -> Result<String, InterpreterError> {
        self.process_operations(operations)?;
        Ok(self.stdout)
    }

    fn process_operations(&mut self, operations: &[Operation]) -> Result<(), InterpreterError> {
        for operation in operations {
            match operation {
                Operation::MoveLeft(count) => {
                    self.pointer = self
                        .pointer
                        .checked_sub(*count)
                        .ok_or(InterpreterError::PointerOverflow)?;
                }
                Operation::MoveRight(count) => {
                    self.pointer = self
                        .pointer
                        .checked_add(*count)
                        .ok_or(InterpreterError::PointerOverflow)?;
                    if self.pointer >= self.memory.len() {
                        return Err(InterpreterError::PointerOverflow);
                    }
                }
                Operation::Increment(count) => {
                    self.memory[self.pointer] = self.memory[self.pointer]
                        .checked_add(*count)
                        .ok_or(InterpreterError::MemoryOverflow)?;
                }
                Operation::Decrement(count) => {
                    self.memory[self.pointer] = self.memory[self.pointer]
                        .checked_sub(*count)
                        .ok_or(InterpreterError::MemoryOverflow)?
                }
                Operation::Input => {
                    let mut buf = [0u8];
                    if let Err(err) = self.stdin.read(&mut buf) {
                        return Err(InterpreterError::StdinError(err));
                    }
                    self.memory[self.pointer] = buf[0] as u8;
                }
                Operation::Output => self.stdout.push(self.memory[self.pointer] as char),
                Operation::Loop(operations) => {
                    while self.memory[self.pointer] != 0 {
                        self.process_operations(operations)?;
                    }
                }
            }
        }

        Ok(())
    }
}

fn parse_source(source: &str) -> Result<Vec<Operation>, InterpreterError> {
    let tokens = source
        .chars()
        .map(|cur| match cur {
            '>' => Token::MoveRight,
            '<' => Token::MoveLeft,
            '+' => Token::Increment,
            '-' => Token::Decrement,
            '.' => Token::Output,
            ',' => Token::Input,
            '[' => Token::LoopBegin,
            ']' => Token::LoopEnd,
            _ => Token::Unknown,
        })
        .filter(|token| token.ne(&Token::Unknown));

    let mut stack: LinkedList<Vec<Operation>> = LinkedList::new();
    stack.push_back(Vec::new());

    for token in tokens {
        let cur_operations = stack.back_mut().expect("Stack should not be empty!");
        match token {
            Token::MoveRight => {
                if let Some(Operation::MoveRight(x)) = cur_operations.last_mut() {
                    *x += 1;
                } else {
                    cur_operations.push(Operation::MoveRight(1))
                }
            }
            Token::MoveLeft => {
                if let Some(Operation::MoveLeft(x)) = cur_operations.last_mut() {
                    *x += 1;
                } else {
                    cur_operations.push(Operation::MoveLeft(1))
                }
            }
            Token::Increment => {
                if let Some(Operation::Increment(x)) = cur_operations.last_mut() {
                    *x += 1;
                } else {
                    cur_operations.push(Operation::Increment(1))
                }
            }
            Token::Decrement => {
                if let Some(Operation::Decrement(x)) = cur_operations.last_mut() {
                    *x += 1;
                } else {
                    cur_operations.push(Operation::Decrement(1))
                }
            }
            Token::Input => cur_operations.push(Operation::Input),
            Token::Output => cur_operations.push(Operation::Output),
            Token::LoopBegin => stack.push_back(Vec::new()),
            Token::LoopEnd => {
                let cur_operations = stack.pop_back().unwrap();
                let prev_operations = stack.back_mut().ok_or_else(|| {
                    InterpreterError::ParseError(String::from("Unexpected end of loop"))
                })?;

                prev_operations.push(Operation::Loop(cur_operations))
            }
            _ => {
                return Err(InterpreterError::ParseError(format!(
                    "Unexpected token {:?}",
                    token
                )))
            }
        }
    }

    let operations = stack.pop_back().unwrap();
    if !stack.is_empty() {
        Err(InterpreterError::ParseError(String::from(
            "Expected end of loop",
        )))
    } else {
        Ok(operations)
    }
}

pub fn interpret(source: &str, stdin: Box<dyn io::Read>) -> Result<String, InterpreterError> {
    let operations = parse_source(source)?;
    let program = Program::new(stdin);
    program.execute(&operations)
}

#[cfg(test)]
mod test {
    use crate::{interpret, parse_source, InterpreterError, Operation};

    #[test]
    fn parse_cat() {
        let source = ",[.,]";
        let expected = vec![
            Operation::Input,
            Operation::Loop(vec![Operation::Output, Operation::Input]),
        ];

        let actual = parse_source(source).expect("It works");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_cat_missing_end_of_loop() {
        let source = ",[.,";

        let actual = parse_source(source);
        assert!(matches!(actual, Err(InterpreterError::ParseError(_))))
    }

    #[test]
    fn parse_cat_redundat_end_of_loop() {
        let source = ",[.,]]";

        let actual = parse_source(source);
        assert!(matches!(actual, Err(InterpreterError::ParseError(_))));
    }

    #[test]
    fn hello_world() {
        let source = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let input = "".as_bytes();
        let expected = String::from("Hello World!\n");

        let actual = interpret(source, Box::new(input)).expect("It works");
        assert_eq!(expected, actual);
    }

    #[test]
    fn cat() {
        let source = ",[.,]";
        let input = "I love programming!".as_bytes();
        let expected = String::from("I love programming!");

        let actual = interpret(source, Box::new(input)).expect("It works");
        assert_eq!(expected, actual);
    }

    #[test]
    fn fibonacci() {
        let source = "+++++++++++
        >+>>>>++++++++++++++++++++++++++++++++++++++++++++
        >++++++++++++++++++++++++++++++++<<<<<<[>[>>>>>>+>
        +<<<<<<<-]>>>>>>>[<<<<<<<+>>>>>>>-]<[>++++++++++[-
        <-[>>+>+<<<-]>>>[<<<+>>>-]+<[>[-]<[-]]>[<<[>>>+<<<
        -]>>[-]]<<]>>>[>>+>+<<<-]>>>[<<<+>>>-]+<[>[-]<[-]]
        >[<<+>>[-]]<<<<<<<]>>>>>[+++++++++++++++++++++++++
        +++++++++++++++++++++++.[-]]++++++++++<[->-<]>++++
        ++++++++++++++++++++++++++++++++++++++++++++.[-]<<
        <<<<<<<<<<[>>>+>+<<<<-]>>>>[<<<<+>>>>-]<-[>>.>.<<<
        [-]]<<[>>+>+<<<-]>>>[<<<+>>>-]<<[<+>-]>[<+>-]<<<-]";
        let input = "".as_bytes();
        let expected = String::from("1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89");

        let actual = interpret(source, Box::new(input)).expect("It works");
        assert_eq!(expected, actual);
    }

    #[test]
    fn catch_pointer_overflow_left() {
        let source = ">><<<";
        let input = "".as_bytes();

        let actual = interpret(source, Box::new(input));
        assert!(matches!(actual, Err(InterpreterError::PointerOverflow)));
    }

    #[test]
    fn catch_pointer_overflow_right() {
        let source = "+[>+]";
        let input = "".as_bytes();

        let actual = interpret(source, Box::new(input));
        assert!(matches!(actual, Err(InterpreterError::PointerOverflow)));
    }

    #[test]
    fn catch_memory_overflow_left() {
        let source = "+--";
        let input = "".as_bytes();

        let actual = interpret(source, Box::new(input));
        assert!(matches!(actual, Err(InterpreterError::MemoryOverflow)));
    }

    #[test]
    fn catch_memory_overflow_right() {
        let source = "+[+]";
        let input = "".as_bytes();

        let actual = interpret(source, Box::new(input));
        assert!(matches!(actual, Err(InterpreterError::MemoryOverflow)));
    }
}
