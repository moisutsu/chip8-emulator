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
        match cpu.fetch_instruction_code() {
            _ => (),
        }
        cpu.print_registers();
        thread::sleep(sleep_time);
    }
}
