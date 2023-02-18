fn evalbf(code: &str) -> &str {
    ""
}

fn main() {

}

mod test {
    use super::*;

    #[test]
    fn test_evalbf() {
        let helloworld = "++++++++++
            [
                >+++++++
                >++++++++++
                >+++
                >+<<<<-
            ]   >++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.
            --------.>+.>.";
    
        let output = evalbf(helloworld);
        println!("> {}", output);
        assert_eq!("Hello World!", output);
    }
}