/// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.0
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum OpCode {
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
    pub fn decode(raw_a: u8, raw_b: u8) -> Self {
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
