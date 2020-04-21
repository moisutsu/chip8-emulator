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
        cpu.print_instruction_code();
        let next_pc = match cpu.fetch_instruction_code() {
            (0x0, 0x0, 0xE, 0xE) => cpu.ret(),
            (0x1, n1, n2, n3) => cpu.jp_addr(n1, n2, n3),
            (0x2, n1, n2, n3) => cpu.call_addr(n1, n2, n3),
            (0x3, x, k1, k2) => cpu.se_vx_byte(x, k1, k2),
            (0x4, x, k1, k2) => cpu.sne_vx_byte(x, k1, k2),
            (0x5, x, y, 0x0) => cpu.se_vx_vy(x, y),
            (0x6, x, k1, k2) => cpu.ld_vx_byte(x, k1, k2),
            (0x7, x, k1, k2) => cpu.add_vx_byte(x, k1, k2),
            (0x8, x, y, 0x0) => cpu.ld_vx_vy(x, y),
            (0x8, x, y, 0x1) => cpu.or_vx_vy(x, y),
            (0x8, x, y, 0x2) => cpu.and_vx_vy(x, y),
            (0x8, x, y, 0x3) => cpu.xor_vx_vy(x, y),
            (0x8, x, y, 0x4) => cpu.add_vx_vy(x, y),
            (0x8, x, y, 0x5) => cpu.sub_vx_vy(x, y),
            (0x8, x, y, 0x6) => cpu.shr_vx_vy(x, y),
            (0x8, x, y, 0x7) => cpu.subn_vx_vy(x, y),
            (0x8, x, y, 0xE) => cpu.shl_vx_vy(x, y),
            (0x9, x, y, 0x0) => cpu.sne_vx_vy(x, y),
            (0xA, n1, n2, n3) => cpu.ld_i_addr(n1, n2, n3),
            (0xB, n1, n2, n3) => cpu.jp_v0_addr(n1, n2, n3),
            (0xC, x, k1, k2) => cpu.rnd_vx_byte(x, k1, k2),
            (0xF, x, 0x0, 0x7) => cpu.ld_vx_dt(x),
            (0xF, x, 0x1, 0x5) => cpu.ld_dt_vx(x),
            (0xF, x, 0x1, 0x8) => cpu.ld_st_vx(x),
            (0xF, x, 0x1, 0xE) => cpu.add_i_vx(x),
            _ => chip8::cpu::NextPc::Next,
        };
        cpu.set_pc(next_pc);
        cpu.print_registers();
        thread::sleep(sleep_time);
    }
}
