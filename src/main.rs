// Chip-8 Technical Reference
// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#00E0
// https://aymanbagabas.com/blog/2018/09/17/chip-8-emulator.html
// https://github.com/mattmikolay/chip-8/wiki/Mastering-CHIP%E2%80%908
// https://chip-8.github.io/links/

struct Chip8 {
    /// Index
    idx: usize,

    /// Program Counter
    pc: usize,

    /// Stack Pointer
    sp: usize,
    stack: [usize; 16],

    /// Registers
    v: [u8; 16],

    mem: Memory,
    gpu: Gpu,

    delay_timer: u8,
    sound_timer: u8,
}
impl Chip8 {
    fn new() -> Self {
        Self {
            idx: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            v: [0; 16],
            mem: Memory::new(),
            gpu: Gpu {},
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    fn load(&mut self, game_path: &str) {
        let s = std::fs::read(game_path).unwrap();

        for (idx, &op) in s.iter().enumerate() {
            self.mem.mem[idx + 0x200] = op;
        }
    }

    fn run(&mut self) {
        loop {
            if self.pc == 4096 {
                break;
            }

            self.cycle();

            if true {
                self.draw_graphics();
            }

            // self.set_keys(); // Capture input
        }
    }

    fn cycle(&mut self) {
        // Fetch Opcode
        let op_a = self.mem.mem[self.pc];
        let op_b = self.mem.mem[self.pc + 1];

        // Decode Opcode
        let op = OpCode::decode(op_a, op_b);

        // Execute Opcode
        self.execute(op);

        // Update timers
        self.delay_timer = self.delay_timer.saturating_sub(1);

        if self.sound_timer == 1 {
            eprintln!("BEEP!");
        }

        self.sound_timer = self.sound_timer.saturating_sub(1);
    }

    fn draw_graphics(&mut self) {}

    fn execute(&mut self, op: OpCode) {
        match op {
            OpCode::Cls => todo!(),
            OpCode::Ret => todo!(),
            OpCode::Jp(_) => todo!(),
            OpCode::Call(_) => todo!(),
            OpCode::SeVx(_, _) => todo!(),
            OpCode::SNeVx(_, _) => todo!(),
            OpCode::SeVxVy(_, _) => todo!(),
            OpCode::LdVx(_, _) => todo!(),
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
            OpCode::LdI(_) => todo!(),
            OpCode::JpV0(_) => todo!(),
            OpCode::Rnd(_, _) => todo!(),
            OpCode::Drw(_, _, _) => todo!(),
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
}

struct Memory {
    // 0x000-0x1FF - Chip 8 interpreter (contains font set in emu)
    // 0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)
    // 0x200-0xFFF - Program ROM and work RAM
    mem: [u8; 4096],
}

impl Memory {
    fn new() -> Self {
        Self { mem: [0; 4096] }
    }
}

struct Gpu {}

/// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.0
#[derive(PartialEq)]
enum OpCode {
    Cls,              // 00E0 - CLS
    Ret,              // 00EE - RET
    Jp(u8),           // 1nnn - JP addr
    Call(u8),         // 2nnn - CALL addr
    SeVx(u8, u8),     // 3xkk - SE Vx, byte
    SNeVx(u8, u8),    // 4xkk - SNE Vx, byte
    SeVxVy(u8, u8),   // 5xy0 - SE Vx, Vy
    LdVx(u8, u8),     // 6xkk - LD Vx, byte
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
    LdI(u8),          // Annn - LD I, addr
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

        match opcode & 0xF000 {
            0x0000 => match opcode & 0x000F {
                0x0000 => Self::Cls,
                0x000E => Self::Ret,
                _ => unreachable!("Invalid Op {:#x}", opcode),
            },
            0x1000 => Self::Jp((opcode & 0x0FFF) as u8),
            0x2000 => Self::Call((opcode & 0x0FFF) as u8),
            0x3000 => {
                let vx = (opcode & 0x0F00) as u8;
                let byte = (opcode & 0x00FF) as u8;
                Self::SeVx(vx, byte)
            }
            0x4000 => {
                let vx = (opcode & 0x0F00) as u8;
                let byte = (opcode & 0x00FF) as u8;
                Self::SNeVx(vx, byte)
            }
            0x5000 => {
                let vx = (opcode & 0x0F00) as u8;
                let vy = (opcode & 0x00F0) as u8;
                Self::SeVxVy(vx, vy)
            }
            0x6000 => {
                let vx = (opcode & 0x0F00) as u8;
                let byte = (opcode & 0x00FF) as u8;
                Self::LdVx(vx, byte)
            }
            0x7000 => {
                let vx = (opcode & 0x0F00) as u8;
                let byte = (opcode & 0x00FF) as u8;
                Self::AddVx(vx, byte)
            }
            0x8000 => {
                let vx = (opcode & 0x0F00) as u8;
                let vy = (opcode & 0x00F0) as u8;

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
                let vx = (opcode & 0x0F00) as u8;
                let vy = (opcode & 0x00F0) as u8;
                Self::SeNeVxVy(vx, vy)
            }
            0xA000 => Self::LdI((opcode & 0x0FFF) as u8),
            0xB000 => Self::JpV0((opcode & 0x0FFF) as u8),
            0xC000 => {
                let vx = (opcode & 0x0F00) as u8;
                let byte = (opcode & 0x00FF) as u8;
                Self::Rnd(vx, byte)
            }
            0xD000 => {
                let vx = (opcode & 0x0F00) as u8;
                let vy = (opcode & 0x00F0) as u8;
                let n = (opcode & 0x000F) as u8;
                Self::Drw(vx, vy, n)
            }
            0xE000 => {
                let vx = (opcode & 0x0F00) as u8;
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
                let vx = (opcode & 0x0F00) as u8;
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
