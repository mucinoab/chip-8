pub mod opcode;

use opcode::OpCode;

use rand::Rng;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const TICKS_PER_FRAME: usize = 10;

const FONTSET: [u8; 80] = [
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

pub struct Chip8 {
    /// Index
    idx: usize,

    /// Program Counter
    pc: u16,

    /// Stack Pointer
    sp: usize,
    stack: [u16; 16],

    /// Registers 0x0 - 0xF
    v: [u8; 16],

    /// Program ROM and work RAM
    /// 0x200 - 0xFFF
    mem: [u8; 4096],

    pub last_executed_instruction: OpCode,

    /// Monochromatic screen
    pub screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],

    delay_timer: u8,
    sound_timer: u8,
    rng: rand::rngs::ThreadRng,

    /// Keypad           Keyboard
    /// +-+-+-+-+        +-+-+-+-+
    /// |1|2|3|C|        |1|2|3|4|
    /// +-+-+-+-+        +-+-+-+-+
    /// |4|5|6|D|        |Q|W|E|R|
    /// +-+-+-+-+   =>   +-+-+-+-+
    /// |7|8|9|E|        |A|S|D|F|
    /// +-+-+-+-+        +-+-+-+-+
    /// |A|0|B|F|        |Z|X|C|V|
    /// +-+-+-+-+        +-+-+-+-+
    keypad: [bool; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        let mut c8 = Self {
            idx: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            v: [0; 16],
            mem: [0; 4096],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            rng: rand::thread_rng(),
            keypad: [false; 16],
            last_executed_instruction: OpCode::Cls,
        };

        // 0x000 to 0x1FF
        c8.mem[..FONTSET.len()].copy_from_slice(&FONTSET); // Copy font into memory.

        c8
    }

    pub fn load(&mut self, game_rom: &[u8]) {
        let start = 0x200;
        let end = start + game_rom.len();

        self.mem[start..end].copy_from_slice(game_rom);
    }

    pub fn reset(&mut self) {
        *self = Chip8::new();
    }

    pub fn cycle(&mut self) {
        // Fetch OpCode
        let op_a = self.mem[self.pc as usize];
        let op_b = self.mem[self.pc as usize + 1];

        // Decode Opcode
        self.last_executed_instruction = OpCode::decode(op_a, op_b);
        self.pc += 2;

        // Execute Opcode
        self.execute(self.last_executed_instruction);

        // Update timers
        self.delay_timer = self.delay_timer.saturating_sub(1);

        if self.sound_timer == 1 {
            // TODO sound
            eprintln!("BEEP!");
        }

        self.sound_timer = self.sound_timer.saturating_sub(1);
    }

    pub fn key2btn(key: &str) -> Option<usize> {
        match key {
            "1" => Some(0x1),
            "2" => Some(0x2),
            "3" => Some(0x3),
            "4" => Some(0xC),
            "q" => Some(0x4),
            "w" => Some(0x5),
            "e" => Some(0x6),
            "r" => Some(0xD),
            "a" => Some(0x7),
            "s" => Some(0x8),
            "d" => Some(0x9),
            "f" => Some(0xE),
            "z" => Some(0xA),
            "x" => Some(0x0),
            "c" => Some(0xB),
            "v" => Some(0xF),
            _ => None,
        }
    }

    pub fn keypress(&mut self, key: usize, pressed: bool) {
        self.keypad[key] = pressed;
    }

    pub fn pixel_cooridinates(&self) -> Vec<(usize, usize)> {
        self.screen
            .iter()
            .enumerate()
            .filter(|(_, p)| **p)
            .map(|(i, _)| {
                let x = i / SCREEN_WIDTH;
                let y = i % SCREEN_WIDTH;

                (x, y)
            })
            .collect()
    }

    /// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1
    fn execute(&mut self, op: OpCode) {
        match op {
            OpCode::Cls => self.clear_screen(),
            OpCode::Ret => {
                self.sp = self.sp.saturating_sub(1);
                self.pc = self.stack[self.sp];
            }
            OpCode::Jp(addr) => self.pc = addr as _,
            OpCode::Call(addr) => {
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                self.pc = addr;
            }
            OpCode::SeVx(reg_idx, value) => {
                if self.v[reg_idx as usize] == value as u8 {
                    self.pc += 2;
                }
            }
            OpCode::SNeVx(reg_idx, value) => {
                if self.v[reg_idx as usize] != value as u8 {
                    self.pc += 2;
                }
            }
            OpCode::SeVxVy(x, y) => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }
            OpCode::LdVx(reg_idx, value) => self.v[reg_idx as usize] = value as _,
            OpCode::AddVx(x, v) => {
                let vx = &mut self.v[x as usize];
                *vx = vx.wrapping_add(v as u8)
            }
            OpCode::LdVxVy(x, y) => self.v[x as usize] = self.v[y as usize],
            OpCode::Or(x, y) => self.v[x as usize] |= self.v[y as usize],
            OpCode::And(x, y) => self.v[x as usize] &= self.v[y as usize],
            OpCode::Xor(x, y) => self.v[x as usize] ^= self.v[y as usize],
            OpCode::Add(x, y) => {
                let x = x as usize;
                let y = y as usize;

                let (new_x, borrow) = self.v[x].overflowing_add(self.v[y]);

                self.v[x] = new_x;
                self.v[0xF] = borrow as _;
            }
            OpCode::Sub(x, y) => {
                let x = x as usize;
                let y = y as usize;

                let (new_x, carry) = self.v[x].overflowing_sub(self.v[y]);

                self.v[x] = new_x;
                self.v[0xF] = carry as _;
            }
            OpCode::Shr(x) => {
                let lsb = self.v[x as usize] & 1;
                self.v[x as usize] >>= 1;
                self.v[0xF] = lsb;
            }
            OpCode::SubN(x, y) => {
                let x = x as usize;
                let y = y as usize;

                let (new_x, carry) = self.v[y].overflowing_sub(self.v[x]);

                self.v[x] = new_x;
                self.v[0xF] = carry as _;
            }
            OpCode::Shl(x) => {
                // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
                // Then Vx is multiplied by 2.
                let msb = (self.v[x as usize] >> 7) & 1;
                self.v[x as usize] <<= 1;
                self.v[0xF] = msb;
            }
            OpCode::SeNeVxVy(x, y) => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            }
            OpCode::LdI(value) => self.idx = value as _,
            OpCode::JpV0(addr) => self.pc = addr + self.v[0] as u16,
            OpCode::Rnd(x, value) => self.v[x as usize] = self.rng.gen::<u8>() & value,
            OpCode::Drw(x, vy, n) => self.draw(x, vy, n),
            OpCode::Skp(x) => {
                if self.keypad[self.v[x as usize] as usize] {
                    self.pc += 2;
                }
            }
            OpCode::SkNp(x) => {
                if !self.keypad[self.v[x as usize] as usize] {
                    self.pc += 2;
                }
            }
            OpCode::LdVxDt(x) => self.v[x as usize] = self.delay_timer,
            OpCode::LdVxK(x) => {
                for (i, k) in self.keypad.iter().enumerate() {
                    if *k {
                        self.v[x as usize] = i as u8;
                        return;
                    }
                }

                self.pc -= 2; // Redo Opcode
            }
            OpCode::LdDtVx(x) => self.delay_timer = self.v[x as usize],
            OpCode::LdStVx(x) => self.sound_timer = self.v[x as usize],
            OpCode::AddIVx(x) => self.idx += self.v[x as usize] as usize,
            OpCode::LdFVx(x) => {
                // Each sprite is stored in 5 bytes, starting from the address 0x000 of ram, so to
                // get the address of the nth sprite, you multiply by that offset.
                self.idx = self.v[x as usize] as usize * 5;
            }
            OpCode::LdBVx(x) => {
                // BCD, takes the decimal value of Vx, and places the hundreds digit in memory at
                // location in I, the tens digit at location I+1, and the ones digit at location
                // I+2.
                let mut vx = self.v[x as usize];

                self.mem[self.idx + 2] = vx % 10;
                vx /= 10;

                self.mem[self.idx + 1] = vx % 10;
                vx /= 10;

                self.mem[self.idx] = vx;
            }
            OpCode::LdIVx(x) => {
                let x = x as usize;
                let registers = &self.v[..=x];
                self.mem[self.idx..=self.idx + x].copy_from_slice(registers);
            }
            OpCode::LdVxI(n) => {
                let n = n as usize;
                let memory = &self.mem[self.idx..=self.idx + n];
                self.v[..=n].copy_from_slice(memory);
            }
        }
    }

    fn draw(&mut self, vx: u8, vy: u8, n: u8) {
        // The interpreter reads n bytes from memory, starting at the address stored in I. These
        // bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed
        // onto the existing screen. If this causes any pixels to be erased, VF is set to 1,
        // otherwise it is set to 0. If the sprite is positioned so part of it is outside the
        // coordinates of the display, it wraps around to the opposite side of the screen. See
        // instruction 8xy3 for more information on XOR, and section 2.4, Display, for more
        // information on the Chip-8 screen and sprites.

        let x_base = self.v[vx as usize] as usize;
        let y_base = self.v[vy as usize] as usize;
        let n = n as usize;

        let mut collision = false;
        let pixels = &self.mem[self.idx..self.idx + n];

        for (y_line, &pixel) in pixels.iter().enumerate() {
            for x_line in 0..8 {
                if (pixel & (0b1000_0000 >> x_line)) != 0 {
                    let x = (x_base + x_line) % SCREEN_WIDTH;
                    let y = (y_base + y_line) % SCREEN_HEIGHT;

                    let current_pixel = &mut self.screen[x + SCREEN_WIDTH * y];

                    if *current_pixel {
                        collision = true;
                    }

                    *current_pixel ^= true;
                }
            }
        }

        self.v[0xF] = collision as u8;
    }

    pub fn clear_screen(&mut self) {
        self.screen.fill(false);
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}
