use rand::Rng;

const TICKS_PER_FRAME: usize = 10;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
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

struct Chip8 {
    /// Index
    idx: usize,

    /// Program Counter
    pc: u16,

    /// Stack Pointer
    sp: usize,
    stack: [u16; 16],

    /// Registers 0x0 - 0xF
    v: [u8; 16],

    /// 0x200-0xFFF - Program ROM and work RAM
    mem: [u8; 4096],

    /// Monochromatic screen
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],

    delay_timer: u8,
    sound_timer: u8,
    rng: rand::rngs::ThreadRng,

    /// Keypad             Keyboard
    /// +-+-+-+-+          +-+-+-+-+
    /// |1|2|3|C|          |1|2|3|4|
    /// +-+-+-+-+          +-+-+-+-+
    /// |4|5|6|D|          |Q|W|E|R|
    /// +-+-+-+-+    =>    +-+-+-+-+
    /// |7|8|9|E|          |A|S|D|F|
    /// +-+-+-+-+          +-+-+-+-+
    /// |A|0|B|F|          |Z|X|C|V|
    /// +-+-+-+-+          +-+-+-+-+
    keypad: [bool; 16],
}

impl Chip8 {
    fn new() -> Self {
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
        };

        // 0x000 to 0x1FF
        c8.mem[..FONTSET.len()].copy_from_slice(&FONTSET); // Copy font into memory.

        c8
    }

    fn load(&mut self, game_path: &str) {
        let s = std::fs::read(game_path).expect("Unable to find given game");

        for (idx, &op) in s.iter().enumerate() {
            self.mem[idx + 0x200] = op;
        }
    }

    fn cycle(&mut self) {
        // Fetch Opcode
        let op_a = self.mem[self.pc as usize];
        let op_b = self.mem[self.pc as usize + 1];

        // Decode Opcode
        let op = OpCode::decode(op_a, op_b);
        self.pc += 2;

        // Execute Opcode
        self.execute(op);

        // Update timers
        self.delay_timer = self.delay_timer.saturating_sub(1);

        if self.sound_timer == 1 {
            // TODO sound
            eprintln!("BEEP!");
        }

        self.sound_timer = self.sound_timer.saturating_sub(1);
    }

    fn draw_graphics(&self) {
        let mut screen = vec![vec![' '; SCREEN_WIDTH]; SCREEN_HEIGHT];

        self.screen
            .iter()
            .enumerate()
            .filter(|(_, p)| **p)
            .map(|(i, _)| i)
            .for_each(|i| {
                let x = i / SCREEN_WIDTH;
                let y = i % SCREEN_WIDTH;

                screen[x][y] = '0';
            });

        let mut s = String::with_capacity((SCREEN_HEIGHT * SCREEN_WIDTH) + SCREEN_HEIGHT);

        for row in screen {
            s.extend(row.iter());
            s.push('\n');
        }

        print!("{esc}[2J{esc}[1;1H{s}", esc = 27 as char);

        // std::io::stdin().read_line(&mut String::new());
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

        self.v[0xF] = collision as _;
    }

    fn run(&mut self) -> ! {
        loop {
            for _ in 0..TICKS_PER_FRAME {
                self.cycle();
            }

            self.draw_graphics();
            std::thread::sleep(std::time::Duration::from_millis(16))
        }
    }

    fn clear_screen(&mut self) {
        self.screen.fill(false);
    }
}

/// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.0
#[derive(Debug, PartialEq)]
enum OpCode {
    Cls,              // 00E0 - CLS
    Ret,              // 00EE - RET
    Jp(u16),          // 1nnn - JP addr
    Call(u16),        // 2nnn - CALL addr
    SeVx(u8, u16),    // 3xkk - SE Vx, value
    SNeVx(u8, u16),   // 4xkk - SNE Vx, value
    SeVxVy(u8, u8),   // 5xy0 - SE Vx, Vy
    LdVx(u8, u16),    // 6xkk - LD Vx, value
    AddVx(u8, u16),   // 7xkk - ADD Vx, value
    LdVxVy(u8, u8),   // 8xy0 - LD Vx, Vy
    Or(u8, u8),       // 8xy1 - OR Vx, Vy
    And(u8, u8),      // 8xy2 - AND Vx, Vy
    Xor(u8, u8),      // 8xy3 - XOR Vx, Vy
    Add(u8, u8),      // 8xy4 - ADD Vx, Vy
    Sub(u8, u8),      // 8xy5 - SUB Vx, Vy
    Shr(u8),          // 8xy6 - SHR Vx {, Vy}
    SubN(u8, u8),     // 8xy7 - SUBN Vx, Vy
    Shl(u8),          // 8xyE - SHL Vx {, Vy}
    SeNeVxVy(u8, u8), // 9xy0 - SNE Vx, Vy
    LdI(u16),         // Annn - LD I, addr
    JpV0(u16),        // Bnnn - JP V0, addr
    Rnd(u8, u8),      // Cxkk - RND Vx, value
    Drw(u8, u8, u8),  // Dxyn - DRW Vx, Vy, nibble
    Skp(u8),          // Ex9E - SKP Vx
    SkNp(u8),         // ExA1 - SKNP Vx
    LdVxDt(u8),       // Fx07 - LD Vx, DT
    LdVxK(u8),        // Fx0A - LD Vx, K
    LdDtVx(u8),       // Fx15 - LD DT, Vx
    LdStVx(u8),       // Fx18 - LD ST, Vx
    AddIVx(u8),       // Fx1E - ADD I, Vx
    LdFVx(u8),        // Fx29 - LD F, Vx
    LdBVx(u8),        // Fx33 - LD B, Vx
    LdIVx(u8),        // Fx55 - LD [I], Vx
    LdVxI(u8),        // Fx65 - LD Vx, [I]
}

impl OpCode {
    fn decode(raw_a: u8, raw_b: u8) -> Self {
        let opcode: u16 = (raw_a as u16) << 8 | (raw_b as u16);

        match opcode & 0xF000 {
            0x0000 => match opcode & 0x000F {
                0x0000 => Self::Cls,
                0x000E => Self::Ret,
                _ => unreachable!("Invalid Op {:#x}", opcode),
            },
            0x1000 => Self::Jp(opcode & 0x0FFF),
            0x2000 => Self::Call(opcode & 0x0FFF),
            0x3000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let value = opcode & 0x00FF;
                Self::SeVx(vx, value)
            }
            0x4000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let value = opcode & 0x00FF;
                Self::SNeVx(vx, value)
            }
            0x5000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let vy = ((opcode & 0x00F0) >> 4) as u8;
                Self::SeVxVy(vx, vy)
            }
            0x6000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let value = opcode & 0x00FF;
                Self::LdVx(vx, value)
            }
            0x7000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let value = opcode & 0x00FF;
                Self::AddVx(vx, value)
            }
            0x8000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let vy = ((opcode & 0x00F0) >> 4) as u8;

                match opcode & 0x000F {
                    0x0000 => Self::LdVxVy(vx, vy),
                    0x0001 => Self::Or(vx, vy),
                    0x0002 => Self::And(vx, vy),
                    0x0003 => Self::Xor(vx, vy),
                    0x0004 => Self::Add(vx, vy),
                    0x0005 => Self::Sub(vx, vy),
                    0x0006 => Self::Shr(vx),
                    0x0007 => Self::SubN(vx, vy),
                    0x000E => Self::Shl(vx),
                    _ => unreachable!("Invalid Op {:#x}", opcode),
                }
            }
            0x9000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let vy = ((opcode & 0x00F0) >> 4) as u8;
                Self::SeNeVxVy(vx, vy)
            }
            0xA000 => Self::LdI(opcode & 0x0FFF),
            0xB000 => Self::JpV0(opcode & 0x0FFF),
            0xC000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let value = (opcode & 0x00FF) as u8;
                Self::Rnd(vx, value)
            }
            0xD000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let vy = ((opcode & 0x00F0) >> 4) as u8;
                let n = (opcode & 0x000F) as u8;
                Self::Drw(vx, vy, n)
            }
            0xE000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                match opcode & 0x00FF {
                    0x009E => Self::Skp(vx),
                    0x00A1 => Self::SkNp(vx),
                    _ => unreachable!("Invalid Op {:#x} {:#x}", opcode, opcode & 0x00FF),
                }
            }
            0xF000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                match opcode & 0x00FF {
                    0x0007 => Self::LdVxDt(vx),
                    0x000A => Self::LdVxK(vx),
                    0x0015 => Self::LdDtVx(vx),
                    0x0018 => Self::LdStVx(vx),
                    0x001E => Self::AddIVx(vx),
                    0x0029 => Self::LdFVx(vx),
                    0x0033 => Self::LdBVx(vx),
                    0x0055 => Self::LdIVx(vx),
                    0x0065 => Self::LdVxI(vx),
                    _ => unreachable!("Invalid Op {:#x}", opcode),
                }
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut c8 = Chip8::new();
    c8.load("./br8kout.ch8");
    c8.run();
}
