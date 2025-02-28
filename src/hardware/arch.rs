use crate::datatypes::Program;

/**
 * Architecture definitions, based on the MIPS ISA.
 * 
 * Doesn't implment sc or ll
 * sammc
 */

// Number of Registers
pub const REG_NUM: u32 = 32;    
// Memory Address Declarations
pub const PC_START: u32 = 0x0040;        // PC Starting Address
pub const STATIC_DATA: u32 = 0x1000;     // Static Data Space
pub const DYNAMIC_DATA: u32 = 0x4000;    // Dyanamic Data Space
pub const END_MEM: u32 = 0x8000;         // Last Valid Memory Address

// Memory Size Declarations
pub const MEM_SIZE: u32 = END_MEM + 1;        // Address Space in bytes

pub trait Computer {
    fn load_program(&mut self, program: Program);
    fn start(&mut self);
}

pub trait MipsIsa {
    // R-Instructions
    fn add(&mut self, rs: u32, rt: u32, rd: u32);
    fn addu(&mut self, rs: u32, rt: u32, rd: u32);
    fn and(&mut self, rs: u32, rt: u32, rd: u32);
    fn jr(&mut self, rs: u32);
    fn nor(&mut self, rs: u32, rt: u32, rd: u32);
    fn or(&mut self, rs: u32, rt: u32, rd: u32);
    fn slt(&mut self, rs: u32, rt: u32, rd: u32);
    fn sltu(&mut self, rs: u32, rt: u32, rd: u32);
    fn sll(&mut self, rs: u32, rd: u32, shamt: u32);
    fn srl(&mut self, rs: u32, rd: u32, shamt: u32);
    fn sub(&mut self, rs: u32, rt: u32, rd: u32);
    fn subu(&mut self, rs: u32, rt: u32, rd: u32);
    // I-Instructions
    fn addi(&mut self, rs: u32, rt: u32, immediate: i16);
    fn addiu(&mut self, rs: u32, rt: u32, immediate: i16);
    fn andi(&mut self, rs: u32, rt: u32, immediate: i16);
    fn ori(&mut self, rs: u32, rt: u32, immediate: i16);
    fn beq(&mut self, rs: u32, rt: u32, immediate: i16);
    fn bne(&mut self, rs: u32, rt: u32, immediate: i16);
    fn lbu(&mut self, rs: u32, rt: u32, immediate: i16);
    fn lhu(&mut self, rs: u32, rt: u32, immediate: i16);
    fn lui(&mut self, rs: u32, immediate: i16);
    fn lw(&mut self, rs: u32, rt: u32, immediate: i16);
    fn sb(&mut self, rs: u32, rt: u32, immediate: i16);
    fn sh(&mut self, rs: u32, rt: u32, immediate: i16);
    fn sw(&mut self, rs: u32, rt: u32, immediate: i16);
    fn slti(&mut self, rs: u32, rt: u32, immediate: i16);
    fn sltiu(&mut self, rs: u32, rt: u32, immediate: u16);    
    // J-Instructions
    fn j(&mut self, address: u32);
    fn jal(&mut self, address: u32);

    // Execute command which also holds the 'decoding'
    fn decode_execute(&mut self, instruction: u32) {
        let opcode = instruction >> 26;        // 31..26
        let rs: u32 = (instruction >> 21) & 0x1F;   // 25..21
        let rt: u32 = (instruction >> 16) & 0x1F;   // 20..16
        let rd: u32 = (instruction >> 11) & 0x1F;   // 15..11
        let shamt: u32 = (instruction >> 6) & 0x1F; // 10..6
        let func: u32 = instruction & 0x3F;         // 5..0

        let immediate: i16 = (instruction & 0xFFFF) as i16; // 16 bits
        let address: u32 = instruction & 0xFF_FFFF;         // 24 bits

        match opcode {
            // R-Type
            0x0 => 
                match func {
                    0x20 => Self::add(self, rs, rt, rd),
                    0x21 => Self::addu(self, rs, rt, rd),
                    0x24 => Self::and(self, rs, rt, rd),
                    0x8 => Self::jr(self, rs),
                    0x27 => Self::nor(self, rs, rt, rd),
                    0x25 => Self::or(self, rs, rt, rd),
                    0x2a => Self::slt(self, rs, rt, rd),
                    0x2b => Self::sltu(self, rs, rt, rd),
                    0x0 => Self::sll(self, rs, rd, shamt),
                    0x2 => Self::srl(self, rs, rd, shamt),
                    0x22 => Self::sub(self, rs, rt, rd),
                    0x23 => Self::subu(self, rs, rt, rd),
                    _ => ()
                },
            
            // J-Type
            0x2 => Self::j(self, address),
            0x3 => Self::jal(self, address),

            // I-Type
            0x8 => Self::addi(self, rs, rt, immediate),
            0x9 => Self::addiu(self, rs, rt, immediate),
            0xc => Self::andi(self, rs, rt, immediate),
            0x4 => Self::beq(self, rs, rt, immediate),
            0x5 => Self::bne(self, rs, rt, immediate),
            0xf => Self::lui(self, rs, immediate),
            0x23 => Self::lw(self, rs, rt, immediate),
            0xd => Self::ori(self, rs, rt, immediate),
            0xa => Self::slti(self, rs, rt, immediate),
            0xb => Self::sltiu(self, rs, rt, immediate as u16),
            0x2b => Self::sw(self, rs, rt, immediate), 
            0x24 => Self::lbu(self, rs, rt, immediate),
            0x25 => Self::lhu(self, rs, rt, immediate),
            0x28 => Self::sb(self, rs, rt, immediate),
            0x29 => Self::sh(self, rs, rt, immediate),
            _ => ()
        }
    }

}




