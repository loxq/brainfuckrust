use std::io::Read;
use std::fs::File;
use std::env;

const TAPESIZE: usize = 1024;
const MAX_RECURSION: i32 = 10;

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
// Direct operators conversions
enum OpCode {
    IncPtr,
    DecPtr,
    Inc,
    Dec,
    Write,
    Read,
    LoopStart,
    LoopStop
}

#[derive(PartialEq)]
#[derive(Debug)]
// "High-level" instructions
enum Instruction {
    IncPtr,
    DecPtr,
    Inc,
    Dec,
    Write,
    Read,
    Loop(Vec<Instruction>)
}

fn lex(code: &str) -> Vec<OpCode> {
    /*
        Parse source code and turn it into proper opcodes. Any invalid character
        is just ignored
     */
    let mut operations = Vec::new();
    
    for symbol in code.chars() {
        let operation = match symbol {
            '>' => Some(OpCode::IncPtr),
            '<' => Some(OpCode::DecPtr),
            '+' => Some(OpCode::Inc),
            '-' => Some(OpCode::Dec),
            '.' => Some(OpCode::Write),
            ',' => Some(OpCode::Read),
            '[' => Some(OpCode::LoopStart),
            ']' => Some(OpCode::LoopStop),
            _ => None
        };

        match operation {
            Some(operation) => operations.push(operation),
            None => ()
        }
    }

    operations
}

fn parsebf(operations: Vec<OpCode>) -> Vec<Instruction> {
    /*
        Parse opcodes and build instructions list, including recursive loops
     */
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_start: usize = 0;
    let mut loop_stack: i32 = 0;

    for (i, operation) in operations.iter().enumerate() {
        if loop_stack == 0 {
            let instruct = match operation {
                OpCode::IncPtr => Some(Instruction::IncPtr),
                OpCode::DecPtr => Some(Instruction::DecPtr),
                OpCode::Inc => Some(Instruction::Inc),
                OpCode::Dec => Some(Instruction::Dec),
                OpCode::Write => Some(Instruction::Write),
                OpCode::Read => Some(Instruction::Read),
                OpCode::LoopStart => { // track loop starting point
                    loop_start = i;
                    loop_stack += 1;
                    None
                },
                OpCode::LoopStop => panic!("Unbalanced loop end at char #{}", i),
            };
            match instruct {
                Some(instruct) => program.push(instruct),
                None => ()
            }
        } else if loop_stack < MAX_RECURSION {
            match operation {
                OpCode::LoopStart => {
                    loop_stack += 1;
                },
                OpCode::LoopStop => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        // Recursive loop parsing
                        program.push(Instruction::Loop(parsebf(operations[loop_start+1..i].to_vec())));
                    }
                },
                _ => (),
            }
        } else {
            panic!("Reached the recursion limit while parsing source code");
        }
    }
    if loop_stack != 0 {
        panic!("Unbalanced loop start at char #{}", loop_start);
    }

    program
}

fn runbf(instructions: &Vec<Instruction>, tape: &mut Vec<u8>, data_pointer: &mut usize) -> String{
    /*
        Run arbitrary brainfuck program.
        Output characters are written to output.
        tape is data mempory.
     */
    let mut output = String::new();
    for instr in instructions {
        match instr {
            Instruction::IncPtr => { 
                *data_pointer += 1; 
                if *data_pointer >= TAPESIZE {
                    panic!("Tape overflow error !");
                }
            },
            Instruction::DecPtr => { 
                *data_pointer -= 1;
                if *data_pointer >= TAPESIZE {  // if underflow it wraps to max value
                    panic!("Tape overflow error !");
                }
            },
            Instruction::Inc => tape[*data_pointer] += 1,
            Instruction::Dec => tape[*data_pointer] -= 1,
            Instruction::Write => {
                output.push(tape[*data_pointer] as char);   // append u8 ascii code to output buffer string
                print!("{}", tape[*data_pointer] as char)
            },
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).expect("Failed to read stdin");
                tape[*data_pointer] = input[0];
            },
            Instruction::Loop(nested_instructions) => {
                while tape[*data_pointer] != 0 {
                    let inner_output = runbf(&nested_instructions, tape, data_pointer);
                    output.push_str(&inner_output);
                }
            }
        }
    }
    output
}

fn read_source(filename: &String) -> String {
    let mut file = File::open(filename).expect("Source file not found");
    let mut source = String::new();
    file.read_to_string(&mut source).expect("Unable to read source file");

    source
}

fn main() {
    let mut tape: Vec<u8> = vec![0; TAPESIZE];
    let mut data_pointer = TAPESIZE / 2; // start in the middle of the tape

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Missing source file");
        println!("Usage: brainfuckrust file.bf");
    }
    let filename = &args[1];

    let source = &read_source(filename);

    let opcodes = lex(source);
    let instructions = parsebf(opcodes);
    runbf(&instructions, &mut tape, &mut data_pointer);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lex() {
        let payload = ".-+[]> <#,";  // all invalid characters (space and '#') should be ignored
    
        let output = lex(payload);
        let canary = &Vec::from([
            OpCode::Write,
            OpCode::Dec,
            OpCode::Inc,
            OpCode::LoopStart,
            OpCode::LoopStop,
            OpCode::IncPtr,
            OpCode::DecPtr,
            OpCode::Read
        ]);

        // Compare two Vec
        assert!(output.len() == canary.len() && output.iter().zip(canary).all(|(a, b)| *a == *b));
    }

    #[test]
    fn test_parsebf() {
        let payload: Vec<OpCode> = Vec::from([
            OpCode::IncPtr,
            OpCode::DecPtr,
            OpCode::Write,
            OpCode::Read,
            OpCode::Inc,
            OpCode::Dec,
            OpCode::LoopStart,
            OpCode::Inc,
            OpCode::Dec,
            OpCode::LoopStop
        ]);
        let canary: &Vec<Instruction> = &Vec::from([
            Instruction::IncPtr,
            Instruction::DecPtr,
            Instruction::Write,
            Instruction::Read,
            Instruction::Inc,
            Instruction::Dec,
            Instruction::Loop(Vec::from([Instruction::Inc, Instruction::Dec]))
        ]);
        let output = parsebf(payload);

        // Compare two Vec
        assert!(output.len() == canary.len() && output.iter().zip(canary).all(|(a, b)| a == b));
    }

    #[test]
    fn test_runbf() {
        let helloworld = "++++++++++
            [
                >+++++++
                >++++++++++
                >+++
                >+<<<<-
            ]   >++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.
            --------.>+.>.";
    
        let mut tape: Vec<u8> = vec![0; 1024];
        let mut data_pointer = 512;  // start at the middle of tape
        
        let opcodes = lex(helloworld);
        let instructions = parsebf(opcodes);

        let output = runbf(&instructions, &mut tape, &mut data_pointer);

        assert_eq!("Hello World!\n", output);
    }
}