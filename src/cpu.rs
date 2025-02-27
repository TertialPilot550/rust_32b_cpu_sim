use crate::arch::Computer;
use crate::arch::DYNAMIC_DATA;
use crate::arch::END_MEM;
use crate::arch::REG_NUM;
use crate::arch::MEM_SIZE;
use crate::arch::PC_START;
use crate::arch::MipsIsa;
use crate::arch::STATIC_DATA;
use crate::program::Program;


/**
 * CPU implementation of the architecture modifiers
 * sammc
 */

struct CPU {
    pub debug_mode: bool,
    pub registers: [i32; REG_NUM as usize],    // Registers
    pub memory: [u8; MEM_SIZE as usize],       // Memory
    pub program_counter: u32                   // Program Counter
}

// CPU Implementation
impl CPU {

    // Constructor using definitions from the arch module
    pub fn new() -> Self {
        CPU {
            debug_mode: true,
            registers: [0; REG_NUM as usize],
            memory: [0; MEM_SIZE as usize],
            program_counter: PC_START as u32
        }
    }


    fn fetch_decode_execute_loop(&mut self) {
        let mut cycle_count: u32 = 0;
        while self.program_counter < STATIC_DATA {
            // Fetch instruction
            let instruction: u32 = self.read_word_from_mem(self.program_counter);

            if instruction == 0xffff_ffff {
                break;
            }

            if self.debug_mode { 
                println!("CYCLE::{} INSTRUCTION::{:#x}", cycle_count, instruction);
            }

            self.decode_execute(instruction);
            self.program_counter += 4;
            cycle_count += 1;
        }


        if self.debug_mode {
            CPU::print_state();
            CPU::print_memory_state();
        }

    }

    

    fn print_state() {

    }
    
    fn print_memory_state() {

    }


    pub fn load_memory(&mut self, address: u32, payload: Vec<u32>) {
        if address % 4 != 0 {
            return ();
        }

        if address + (payload.len() * 4) as u32 > END_MEM {
            return ();
        }

        let mut offset = 0;
        for data in payload {
            self.write_word_to_mem(address + (offset), data);
            offset += 4;
        }
    }

    pub fn read_word_from_mem(&self, address: u32) -> u32 {
        // must be word boundary
        if address % 4 != 0 {
            return 0;
        }
        // construct word
        let mut acc: u32 = 0;
        acc += (self.memory[address as usize] as u32) << 24;
        acc += (self.memory[(address+1) as usize] as u32) << 16;
        acc += (self.memory[(address+2) as usize] as u32) << 8;
        acc += (self.memory[(address+3) as usize] as u32);
        acc
    }

    pub fn write_word_to_mem(&mut self, address: u32, value: u32) {
        // must be word boundary
        if address % 4 != 0 {
            return ();
        }

        // split the word into bytes
        let msb: u8 = (value >> 24) as u8;
        let higher: u8 = ((value >> 16) & 0xFF) as u8;
        let lower: u8 = ((value >> 8) & 0xFF) as u8;
        let lsb: u8 = ((value) & 0xFF) as u8;

        // store each byte
        self.memory[(address) as usize] = msb;
        self.memory[(address + 1) as usize] = higher;
        self.memory[(address + 2) as usize] = lower;
        self.memory[(address + 3) as usize] = lsb;
    }

}


impl Computer for CPU {
    fn load_program(&mut self, program: Program) {
        let max_program_size: usize = (STATIC_DATA - PC_START) as usize;
        let max_static_data_size: usize = (DYNAMIC_DATA - STATIC_DATA) as usize;

        // if either the program or the data is too long, return with no action
        if program.instructions.len() > max_program_size || program.data.len() > max_static_data_size {
            return ();
        }

        // Load Instructions
        self.load_memory(PC_START, program.instructions);
        // Load Static Data
        self.load_memory(STATIC_DATA, program.data);
    }

    fn start(&mut self) {
        self.fetch_decode_execute_loop();
    }
}

