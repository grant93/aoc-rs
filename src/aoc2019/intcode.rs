use std::io;

#[derive(FromPrimitive)]
enum OpCode {
    Addition = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    AdjustBase = 9,
    Halt = 99,
}

#[derive(Debug)]
pub struct VirtualMachine {
    memory: Vec<i64>,
    ip: usize,
    base: i64,
}

impl VirtualMachine {
    pub fn new(mut mem: Vec<i64>) -> Self {
        let mut extra: Vec<i64> = vec![0; 2000];
        mem.append(&mut extra);
        VirtualMachine {
            memory: mem,
            ip: 0,
            base: 0,
        }
    }

    pub fn run(&mut self, input: &mut Vec<i64>, output: &mut dyn io::Write) -> i64 {
        loop {
            let instr = self.memory[self.ip] % 100;
            match num::FromPrimitive::from_i64(instr) {
                Some(OpCode::Addition) => self.add(),
                Some(OpCode::Multiply) => self.multiply(),
                Some(OpCode::Input) => self.input(input),
                Some(OpCode::Output) => self.output(output),
                Some(OpCode::JumpIfTrue) => self.jump_if_true(),
                Some(OpCode::JumpIfFalse) => self.jump_if_false(),
                Some(OpCode::LessThan) => self.less_than(),
                Some(OpCode::Equals) => self.equals(),
                Some(OpCode::AdjustBase) => self.adjustbase(),
                Some(OpCode::Halt) => return self.halt(),
                _ => panic!("AHHH"),
            };
        }
    }

    fn parse_arg(&self, pos: usize) -> i64 {
        let mode = (self.memory[self.ip] / 10i64.pow((1 + pos).try_into().unwrap())) % 10;
        let arg = self.memory[self.ip + pos];
        match mode {
            0 => self.memory[arg as usize],
            1 => arg,
            2 => self.memory[(self.base + arg) as usize],
            _ => panic!("invalid mode detected"),
        }
    }

    fn write_result(&mut self, pos: usize, val: i64) {
        let mode = (self.memory[self.ip] / 10i64.pow((1 + pos).try_into().unwrap())) % 10;
        let arg = self.memory[self.ip + pos];
        match mode {
            0 => self.memory[arg as usize] = val,
            2 => self.memory[(self.base + arg) as usize] = val,
            _ => panic!("invalid mode detected"),
        };
    }

    fn add(&mut self) {
        self.write_result(3, self.parse_arg(1) + self.parse_arg(2));
        self.ip += 4;
    }

    fn multiply(&mut self) {
        self.write_result(3, self.parse_arg(1) * self.parse_arg(2));
        self.ip += 4;
    }

    fn input(&mut self, input: &mut Vec<i64>) {
        self.write_result(1, input[0]);
        self.ip += 2;
    }

    fn jump_if_true(&mut self) {
        if self.parse_arg(1) != 0 {
            self.ip = self.parse_arg(2) as usize;
        } else {
            self.ip += 3;
        }
    }

    fn jump_if_false(&mut self) {
        if self.parse_arg(1) == 0 {
            self.ip = self.parse_arg(2) as usize;
        } else {
            self.ip += 3;
        }
    }

    fn less_than(&mut self) {
        if self.parse_arg(1) < self.parse_arg(2) {
            self.write_result(3, 1);
        } else {
            self.write_result(3, 0);
        }
        self.ip += 4;
    }

    fn equals(&mut self) {
        if self.parse_arg(1) == self.parse_arg(2) {
            self.write_result(3, 1);
        } else {
            self.write_result(3, 0);
        }
        self.ip += 4;
    }

    fn adjustbase(&mut self) {
        self.base += self.parse_arg(1);
        self.ip += 2;
    }

    fn output(&mut self, output: &mut dyn io::Write) {
        writeln!(output, "{}", self.parse_arg(1)).unwrap();
        self.ip += 2;
    }

    fn halt(&self) -> i64 {
        self.memory[0]
    }
}
