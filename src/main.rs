use std::rc::Rc;
use std::sync::Mutex;

const PC_START: u16 = 0x3000;
const NOPS: usize = 16;

enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    RPC,
    RCND,
    RCNT,
}
enum Flag {
    FP = 1 << 0,
    FZ = 1 << 1,
    FN = 1 << 2,
}

struct VM {
    // pc: u16,
    regs: [u16; Register::RCNT as usize],
    mem: [u16; u16::MAX as usize],
    /// Op code execution lookup table,
    /// array of opcodes to be executed based on their index.
    /// `v_str` refers to vm str as `str` is a reserved word in rust
    op_ex: [fn(&VM, u16); NOPS],
}

fn main() {
    let mut vm = VM::new();
    vm.mw(0, 1234);
    vm.do_op(0, 0);
    println!("Location at address {} is {}", 0, vm.mem[0]);
}

impl VM {
    fn new() -> Self {
        Self {
            regs: [0; Register::RCNT as usize],
            mem: [0; u16::MAX as usize],
            op_ex: [
                Self::br,
                Self::add,
                Self::ld,
                Self::st,
                Self::jsr,
                Self::and,
                Self::ldr,
                Self::v_str,
                Self::rti,
                Self::not,
                Self::ldi,
                Self::sti,
                Self::jmp,
                Self::res,
                Self::lea,
                Self::trap,
            ],
        }
    }

    fn do_op(&self, op: usize, i: u16) {
        self.op_ex[op](self, i);
    }

    fn br(&self, i: u16) {
        println!("br");
    }
    fn add(&self, i: u16) {}
    fn ld(&self, i: u16) {}
    fn st(&self, i: u16) {}
    fn jsr(&self, i: u16) {}
    fn and(&self, i: u16) {}
    fn ldr(&self, i: u16) {}
    fn v_str(&self, i: u16) {}
    fn rti(&self, i: u16) {}
    fn not(&self, i: u16) {}
    fn ldi(&self, i: u16) {}
    fn sti(&self, i: u16) {}
    fn jmp(&self, i: u16) {}
    fn res(&self, i: u16) {}
    fn lea(&self, i: u16) {}
    fn trap(&self, i: u16) {}

    #[inline(always)]
    fn process_signed_num(&mut self, reg: Register) {
        let r = reg as usize;
        if self.regs[r] == 0 {
            self.regs[Register::RCND as usize] = Flag::FZ as u16; // the value in r is zero
        } else if self.regs[r] >> 15 != 0 {
            self.regs[Register::RCND as usize] = Flag::FN as u16; // the value in r is z negative number
        } else {
            self.regs[Register::RCND as usize] = Flag::FP as u16; // the value in r is a positive number
        }
    }

    /// Shifts the instruction by 12 to get the leading 4 bits which is the opcode.
    #[inline(always)]
    fn opc(i: u16) -> u16 {
        i >> 12
    }

    #[inline(always)]
    fn mr(&self, address: u16) -> u16 {
        self.mem[address as usize]
    }

    #[inline(always)]
    fn mw(&mut self, address: u16, value: u16) {
        self.mem[address as usize] = value;
    }
}
