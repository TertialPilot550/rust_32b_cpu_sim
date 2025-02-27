/**
 * Hold the Structs for programs as well as intermediate forms of programs
 * 
 */
use std::vec::Vec;

pub struct Program {
    pub instructions: Vec<u32>,
    pub data: Vec<u32>
}
impl Program {
    pub fn new() -> Self {
        Program {
            instructions: Vec::new(),
            data: Vec::new()
        }
    }
}

struct Protogram {
    text: Vec<Instruction>, 
    data: Vec<StaticData>
}

pub enum Instruction {
    RInstruction, IInstruction, JInstruction
}

pub struct RInstruction {
    label: String,
    opcode: u8,
    rs: u8,
    rt: u8,
    rd: u8,
    shamt: u8,
    func: u8
}
pub struct IInstruction {
    label: String,
    opcode: u8,
    rs: u8,
    rt: u8,
    immediate: i16,
    lblOp: String
}
pub struct JInstruction {
    label: String,
    opcode: u8,
    address: u32,
    lblOp: String
}

pub struct StaticData {
    label: String,
    value: Vec<i32>
}