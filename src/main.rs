extern crate chip8;

fn main() {
    let mut cpu = chip8::Cpu::new();
    cpu.init_ram();
    cpu.print_registers();
}
