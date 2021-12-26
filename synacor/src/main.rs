use std::fs;
use std::convert::TryFrom;
use std::io::Read;

struct Program {
    memory: Vec<u16>,
    ip: usize,
    regs: Vec<u16>,
    stack: Vec<u16>,
    is_running: bool,
    debug: bool,
}

#[derive(Debug)]
enum Op {
    Halt = 0,
    Set,
    Push,
    Pop,
    Eq,
    Gt,
    Jmp,
    Jt,
    Jf,
    Add,
    Mult,
    Mod,
    And,
    Or,
    Not,
    Rmem,
    Wmem,
    Call,
    Ret,
    Out,
    In,
    NoOp = 21,
}

impl TryFrom<u16> for Op {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == Op::Halt as u16 => Ok(Op::Halt),
            x if x == Op::Set as u16 => Ok(Op::Set),
            x if x == Op::Push as u16 => Ok(Op::Push),
            x if x == Op::Pop as u16 => Ok(Op::Pop),
            x if x == Op::Eq as u16 => Ok(Op::Eq),
            x if x == Op::Gt as u16 => Ok(Op::Gt),
            x if x == Op::Jmp as u16 => Ok(Op::Jmp),
            x if x == Op::Jt as u16 => Ok(Op::Jt),
            x if x == Op::Jf as u16 => Ok(Op::Jf),
            x if x == Op::Add as u16 => Ok(Op::Add),
            x if x == Op::Mult as u16 => Ok(Op::Mult),
            x if x == Op::Mod as u16 => Ok(Op::Mod),
            x if x == Op::And as u16 => Ok(Op::And),
            x if x == Op::Or as u16 => Ok(Op::Or),
            x if x == Op::Not as u16 => Ok(Op::Not),
            x if x == Op::Rmem as u16 => Ok(Op::Rmem),
            x if x == Op::Wmem as u16 => Ok(Op::Wmem),
            x if x == Op::Call as u16 => Ok(Op::Call),
            x if x == Op::Ret as u16 => Ok(Op::Ret),
            x if x == Op::Out as u16 => Ok(Op::Out),
            x if x == Op::In as u16 => Ok(Op::In),
            x if x == Op::NoOp as u16 => Ok(Op::NoOp),
            _ => Err(()),
        }
    }
}

impl Program {
    fn load(&mut self, content: &Vec<u8>) -> usize {
        let mut idx = 0;

        if content.len() % 2 != 0 {
            panic!("Invalid content size");
        }

        while idx != content.len() {
            self.memory[idx / 2] = content[idx] as u16 + content[idx+1] as u16 * 256;
            idx += 2;
        }

        0
    }

    fn dump(&self) {
        for n in 0..256 {
            println!("{}", self.memory[n]);
        }
    }

    fn run(&mut self) {
        self.is_running = true;

        while self.is_running {
            self.step();
        }
    }

    fn step(&mut self) {
        let current_ip = self.ip;
        let current_op = self.memory[current_ip];

        self.ip += 1;

        if self.debug {
            print!("ip:{} ", current_ip);
        }

        match current_op.try_into() {
            Ok(Op::Halt) => {
                self.is_running = false;
            },
            Ok(Op::Set) => self.op_set(),
            Ok(Op::Push) => self.op_push(),
            Ok(Op::Pop) => self.op_pop(),
            Ok(Op::Eq) => self.op_eq(),
            Ok(Op::Gt) => self.op_gt(),
            Ok(Op::Jmp) => self.op_jmp(),
            Ok(Op::Jt) => self.op_jt(),
            Ok(Op::Jf) => self.op_jf(),
            Ok(Op::Add) => self.op_add(),
            Ok(Op::Mult) => self.op_mult(),
            Ok(Op::Mod) => self.op_mod(),
            Ok(Op::And) => self.op_and(),
            Ok(Op::Or) => self.op_or(),
            Ok(Op::Not) => self.op_not(),
            Ok(Op::Rmem) => self.op_rmem(),
            Ok(Op::Wmem) => self.op_wmem(),
            Ok(Op::Call) => self.op_call(),
            Ok(Op::Ret) => self.op_ret(),
            Ok(Op::In) => self.op_in(),
            Ok(Op::Out) => self.op_out(),
            Ok(Op::NoOp) => {},
            _ => {
                if self.debug {
                    println!("op code {:?} was not implemented.", current_op);
                }
                unimplemented!();
            }
        }
    }

