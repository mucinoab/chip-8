// Chip-8 Technical Reference
// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#00E0
// https://aymanbagabas.com/blog/2018/09/17/chip-8-emulator.html
// https://github.com/mattmikolay/chip-8/wiki/Mastering-CHIP%E2%80%908
// https://chip-8.github.io/links/
// https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

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
    stack: [usize; 16],

    /// Registers
    v: [u8; 16],

    /// Ram Memory
    /// 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    /// 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    /// 0x200-0xFFF - Program ROM and work RAM
    mem: [u8; 4096],

    gpu: Gpu,

    delay_timer: u8,

    sound_timer: u8,
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
            gpu: Gpu::new(),
            delay_timer: 0,
            sound_timer: 0,
        };
        c8.mem[..FONTSET.len()].copy_from_slice(&FONTSET); // Copy font into memory.

        c8
    }

    fn load(&mut self, game_path: &str) {
        let s = std::fs::read(game_path).expect("Unable to find given game");

        for (idx, &op) in s.iter().enumerate() {
            self.mem[idx + 0x200] = op;
        }
    }

    fn run(&mut self) {
        loop {
            self.cycle();

            if true {
                self.draw_graphics();
            }

            // self.set_keys(); // Capture input
        }
    }

    fn cycle(&mut self) {
        // Fetch Opcode
        let op_a = self.mem[self.pc as usize];
        let op_b = self.mem[self.pc as usize + 1];

        // Decode Opcode
        let op = OpCode::decode(op_a, op_b);
        self.pc += 2;

        eprintln!("{:?}, pc: {}", op, self.pc);
        // Execute Opcode
        self.execute(op);

        // Update timers
        self.delay_timer = self.delay_timer.saturating_sub(1);

        if self.sound_timer == 1 {
            eprintln!("BEEP!");
        }

        self.sound_timer = self.sound_timer.saturating_sub(1);
        // std::io::stdin().read_line(&mut String::new());
    }

    fn draw_graphics(&self) {
        let mut screen = String::new();

        for x in 0..SCREEN_HEIGHT {
            for y in 0..SCREEN_WIDTH {
                let coor = x + y;
                if self.gpu.screen[coor] {
                    screen.push('#');
                } else {
                    screen.push(' ');
                }
            }
            screen.push('\n');
        }

        eprintln!("{}", screen);
        // std::io::stdin().read_line(&mut String::new());
    }

    /// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1
    fn execute(&mut self, op: OpCode) {
        match op {
            OpCode::Cls => self.gpu.clear(),
            OpCode::Ret => {
                self.pc = self.stack[self.sp] as u16;
                self.sp = self.sp.saturating_sub(1);
            }
            OpCode::Jp(addr) => self.pc = addr as _,
            OpCode::Call(_) => todo!(),
            OpCode::SeVx(_, _) => todo!(),
            OpCode::SNeVx(_, _) => todo!(),
            OpCode::SeVxVy(_, _) => todo!(),
            OpCode::LdVx(idx, value) => self.v[idx as usize] = value as _,
            OpCode::AddVx(_, _) => todo!(),
            OpCode::LdVxVy(_, _) => todo!(),
            OpCode::Or(_, _) => todo!(),
            OpCode::And(_, _) => todo!(),
            OpCode::Xor(_, _) => todo!(),
            OpCode::Add(_, _) => todo!(),
            OpCode::Sub(_, _) => todo!(),
            OpCode::Shr(_) => todo!(),
            OpCode::SubN(_, _) => todo!(),
            OpCode::Shl(_) => todo!(),
            OpCode::SeNeVxVy(_, _) => todo!(),
            OpCode::LdI(byte) => self.idx = byte as _,
            OpCode::JpV0(_) => todo!(),
            OpCode::Rnd(_, _) => todo!(),
            OpCode::Drw(vx, vy, n) => self.draw(vx, vy, n),
            OpCode::Skp(_) => todo!(),
            OpCode::SkNp(_) => todo!(),
            OpCode::LdVxDt(_) => todo!(),
            OpCode::LdVxK => todo!(),
            OpCode::LdDtVx(_) => todo!(),
            OpCode::LdStVx(_) => todo!(),
            OpCode::AddIVx(_) => todo!(),
            OpCode::LdFVx(_) => todo!(),
            OpCode::LdBVx(_) => todo!(),
            OpCode::LdIVx(_) => todo!(),
            OpCode::LdVxI(_) => todo!(),
            OpCode::Nop => todo!(),
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

                    let current_pixel = &mut self.gpu.screen[x + SCREEN_WIDTH * y];

                    if *current_pixel {
                        collision = true;
                    }

                    *current_pixel ^= true;
                }
            }
        }

        self.v[0xF] = collision as _;
    }
}

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

