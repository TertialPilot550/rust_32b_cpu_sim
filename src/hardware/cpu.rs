use super::{arch, datatypes::Program};

/**
 * CPU implementation of the architecture defintions
 * sammc
 */

pub struct CPU {
    pub debug_mode: bool,
    pub registers: [i32; arch::REG_NUM as usize],    // Registers
    pub memory: [u8; arch::MEM_SIZE as usize],       // Memory
    pub program_counter: u32                   // Program Counter
}

// CPU Implementation
impl CPU {

    // Constructor using definitions from the arch module
    pub fn new() -> Self {
        let res = CPU {
            debug_mode: true,
            registers: [0; arch::REG_NUM as usize],
            memory: [0; arch::MEM_SIZE as usize],
            program_counter: arch::PC_START as u32
        };
        if res.debug_mode { res.print_state() };
        res
    }


    fn fetch_decode_execute_loop(&mut self) {
        let mut cycle_count: u32 = 1;
        while self.program_counter < arch::STATIC_DATA {
            // Fetch instruction
            let instruction: u32 = self.read_word_from_mem(self.program_counter);

            if self.debug_mode { 
                print!("CYCLE::{:03} INSTRUCTION::{:#010x}  ", cycle_count, instruction);
                if cycle_count % 4 == 0 { println!(); }
            }

            if instruction == 0xFFFFFFFF as u32 {
                break;
            }

            arch::MipsIsa::decode_execute(self, instruction);
            self.registers[0] = 0; // ensure zero register is 0
            self.program_counter += 4;
            cycle_count += 1;
        }

        if self.debug_mode {
            println!();
            self.print_state();
        }
    }

    
    // pc and registers
    fn print_state(&self) {
        println!("-----------------------------------------------------------------------------------------------------------------------------------------------");
        println!("                                                               PC {:#010x}                                      ", self.program_counter);
        println!("-----------------------------------------------------------------------------------------------------------------------------------------------");
        let width = 8;
        let height = 4;

        for i in 0..height {
            for j in 0..width {
                let reg_num: u32 = (width*i) + j;
                print!("R[{:02}]: {:#010x} ", reg_num, self.registers[reg_num as usize]);
            }
            println!();
        }
        println!("-----------------------------------------------------------------------------------------------------------------------------------------------");
        self.print_memory_contents();
        println!("-----------------------------------------------------------------------------------------------------------------------------------------------");

    }
    
    // prints a configured amount of memory content
    fn print_memory_contents(&self) {
        let width = 11;
        let height = 11;

        let mut addr: u32 = arch::PC_START;
        println!("Starting @ {:#010x}", arch::PC_START);
        for i in 0..height {
            print!("ADDR:{:#010x}      |", addr);
            for j in 0..width {
                print!("{:#010x}|", self.read_word_from_mem(addr));
                addr += 4;
            }
            println!("");

        }

    }


