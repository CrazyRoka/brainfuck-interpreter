use std::collections::LinkedList;

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
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    Loop(Vec<Operation>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum InterpreterError {
    ParseError(String),
}

struct Program {
    memory: [u8; MEMORY_SIZE],
    pointer: usize,
    stdin: LinkedList<char>,
    stdout: String,
}

impl Program {
    fn new(stdin: LinkedList<char>) -> Self {
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
                Operation::MoveLeft => self.pointer -= 1,
                Operation::MoveRight => self.pointer += 1,
                Operation::Increment => self.memory[self.pointer] += 1,
                Operation::Decrement => self.memory[self.pointer] -= 1,
                Operation::Input => {
                    let input = self.stdin.pop_front().unwrap_or(0 as char);
                    self.memory[self.pointer] = input as u8;
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
        match token {
            Token::MoveRight => stack.back_mut().unwrap().push(Operation::MoveRight),
            Token::MoveLeft => stack.back_mut().unwrap().push(Operation::MoveLeft),
            Token::Increment => stack.back_mut().unwrap().push(Operation::Increment),
            Token::Decrement => stack.back_mut().unwrap().push(Operation::Decrement),
            Token::Input => stack.back_mut().unwrap().push(Operation::Input),
            Token::Output => stack.back_mut().unwrap().push(Operation::Output),
            Token::LoopBegin => stack.push_back(Vec::new()),
            Token::LoopEnd => {
                let operations = stack.pop_back().unwrap();
                if stack.is_empty() {
                    return Err(InterpreterError::ParseError(String::from(
                        "Unexpected end of loop",
                    )));
                }

                stack.back_mut().unwrap().push(Operation::Loop(operations))
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

pub fn interpret(source: &str, input: &str) -> Result<String, InterpreterError> {
    let operations = parse_source(source)?;
    let input = input.chars().collect();
    let program = Program::new(input);
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

        let actual = parse_source(source);
        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn parse_cat_missing_end_of_loop() {
        let source = ",[.,";
        let expected = Err(InterpreterError::ParseError(String::from(
            "Expected end of loop",
        )));

        let actual = parse_source(source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_cat_redundat_end_of_loop() {
        let source = ",[.,]]";
        let expected = Err(InterpreterError::ParseError(String::from(
            "Unexpected end of loop",
        )));

        let actual = parse_source(source);
        assert_eq!(expected, actual);
    }

    #[test]
    fn hello_world() {
        let source = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let input = "";
        let expected = String::from("Hello World!\n");

        let actual = interpret(source, input);
        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn cat() {
        let source = ",[.,]";
        let input = "I love programming!";
        let expected = String::from(input);

        let actual = interpret(source, input);
        assert_eq!(Ok(expected), actual);
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
        let input = "";
        let expected = String::from("1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89");

        let actual = interpret(source, input);
        assert_eq!(Ok(expected), actual);
    }
}