#[derive(PartialEq)]
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

fn parsebf(code: &str) -> Vec<OpCode> {
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

fn runbf(code: &str) -> &str {
    
    ""
}

fn main() {

}

mod test {
    use super::*;

    #[test]
    fn test_parsebf() {
        let payload = ".-+[]> <#,";  // all invalid characters (space and '#') should be ignored
    
        let output = parsebf(payload);
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

        assert!(output.len() == canary.len() && output.iter().zip(canary).all(|(a, b)| *a == *b))
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