    pub fn load_memory(&mut self, address: u32, payload: Vec<u32>) {
        if address % 4 != 0 {
            return ();
        }

        if address + (payload.len() * 4) as u32 > arch::END_MEM {
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
        acc += self.memory[(address+3) as usize] as u32;
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


impl arch::Computer for CPU {
    fn load_program(&mut self, program: Program) {
        let max_program_size: usize = (arch::STATIC_DATA - arch::PC_START) as usize;
        let max_static_data_size: usize = (arch::DYNAMIC_DATA - arch::STATIC_DATA) as usize;

        // if either the program or the data is too long, return with no action
        if program.instructions.len() > max_program_size || program.data.len() > max_static_data_size {
            return ();
        }

        // Load Instructions
        self.load_memory(arch::PC_START, program.instructions);
        // Load Static Data
        self.load_memory(arch::STATIC_DATA, program.data);
    }

    fn start(&mut self) {
        self.fetch_decode_execute_loop();
    }
}

impl arch::MipsIsa for CPU {
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
        self.registers[rt as usize] = self.registers[rs as usize] & immediate as i32;
    }

    fn ori(&mut self, rs: u32, rt: u32, immediate: i16) {
        self.registers[rt as usize] = self.registers[rs as usize] | immediate as i32;

    }

    fn beq(&mut self, rs: u32, rt: u32, immediate: i16) {
        if self.registers[rs as usize] == self.registers[rt as usize] { 
            self.program_counter = (self.program_counter as i64 + (immediate*4) as i64) as u32;
        }
    }

    fn bne(&mut self, rs: u32, rt: u32, immediate: i16) {
        if self.registers[rs as usize] != self.registers[rt as usize] { 
            self.program_counter = (self.program_counter as i64 + (immediate*4) as i64) as u32;
        }
    }

    fn lbu(&mut self, rs: u32, rt: u32, immediate: i16) {
        self.registers[rt as usize] = self.memory[(self.registers[rs as usize] + immediate as i32) as usize] as i32;
    }

    fn lhu(&mut self, rs: u32, rt: u32, immediate: i16) {
        // if not on a half word boundary, fail
        if (self.registers[rs as usize] + immediate as i32) % 2 != 0 {
            return ();
        }

        self.registers[rt as usize] = self.memory[(self.registers[rs as usize] + immediate as i32) as usize] as i32;
        self.registers[rt as usize] = self.memory[(self.registers[rs as usize] + immediate as i32 + 1) as usize] as i32;
    }

    fn lui(&mut self, rt: u32, immediate: i16) {
        self.registers[rt as usize] = (immediate as i32) << 16;
    }

    fn lw(&mut self, rs: u32, rt: u32, immediate: i16) {
        self.registers[rt as usize] = self.read_word_from_mem((rs as i32 + immediate as i32) as u32) as i32;
    }

    fn sb(&mut self, rs: u32, rt: u32, immediate: i16) {
        self.memory[(rt + immediate as u32) as usize] = (self.registers[rs as usize] & 0xFF) as u8;
    }

    fn sh(&mut self, rs: u32, rt: u32, immediate: i16) {
        // if not on a half word boundary, fail
        if (self.registers[rs as usize] + immediate as i32) % 2 != 0 {
            return ();
        }


        let val: i32 = self.registers[rs as usize] & 0xFFFF; 

        self.memory[(self.registers[rt as usize] + immediate as i32) as usize] = (val >> 8) as u8;
        self.memory[(self.registers[rt as usize] + immediate as i32 + 1) as usize] = (val & 0xFF) as u8;
    }

    fn sw(&mut self, rs: u32, rt: u32, immediate: i16) {
        self.write_word_to_mem((self.registers[rt as usize] + immediate as i32) as u32, self.registers[rs as usize] as u32);
    }

    fn slti(&mut self, rs: u32, rt: u32, immediate: i16) {
        if self.registers[rs as usize] < immediate as i32 {
            self.registers[rt as usize] = 1;
            return ();
        }
        self.registers[rt as usize] = 0;
    }

    fn sltiu(&mut self, rs: u32, rt: u32, immediate: u16) {
        if (self.registers[rs as usize] as u32) < (immediate as u32) {
            self.registers[rt as usize] = 1;
            return ();
        }
        self.registers[rt as usize] = 0;
    }

    fn j(&mut self, address: u32) {
        let addr_real = (address & 0xFF_FFFF) << 2; // ensure a 24 bit number, append 2 zeros
        self.program_counter = addr_real | (self.program_counter & 0xF000_0000); // borrow 4 msb from pc
    }

    fn jal(&mut self, address: u32) {
        self.registers[31] = self.program_counter as i32; // ra
        let addr_real = (address & 0xFF_FFFF) << 2; // ensure a 24 bit number, append 2 zeros
        self.program_counter = addr_real | (self.program_counter & 0xF000_0000); // borrow 4 msb from pc
    }
}