struct Gpu {
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Gpu {
    fn clear(&mut self) {
        self.screen.fill(false);
    }

    fn new() -> Self {
        Self {
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }
}

/// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.0
#[derive(Debug, PartialEq)]
enum OpCode {
    Cls,              // 00E0 - CLS
    Ret,              // 00EE - RET
    Jp(usize),        // 1nnn - JP addr
    Call(u8),         // 2nnn - CALL addr
    SeVx(u8, u8),     // 3xkk - SE Vx, byte
    SNeVx(u8, u8),    // 4xkk - SNE Vx, byte
    SeVxVy(u8, u8),   // 5xy0 - SE Vx, Vy
    LdVx(u8, u16),    // 6xkk - LD Vx, byte
    AddVx(u8, u8),    // 7xkk - ADD Vx, byte
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
    JpV0(u8),         // Bnnn - JP V0, addr
    Rnd(u8, u8),      // Cxkk - RND Vx, byte
    Drw(u8, u8, u8),  // Dxyn - DRW Vx, Vy, nibble
    Skp(u8),          // Ex9E - SKP Vx
    SkNp(u8),         // ExA1 - SKNP Vx
    LdVxDt(u8),       // Fx07 - LD Vx, DT
    LdVxK,            // Fx0A - LD Vx, K
    LdDtVx(u8),       // Fx15 - LD DT, Vx
    LdStVx(u8),       // Fx18 - LD ST, Vx
    AddIVx(u8),       // Fx1E - ADD I, Vx
    LdFVx(u8),        // Fx29 - LD F, Vx
    LdBVx(u8),        // Fx33 - LD B, Vx
    LdIVx(u8),        // Fx55 - LD [I], Vx
    LdVxI(u8),        // Fx65 - LD Vx, [I]
    //
    Nop,
}

impl OpCode {
    fn decode(raw_a: u8, raw_b: u8) -> Self {
        let opcode: u16 = (raw_a as u16) << 8 | (raw_b as u16);
        eprint!("{:#8x} ", opcode);

        match opcode & 0xF000 {
            0x0000 => match opcode & 0x000F {
                0x0000 => Self::Cls,
                0x000E => Self::Ret,
                _ => unreachable!("Invalid Op {:#x}", opcode),
            },
            0x1000 => Self::Jp((opcode & 0x0FFF) as _),
            0x2000 => Self::Call((opcode & 0x0FFF) as u8),
            0x3000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let byte = (opcode & 0x00FF) as u8;
                Self::SeVx(vx, byte)
            }
            0x4000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let byte = (opcode & 0x00FF) as u8;
                Self::SNeVx(vx, byte)
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
                let byte = (opcode & 0x00FF) as u8;
                Self::AddVx(vx, byte)
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
            0xB000 => Self::JpV0((opcode & 0x0FFF) as u8),
            0xC000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                let byte = (opcode & 0x00FF) as u8;
                Self::Rnd(vx, byte)
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
                    _ => {
                        eprintln!("Invalid Op {:#x} {:#x}", opcode, opcode & 0x00FF);
                        Self::Nop
                    }
                }
            }
            0xF000 => {
                let vx = ((opcode & 0x0F00) >> 8) as u8;
                match opcode & 0x00FF {
                    0x0007 => Self::LdVxDt(vx),
                    0x000A => Self::LdVxK,
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
    // set up input Bevy? Think about wasm
    // set up graphics Bevy?
    let mut c8 = Chip8::new();
    c8.load("./test_opcode.ch8");
    c8.run();
}
