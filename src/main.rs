#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
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
                OpCode::LoopStart => {
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

fn runbf(code: &str) -> &str {
    
    ""
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

        assert!(output.len() == canary.len() && output.iter().zip(canary).all(|(a, b)| a == b));
    }

    //#[test]
    fn test_runbf() {
        let helloworld = "++++++++++
            [
                >+++++++
                >++++++++++
                >+++
                >+<<<<-
            ]   >++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.
            --------.>+.>.";
    
        let output = runbf(helloworld);
        println!("> {}", output);
        assert_eq!("Hello World!", output);
    }
}