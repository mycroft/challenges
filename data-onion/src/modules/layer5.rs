use crate::modules::ascii85::decode_ascii85;

struct Machine {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    la: u32,
    lb: u32,
    lc: u32,
    ld: u32,
    ptr: u32,
    pc: usize,
    memory: Vec<u8>,
}

pub fn decode_layer5(orig: &str) -> String {
    let decoded = decode_ascii85(orig);
    let debug = false;
    let mut res = vec![];

    let mut machine = Machine {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: 0,
        la: 0,
        lb: 0,
        lc: 0,
        ld: 0,
        ptr: 0,
        pc: 0,
        memory: decoded,
    };

    loop {
        if machine.pc >= machine.memory.len() {
            break;
        }

        let opcode = machine.memory[machine.pc];

        if debug {
            print!("PC: 0x{:04x} OP: 0x{:02x} - ", machine.pc, opcode);
        }

        machine.pc += 1;

        match opcode {
            0x01 => {
                // HALT
                if debug {
                    println!("HALT");
                }
                break;
            }
            0x02 => {
                // OUT a
                res.push(machine.a);

                if debug {
                    println!("OUT '{}'", machine.a as char);
                }
            }
            0x21 => {
                // JEZ imm32
                let val: u32 = machine.memory[machine.pc] as u32
                    | (machine.memory[machine.pc + 1] as u32) << 8
                    | (machine.memory[machine.pc + 2] as u32) << 16
                    | (machine.memory[machine.pc + 3] as u32) << 24;
                machine.pc += 4;

                if debug {
                    println!("JEZ 0x{:04x} (f:{})", val, machine.f);
                }

                if machine.f == 0 {
                    machine.pc = val as usize;
                }
            }
            0x22 => {
                // JNZ imm32
                let val: u32 = machine.memory[machine.pc] as u32
                    | (machine.memory[machine.pc + 1] as u32) << 8
                    | (machine.memory[machine.pc + 2] as u32) << 16
                    | (machine.memory[machine.pc + 3] as u32) << 24;
                machine.pc += 4;

                if debug {
                    println!("JNZ 0x{:04x} (f:{})", val, machine.f);
                }

                if machine.f != 0 {
                    machine.pc = val as usize;
                }
            }
            0xC1 => {
                // CMP
                if debug {
                    println!("CMP a:{} b:{}", machine.a, machine.b);
                }
                machine.f = if machine.a == machine.b { 0x00 } else { 0x01 }
            }
            0xC2 => {
                // ADD a <- b
                let z = (machine.a as i32 + machine.b as i32) % 256;

                if debug {
                    println!("ADD a:{} <- a:{} b:{}", z, machine.a, machine.b);
                }

                machine.a = z as u8;
            }
            0xC3 => {
                // SUB a <- b
                let z = if machine.b > machine.a {
                    (machine.a as u32 + 256 - machine.b as u32) as u8
                } else {
                    (machine.a - machine.b) as u8
                };

                if debug {
                    println!("SUB a:{} <- a:{} b:{}", z as u8, machine.a, machine.b);
                }

                machine.a = z as u8;
            }
            0xC4 => {
                // XOR a <- b
                if debug {
                    println!(
                        "XOR a:{} <- a:{} b:{}",
                        machine.a ^ machine.b,
                        machine.a,
                        machine.b
                    );
                }

                machine.a ^= machine.b;
            }
            0xE1 => {
                // APTR imm8
                let val = machine.memory[machine.pc];
                machine.pc += 1;

                if debug {
                    println!("APTR 0x{:02x}", val);
                }

                machine.ptr += val as u32;
            }
            opcode if opcode & 0b11000111 == 0b01000000 => {
                // MVI {dest} <- imm8
                let val = machine.memory[machine.pc];
                machine.pc += 1;

                let dst_bits = (opcode & 0b00111000) >> 3;
                if debug {
                    println!("MVI {} <- 0 (val:0x{:02x})", dst_bits, val,);
                }

                match dst_bits {
                    1 => machine.a = val,
                    2 => machine.b = val,
                    3 => machine.c = val,
                    4 => machine.d = val,
                    5 => machine.e = val,
                    6 => machine.f = val,
                    7 => machine.memory[(machine.ptr + machine.c as u32) as usize] = val,
                    _ => unreachable!(),
                };
            }
            opcode if opcode & 0b11000111 == 0b10000000 => {
                // MVI32 {dest} <- imm32
                let val: u32 = machine.memory[machine.pc] as u32
                    | (machine.memory[machine.pc + 1] as u32) << 8
                    | (machine.memory[machine.pc + 2] as u32) << 16
                    | (machine.memory[machine.pc + 3] as u32) << 24;
                machine.pc += 4;

                let dst_bits = (opcode & 0b00111000) >> 3;
                if debug {
                    println!("MVI32 {} <- 0 (val:0x{:08x})", dst_bits, val);
                }

                match dst_bits {
                    1 => machine.la = val,
                    2 => machine.lb = val,
                    3 => machine.lc = val,
                    4 => machine.ld = val,
                    5 => machine.ptr = val,
                    6 => machine.pc = val as usize,
                    _ => unreachable!(),
                };
            }
            opcode if opcode & 0b11000000 == 0b01000000 => {
                // MV {dest} <- {src}
                let src_bits = opcode & 0b00000111;
                let dst_bits = (opcode & 0b00111000) >> 3;

                let val = match src_bits {
                    1 => machine.a,
                    2 => machine.b,
                    3 => machine.c,
                    4 => machine.d,
                    5 => machine.e,
                    6 => machine.f,
                    7 => machine.memory[(machine.ptr + machine.c as u32) as usize],
                    _ => unreachable!(),
                };

                if debug {
                    println!("MV {} <- {} (val:0x{:02x})", dst_bits, src_bits, val);
                }

                match dst_bits {
                    1 => machine.a = val,
                    2 => machine.b = val,
                    3 => machine.c = val,
                    4 => machine.d = val,
                    5 => machine.e = val,
                    6 => machine.f = val,
                    7 => machine.memory[(machine.ptr + machine.c as u32) as usize] = val,
                    _ => unreachable!(),
                };
            }
            opcode if opcode & 0b11000000 == 0b10000000 => {
                // MV32 {dest} <- {src}
                let src_bits = opcode & 0b00000111;
                let dst_bits = (opcode & 0b00111000) >> 3;

                let val = match src_bits {
                    1 => machine.la,
                    2 => machine.lb,
                    3 => machine.lc,
                    4 => machine.ld,
                    5 => machine.ptr,
                    6 => machine.pc as u32,
                    _ => unreachable!(),
                };

                if debug {
                    println!("MV32 {} <- {} (val:0x{:08x})", dst_bits, src_bits, val);
                }

                match dst_bits {
                    1 => machine.la = val,
                    2 => machine.lb = val,
                    3 => machine.lc = val,
                    4 => machine.ld = val,
                    5 => machine.ptr = val,
                    6 => machine.pc = val as usize,
                    _ => unreachable!(),
                };
            }
            _ => {
                panic!("OPCODE {:02x} was not implemented.", opcode)
            }
        }
    }

    println!("{}", String::from_utf8(res).unwrap());

    String::from("")
}
