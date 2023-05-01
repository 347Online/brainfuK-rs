use std::io::Read;

const TAPE_LENGTH: usize = 30;

pub struct Brainfuck {
    depth: usize,

    tape: [u8; TAPE_LENGTH],
    index: usize,

    program: Vec<char>,
    pc: usize,
}

impl Brainfuck {
    pub fn new() -> Self {
        Brainfuck {
            depth: 0,

            tape: [0; TAPE_LENGTH],
            index: 0,

            program: vec![],
            pc: 0,
        }
    }

    pub fn exec(&mut self, source: &str) {
        let chars: Vec<char> = source.chars().collect();
        self.program = chars.clone();
        let len = chars.len();

        self.pc = 0;
        while self.pc < len {
            let c = chars[self.pc];
            self.pc += 1;

            let width = self.depth;
            print!("{:width$}", "");
            match c {
                '+' => self.increment(),
                '-' => self.decrement(),

                '>' => self.shift_right(),
                '<' => self.shift_left(),

                '.' => self.write(),
                ',' => self.read(),

                '[' => self.loop_open(),
                ']' => self.loop_close(),

                _ => ()
            }
        }
    }

    fn increment(&mut self) {
        #[cfg(debug_assertions)]
        println!("increment");

        if self.tape[self.index] == u8::MAX {
            self.tape[self.index] = 0;
        } else {
            self.tape[self.index] += 1;
        }
    }

    fn decrement(&mut self) {
        #[cfg(debug_assertions)]
        println!("decrement");

        if self.tape[self.index] == 0 {
            self.tape[self.index] = u8::MAX;
        } else {
            self.tape[self.index] -= 1;
        }
    }

    fn shift_right(&mut self) {
        #[cfg(debug_assertions)]
        println!("shift right");

        if self.index == TAPE_LENGTH {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }

    fn shift_left(&mut self) {
        #[cfg(debug_assertions)]
        println!("shift left");

        if self.index == 0 {
            self.index = TAPE_LENGTH - 1;
        } else {
            self.index -= 1;
        }
    }

    fn read(&mut self) {
        #[cfg(debug_assertions)]
        println!("read");

        let mut stdin = std::io::stdin();
        let mut buf = [0; 1];
        stdin.read_exact(&mut buf).expect("Could not read byte from stdin");
        self.tape[self.index] = buf[0];
    }

    fn write(&self) {
        #[cfg(debug_assertions)]
        println!("write");

        let byte = self.tape[self.index];
        let c = char::from(byte);
        print!("{}", c);

    }

    fn loop_open(&mut self) {
        #[cfg(debug_assertions)]
        println!("loop open");

        if self.tape[self.index] != 0 {
            return;
        }

        let start_depth = self.depth;
        loop {
            if self.program[self.pc] == '[' {
                self.depth += 1;
            }
            if self.program[self.pc] == ']' {
                self.depth -= 1;
                if self.depth == start_depth {
                    break;
                }
            }
            self.pc += 1;
        }
    }

    fn loop_close(&mut self) {
        #[cfg(debug_assertions)]
        println!("loop close");

        if self.tape[self.index] == 0 {
            return;
        }

        let start_depth = self.depth;
        loop {
            self.pc -= 1;
            if self.program[self.pc] == ']' {
                self.depth += 1;
            }
            if self.program[self.pc] == '[' {
                self.depth -= 1;
                if self.depth == start_depth {
                    self.pc += 1;
                    break;
                }
            }
        }
    }
}

impl Default for Brainfuck {
    fn default() -> Self {
        Self::new()
    }
}