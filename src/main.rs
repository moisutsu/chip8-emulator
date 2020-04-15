extern crate chip8;

use std::env;
use std::process::exit;

fn main() {
    let mut cpu = chip8::Cpu::new();
    cpu.init_ram();
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        eprintln!("[Error] No input file.");
        exit(1);
    }
    cpu.load_rom(&argv[1]).unwrap();
    cpu.print_registers();
}