    // number to machine value
    fn get_real_value(&self, v: u16) -> u16{
        match v {
            0..=32767 => v,
            32768..=32775 => self.regs[v as usize - 32768],
            32776..=65535 => panic!("Invalid value: {}", v),
        }
    }

    // number to register number
    fn get_next_register(&mut self) -> u16 {
        let v = self.memory[self.ip];
        self.ip += 1;

        match v {
            32768..=32775 => v - 32768,
            _ => panic!("Invalid register number: {}", v),
        }
    }

    // get next value & move ip
    fn get_next_value(&mut self) -> u16 {
        let raw_value = self.memory[self.ip];
        let v = self.get_real_value(raw_value);
        self.ip += 1;
        v
    }

    fn get_next_raw_value(&mut self) -> u16 {
        let raw_value = self.memory[self.ip];
        self.ip += 1;
        raw_value
    }

    // set register value
    fn set_register(&mut self, register: usize, value: u16) {
        self.regs[register] = value;
    }

    // set value
    fn set_value(&mut self, idx: usize, val: u16) {
        match idx {
            0..=32767 => self.memory[idx] = val,
            32768..=32775 => self.regs[idx - 32768] = val,
            _ => panic!("Invalid location: {}", idx),
        }
    }

    // operators
    fn op_set(&mut self) { // op code 1
        // set: 1 a b
        // set register <a> to the value of <b>
        let a = self.get_next_register();
        let b = self.get_next_value();

        if self.debug {
            println!("set r{} {}", a, b);
        }

        self.set_register(a as usize, b);
    }

    fn op_push(&mut self) {
        // push: 2 a
        // push <a> onto the stack
        let a = self.get_next_value();

        if self.debug {
            println!("push {}", a);
        }

        self.stack.push(a);
    }

    fn op_pop(&mut self) {
        // pop: 3 a
        // remove the top element from the stack and write it into <a>; empty stack = error

        let a = self.get_next_raw_value();

        if self.debug {
            println!("pop {}", a);
        }

        if let Some(val) = self.stack.pop() {
            self.set_value(a as usize, val)
        } else {
            panic!("Not enough element to pop from stack");
        }
    }

    fn op_eq(&mut self) {
        // eq: 4 a b c
        // set <a> to 1 if <b> is equal to <c>; set it to 0 otherwise
        let a = self.get_next_raw_value();
        let b = self.get_next_value();
        let c = self.get_next_value();

        if self.debug {
            println!("eq {} {} {}", a, b, c);
        }

        let val = if b == c {
            1
        } else {
            0
        };

        self.set_value(a as usize, val);
    }

    fn op_gt(&mut self) {
        // gt: 5 a b c
        // set <a> to 1 if <b> is greater than <c>; set it to 0 otherwise
        let a = self.get_next_raw_value();
        let b = self.get_next_value();
        let c = self.get_next_value();

        if self.debug {
            println!("gt {} {} {}", a, b, c);
        }

        let val = if b > c {
            1
        } else {
            0
        };

        self.set_value(a as usize, val);
    }

    fn op_jmp(&mut self) { // op code: 6
        let a = self.get_next_value();

        if self.debug {
            println!("jmp {}", a);
        }

        self.ip = a as usize;
    }

    fn op_jt(&mut self) { // op code: 7
        // jt: 7 a b
        // if <a> is nonzero, jump to <b>
        let a = self.get_next_value();
        let b = self.get_next_value();

        if self.debug {
            println!("jt {} {}", a, b);
        }

        if a != 0 {
            self.ip = b as usize;
        }
    }

    fn op_jf(&mut self) { // op code: 8
        // jf: 8 a b
        // if <a> is zero, jump to <b>
        let a = self.get_next_value();
        let b = self.get_next_value();

        if self.debug {
            println!("jf {} {}", a, b);
        }

        if a == 0 {
            self.ip = b as usize;
        }
    }

    fn op_add(&mut self) {
        // add: 9 a b c
        // assign into <a> the sum of <b> and <c> (modulo 32768)
        let a = self.get_next_raw_value();
        let b = self.get_next_value();
        let c = self.get_next_value();

        let val = ((b as u32 + c as u32) % 32768) as u16;

        if self.debug {
            println!("add {} {} {} (val: {})", a, b, c, val);
        }

        self.set_value(a as usize, val);
    }

