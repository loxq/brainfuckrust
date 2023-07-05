# Brainfuck interpreter written in plain Rust

## Run tests

```
$ cargo test

   Compiling brainfuckrust v0.1.0 (/brainfuckrust)
    Finished test [unoptimized + debuginfo] target(s) in 0.29s
     Running unittests src/main.rs (target/debug/deps/brainfuckrust-51b6ec4abbc88f66)

running 3 tests
test test::test_lex ... ok
test test::test_parsebf ... ok
test test::test_runbf ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Debug

```
$ cargo run -- code.bf
```

## Make and run the release version

```
cargo build --release
./target/release/brainfuckrust helloworld.bf
```

## Output Example

```
$ cat helloworld.bf
    ++++++++++
    [
        >+++++++
        >++++++++++
        >+++
        >+<<<<-
    ]   >++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.
    --------.>+.>.
```

```
$ brainfuckrust helloworld.bf

Hello World!

```
