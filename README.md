# brainfuK-rs
Another Brainfuck Interpreter, written in Rust

## Usage

You can execute a Brainfuck source file by passing the path as the first argument, e.g.
```bash
brainfuck hello.bf
```
Output:
```
Hello, World!
```

You can also execute a Brainfuck script directly with the `-e` flag, e.g.
```bash
brainfuck -e ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+."
```
Output:
```
Hello, World!
```