impl MipsIsa for CPU {
    fn add(&mut self, rs: u32, rt: u32, rd: u32) {
        self.registers[rd as usize] = self.registers[rs as usize] + self.registers[rt as usize];

        // TODO overflow

    }

    fn addu(&mut self, rs: u32, rt: u32, rd: u32) {
        self.registers[rd as usize] = self.registers[rs as usize] + self.registers[rt as usize];
    }

    fn and(&mut self, rs: u32, rt: u32, rd: u32) {
        self.registers[rd as usize] = self.registers[rs as usize] & self.registers[rt as usize];

    }

    fn jr(&mut self, rs: u32) {
        self.program_counter = self.registers[rs as usize] as u32;
    }

    fn nor(&mut self, rs: u32, rt: u32, rd: u32) {
        self.registers[rd as usize] = !(self.registers[rs as usize] | self.registers[rt as usize]);

    }

    fn or(&mut self, rs: u32, rt: u32, rd: u32) {
        self.registers[rd as usize] = self.registers[rs as usize] | self.registers[rt as usize];

    }

    fn slt(&mut self, rs: u32, rt: u32, rd: u32) {
        let cmp = self.registers[rs as usize] < self.registers[rt as usize];
        self.registers[rd as usize] = if cmp {1} else {0};
    }

    fn sltu(&mut self, rs: u32, rt: u32, rd: u32) {
        let cmp = (self.registers[rs as usize] as u32) < (self.registers[rt as usize] as u32);
        self.registers[rd as usize] = if cmp {1} else {0};
    }

    fn sll(&mut self, rs: u32, rt: u32, shamt: u32) {
        self.registers[rt as usize] = self.registers[rs as usize] << shamt;

    }

    fn srl(&mut self, rs: u32, rt: u32, shamt: u32) {
        self.registers[rt as usize] = self.registers[rs as usize] >> shamt;
    }

    fn sub(&mut self, rs: u32, rt: u32, rd: u32) {
        self.registers[rd as usize] = self.registers[rs as usize] - self.registers[rt as usize];
    }

    fn subu(&mut self, rs: u32, rt: u32, rd: u32) {
        self.registers[rd as usize] = (self.registers[rs as usize]) - (self.registers[rt as usize]); // TODO
    }

    fn addi(&mut self, rs: u32, rt: u32, immediate: i16) {
        self.registers[rt as usize] = self.registers[rs as usize] + immediate as i32; // TODO OVERFLOW

    }

    fn addiu(&mut self, rs: u32, rt: u32, immediate: i16) {
        self.registers[rt as usize] = self.registers[rs as usize] + immediate as i32;
    }

    fn andi(&mut self, rs: u32, rt: u32, immediate: i16) {
        todo!()
    }

    fn ori(&mut self, rs: u32, rt: u32, immediate: i16) {
        todo!()
    }

    fn beq(&mut self, rs: u32, rt: u32, immediate: u16) {
        todo!()
    }

    fn bne(&mut self, rs: u32, rt: u32, immediate: u16) {
        todo!()
    }

    fn lbu(&mut self, rs: u32, rt: u32, immediate: i16) {
        todo!()
    }

    fn lhu(&mut self, rs: u32, rt: u32, immediate: i16) {
        todo!()
    }

    fn lui(&mut self, rs: u32, immediate: i16) {
        todo!()
    }

    fn lw(&mut self, rs: u32, rt: u32, immediate: i16) {
        todo!()
    }

    fn sb(&mut self, rs: u32, rt: u32, immediate: i16) {
        todo!()
    }

    fn sh(&mut self, rs: u32, rt: u32, immediate: i16) {
        todo!()
    }

    fn sw(&mut self, rs: u32, rt: u32, immediate: i16) {
        todo!()
    }

    fn slti(&mut self, rs: u32, rt: u32, immediate: i16) {
        todo!()
    }

    fn sltiu(&mut self, rs: u32, rt: u32, immediate: u16) {
        todo!()
    }

    fn j(&mut self, address: u32) {
        todo!()
    }

    fn jal(&mut self, address: u32) {
        todo!()
    }
}