    fn op_mult(&mut self) {
        // mult: 10 a b c
        // store into <a> the product of <b> and <c> (modulo 32768)
        let a = self.get_next_raw_value();
        let b = self.get_next_value();
        let c = self.get_next_value();

        let val = ((b as u32 * c as u32) % 32768) as u16;

        if self.debug {
            println!("mult {} {} {} (val: {})", a, b, c, val);
        }

        self.set_value(a as usize, val);
    }

    fn op_mod(&mut self) {
        // mod: 11 a b c
        // store into <a> the remainder of <b> divided by <c>
        let a = self.get_next_raw_value();
        let b = self.get_next_value();
        let c = self.get_next_value();

        let val = ((b as u32 % c as u32) % 32768) as u16;

        if self.debug {
            println!("mod {} {} {} (val: {})", a, b, c, val);
        }

        self.set_value(a as usize, val);
    }

    fn op_and(&mut self) {
        // and: 12 a b c
        // stores into <a> the bitwise and of <b> and <c>
        let a = self.get_next_raw_value();
        let b = self.get_next_value();
        let c = self.get_next_value();

        let val = b & c;

        if self.debug {
            println!("and {} {} {}", a, b, c);
        }

        self.set_value(a as usize, val);
    }

    fn op_or(&mut self) {
        // and: 12 a b c
        // stores into <a> the bitwise and of <b> and <c>
        let a = self.get_next_raw_value();
        let b = self.get_next_value();
        let c = self.get_next_value();

        let val = b | c;

        if self.debug {
            println!("or {} {} {}", a, b, c);
        }

        self.set_value(a as usize, val);
    }

    fn op_not(&mut self) {
        // not: 14 a b
        // stores 15-bit bitwise inverse of <b> in <a>
        let a = self.get_next_raw_value();
        let b = self.get_next_value();

        let val = !b & 0x7fff;

        if self.debug {
            println!("not {} {} (=> {})", a, b, val);
        }

        self.set_value(a as usize, val);
    }
    
    fn op_rmem(&mut self) {
        // rmem: 15 a b
        // read memory at address <b> and write it to <a>
        let a = self.get_next_raw_value();
        let b = self.get_next_value();

        if self.debug {
            println!("rmem {} {}", a, b);
        }

        self.set_value(a as usize, self.memory[b as usize]);
    }

    fn op_wmem(&mut self) {
        // wmem: 16 a b
        // write the value from <b> into memory at address <a>
        let a = self.get_next_value();
        let b = self.get_next_value();

        if self.debug {
            println!("wmem {} {}", a, b);
        }

        self.memory[a as usize] = b;
    }

    fn op_call(&mut self) {
        // call: 17 a
        // write the address of the next instruction to the stack and jump to <a>
        let a = self.get_next_value();

        if self.debug {
            println!("call {}", a);
        }

        self.stack.push(self.ip as u16);

        self.ip = a as usize;
    }

    fn op_ret(&mut self) {
        // ret: 18
        // remove the top element from the stack and jump to it; empty stack = halt

        if self.debug {
            println!("ret");
        }

        if let Some(val) = self.stack.pop() {
            self.ip = val as usize;
        } else {
            self.is_running = false;
        }
    }

    fn op_in(&mut self) {
        let a = self.get_next_raw_value();

        let mut buf = [0; 1];
        let mut res = std::io::stdin().read_exact(&mut buf);

        if self.debug {
            println!("in {}", a);
        }

        if buf[0] == 'X' as u8 {
            //self.debug = true;

            println!("Debug mode activated.");

            self.regs[7] = 25734;
            self.regs[0] = 6;

            res = std::io::stdin().read_exact(&mut buf);
            res = std::io::stdin().read_exact(&mut buf);
        }

        if res.is_err() {
            println!("Input error.");
            self.is_running = false;
        }

        self.set_value(a as usize, buf[0] as u16);
    }

    fn op_out(&mut self) { // op code: 19
        let a = self.get_next_value();
        let c = a as u8 as char;

        if self.debug {
            if c == '\n' {
                println!("print '\n'");
            } else {
                println!("print '{}'", c);
            }
        } else {
            print!("{}", c);
        }
    }
}


fn main() {
    let file_content = fs::read("challenge.bin").expect("file");

    let mut prog = Program {
        memory: vec![0; 0x7ffff],
        ip: 0,
        stack: vec![],
        regs: vec![0; 8],
        is_running: false,
        debug: false,
    };

    prog.load(&file_content);
    prog.run();
}
