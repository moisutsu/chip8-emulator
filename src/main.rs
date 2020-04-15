extern crate chip8;

use std::env;
use std::process::exit;
use std::{thread, time};

fn main() {
    let mut cpu = chip8::Cpu::new();
    cpu.init_ram();
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        eprintln!("[Error] No input file.");
        exit(1);
    }
    cpu.load_rom(&argv[1]).unwrap();
    let sleep_time = time::Duration::from_millis(1000);
    loop {
        let next_pc = match cpu.fetch_instruction_code() {
            (0x0, 0x0, 0xE, 0xE) => cpu.ret(),
            (0x1, n1, n2, n3) => cpu.jp_addr(n1, n2, n3),
            _ => chip8::cpu::NextPc::Next,
        };
        cpu.set_pc(next_pc);
        cpu.print_registers();
        thread::sleep(sleep_time);
    }
}
