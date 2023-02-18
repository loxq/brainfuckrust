use std::io::Read;

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
        } else {
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
        }
    }
    if loop_stack != 0 {
        panic!("Unbalanced loop start at char #{}", loop_start);
    }

    program
}

fn runbf(instructions: &Vec<Instruction>, tape: &mut Vec<u8>, data_pointer: &mut usize, output_buf: &mut String) {
    /*
        Run arbitrary brainfuck program.
        Output is written to &output_buf String.
        tape is data mempory.
     */
    for instr in instructions {
        match instr {
            Instruction::IncPtr => *data_pointer += 1,
            Instruction::DecPtr => *data_pointer -= 1,
            Instruction::Inc => tape[*data_pointer] += 1,
            Instruction::Dec => tape[*data_pointer] -= 1,
            
            // Uncomment this for direct print to stdout
            //Instruction::Write => print!("{}", tape[*data_pointer] as char),

            Instruction::Write => output_buf.push(tape[*data_pointer] as char), // append u8 ascii code to output buffer string
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).expect("Failed to read stdin");
                tape[*data_pointer] = input[0];
            },
            Instruction::Loop(nested_instructions) => {
                while tape[*data_pointer] != 0 {
                    runbf(&nested_instructions, tape, data_pointer, output_buf)
                }
            }
        }
    }
}

fn main() {

}

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
    
        let mut output_buf = String::new();
        let mut tape: Vec<u8> = vec![0; 1024];
        let mut data_pointer = 512;  // start at the middle of tape
        
        let opcodes = lex(helloworld);
        let instructions = parsebf(opcodes);

        runbf(&instructions, &mut tape, &mut data_pointer, &mut output_buf);

        assert_eq!("Hello World!\n", output_buf);
    }
}