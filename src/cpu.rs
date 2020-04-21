use rand;
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

pub enum NextPc {
    Next,
    Skip,
    Jump(u16),
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
            ram: [0; 0xFFF],
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
        for i in 0..16 {
            print!("V{:X}: {:X}, ", i, self.v[i]);
        }
        print!("I: {:X}, ", self.i);
        print!("PC: {:X}, ", self.pc);
        println!("SP: {:X}", self.sp);
    }
    pub fn print_instruction_code(&self) {
        let (b1, b2): (u8, u8) = (self.ram[self.pc as usize], self.ram[(self.pc + 1) as usize]);
        let v1 = b1 >> 4;
        let v2 = b1 & 0b00001111;
        let v3 = b2 >> 4;
        let v4 = b2 & 0b00001111;
        println!("命令コード: {:X}{:X}{:X}{:X}", v1, v2, v3, v4);
    }
    pub fn load_rom(&mut self, file_path: &str) -> std::io::Result<()> {
        let mut file = File::open(file_path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        self.ram[0x200..0x200 + buf.len()].copy_from_slice(&buf);
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
    pub fn set_pc(&mut self, next_pc: NextPc) {
        match next_pc {
            NextPc::Next => {
                self.pc += 2;
            }
            NextPc::Skip => {
                self.pc += 4;
            }
            NextPc::Jump(addr) => self.pc = addr,
        }
    }
    // 命令セットの定義
    pub fn ret(&mut self) -> NextPc {
        let next_pc = self.stack[self.sp as usize];
        self.sp -= 1;
        NextPc::Jump(next_pc)
    }
    pub fn jp_addr(&mut self, n1: u8, n2: u8, n3: u8) -> NextPc {
        let next_pc = ((n1 as u16) << 8) + ((n2 as u16) << 4) + n3 as u16;
        NextPc::Jump(next_pc)
    }
    pub fn call_addr(&mut self, n1: u8, n2: u8, n3: u8) -> NextPc {
        self.sp += 1;
        let next_pc = ((n1 as u16) << 8) + ((n2 as u16) << 4) + n3 as u16;
        NextPc::Jump(next_pc)
    }
    pub fn se_vx_byte(&self, x: u8, k1: u8, k2: u8) -> NextPc {
        if self.v[x as usize] == (k1 << 4) + k2 {
            NextPc::Skip
        } else {
            NextPc::Next
        }
    }
    pub fn sne_vx_byte(&self, x: u8, k1: u8, k2: u8) -> NextPc {
        if self.v[x as usize] != (k1 << 4) + k2 {
            NextPc::Skip
        } else {
            NextPc::Next
        }
    }
    pub fn se_vx_vy(&self, x: u8, y: u8) -> NextPc {
        if self.v[x as usize] == self.v[y as usize] {
            NextPc::Skip
        } else {
            NextPc::Next
        }
    }
    pub fn ld_vx_byte(&mut self, x: u8, k1: u8, k2: u8) -> NextPc {
        self.v[x as usize] = (k1 << 4) + k2;
        NextPc::Next
    }
    pub fn add_vx_byte(&mut self, x: u8, k1: u8, k2: u8) -> NextPc {
        self.v[x as usize] = self.v[x as usize] + (k1 << 4) + k2;
        NextPc::Next
    }
    pub fn ld_vx_vy(&mut self, x: u8, y: u8) -> NextPc {
        self.v[x as usize] = self.v[y as usize];
        NextPc::Next
    }
    pub fn or_vx_vy(&mut self, x: u8, y: u8) -> NextPc {
        self.v[x as usize] = self.v[x as usize] | self.v[y as usize];
        NextPc::Next
    }
    pub fn and_vx_vy(&mut self, x: u8, y: u8) -> NextPc {
        self.v[x as usize] &= self.v[y as usize];
        NextPc::Next
    }
    pub fn xor_vx_vy(&mut self, x: u8, y: u8) -> NextPc {
        self.v[x as usize] ^= self.v[y as usize];
        NextPc::Next
    }
    pub fn add_vx_vy(&mut self, x: u8, y: u8) -> NextPc {
        // 16bitに変換してから255より大きいかを判定する
        let retval: u16 = self.v[x as usize] as u16 + self.v[y as usize] as u16;
        self.v[0xF] = if retval > 255 { 1 } else { 0 };
        // as u8 より下位8bitのみ保持
        self.v[x as usize] = retval as u8;
        NextPc::Next
    }
    pub fn sub_vx_vy(&mut self, x: u8, y: u8) -> NextPc {
        self.v[0xF] = if self.v[x as usize] > self.v[y as usize] {
            1
        } else {
            0
        };
        // u8 - u8 で負になる場合の代入は未実装
        if self.v[x as usize] >= self.v[y as usize] {
            self.v[x as usize] = self.v[x as usize] - self.v[y as usize];
        }
        NextPc::Next
    }
    pub fn shr_vx_vy(&mut self, x: u8, _y: u8) -> NextPc {
        self.v[0xF] = if self.v[x as usize] % 2 == 1 { 1 } else { 0 };
        self.v[x as usize] >>= 1;
        NextPc::Next
    }
    pub fn subn_vx_vy(&mut self, x: u8, y: u8) -> NextPc {
        self.v[0xF] = if self.v[y as usize] > self.v[x as usize] {
            1
        } else {
            0
        };
        // u8 - u8 が負になる場合は未実装
        if self.v[y as usize] >= self.v[x as usize] {
            self.v[x as usize] = self.v[y as usize] - self.v[x as usize];
        }
        NextPc::Next
    }
    pub fn shl_vx_vy(&mut self, x: u8, _y: u8) -> NextPc {
        self.v[0xF] = if self.v[x as usize] >= 128 { 1 } else { 0 };
        self.v[x as usize] <<= 1;
        NextPc::Next
    }
    pub fn sne_vx_vy(&self, x: u8, y: u8) -> NextPc {
        if self.v[x as usize] != self.v[y as usize] {
            NextPc::Skip
        } else {
            NextPc::Next
        }
    }
    pub fn ld_i_addr(&mut self, n1: u8, n2: u8, n3: u8) -> NextPc {
        self.i = ((n1 as u16) << 8) + ((n2 as u16) << 4) + n3 as u16;
        NextPc::Next
    }
    pub fn jp_v0_addr(&self, n1: u8, n2: u8, n3: u8) -> NextPc {
        let next_pc: u16 = ((n1 as u16) << 8) + ((n2 as u16) << 4) + n3 as u16 + self.v[0x0] as u16;
        NextPc::Jump(next_pc)
    }
    pub fn rnd_vx_byte(&mut self, x: u8, k1: u8, k2: u8) -> NextPc {
        let kk: u8 = (k1 << 4) + k2;
        self.v[x as usize] = kk & rand::random::<u8>();
        NextPc::Next
    }
    pub fn ld_vx_dt(&mut self, x: u8) -> NextPc {
        self.v[x as usize] = self.delay;
        NextPc::Next
    }
    pub fn ld_dt_vx(&mut self, x: u8) -> NextPc {
        self.delay = self.v[x as usize];
        NextPc::Next
    }
    pub fn ld_st_vx(&mut self, x: u8) -> NextPc {
        self.sound = self.v[x as usize];
        NextPc::Next
    }
    pub fn add_i_vx(&mut self, x: u8) -> NextPc {
        self.i += self.v[x as usize] as u16;
        NextPc::Next
    }
}
