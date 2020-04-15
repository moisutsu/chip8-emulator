use std::fs::File;
use std::io::Read;

pub struct Cpu {
    v: [u8; 16],
    i: u16,
    delay: u8,
    sound: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    ram: [u8; 0xFFF],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            v: [0; 16],
            i: 0,
            delay: 0,
            sound: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            ram: [0; 0xFFF]
        }
    }
    pub fn init_ram(&mut self) {
        let fontpreset = vec![
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        self.ram[..fontpreset.len()].copy_from_slice(&fontpreset);
    }
    pub fn print_registers(&self) {
        for i in 0..15 {
            print!("V{:X}: {:X}, ", i, self.v[i]);
        }
        println!("VF: {:X}", self.v[15]);
    }
    pub fn load_rom(&mut self, file_path: &str) -> std::io::Result<()> {
        let mut file = File::open(file_path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        self.ram[0x200..0x200+buf.len()].copy_from_slice(&buf);
        Ok(())
    }
    /// pcが指す16ビットの命令コードをフェッチし,4ビットずつにわけて返す
    pub fn fetch_instruction_code(&self) -> (u8, u8, u8, u8) {
        let (b1, b2): (u8, u8) = (self.ram[self.pc as usize], self.ram[(self.pc + 1) as usize]);
        let v1 = b1 >> 4;
        let v2 = b1 & 0b00001111;
        let v3 = b2 >> 4;
        let v4 = b2 & 0b00001111;
        (v1, v2, v3, v4)
    }
}
