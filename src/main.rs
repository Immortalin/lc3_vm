use lazy_static::lazy_static;
use std::sync::Mutex;

static PC_START: u16 = 0x3000;
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

/// Array of opcodes to be executed based on their index
// v_str refers to vm str as `str` is a reserved word in rust
const op_ex: [fn(u16); NOPS] = [
    br, add, ld, st, jsr, and, ldr, v_str, rti, not, ldi, sti, jmp, res, lea, trap,
];

lazy_static! {
    static ref mem: Mutex::<[u16; u16::MAX as usize]> = Mutex::new([0; u16::MAX as usize]);
    static ref regs: Mutex::<[u16; Register::RCNT as usize]> =
        Mutex::new([0; Register::RCNT as usize]);
}

fn main() {
    mw(0, 0x1234);
    op_ex[0](0);
    println!("Location at address {} is {}", 0, mem.lock().unwrap()[0]);
}

fn br(i: u16) {
    println!("br");
}
fn add(i: u16) {}
fn ld(i: u16) {}
fn st(i: u16) {}
fn jsr(i: u16) {}
fn and(i: u16) {}
fn ldr(i: u16) {}
fn v_str(i: u16) {}
fn rti(i: u16) {}
fn not(i: u16) {}
fn ldi(i: u16) {}
fn sti(i: u16) {}
fn jmp(i: u16) {}
fn res(i: u16) {}
fn lea(i: u16) {}
fn trap(i: u16) {}

/// Shifts the instruction by 12 to get the leading 4 bits which is the opcode.
#[inline(always)]
fn opc(i: u16) -> u16 {
    i >> 12
}

#[inline(always)]
fn mr(address: u16) -> u16 {
    mem.lock().unwrap()[address as usize]
}

#[inline(always)]
fn mw(address: u16, value: u16) {
    mem.lock().unwrap()[address as usize] = value;
}
