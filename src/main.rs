use hardware::arch::Computer;

use hardware::cpu::CPU as CPU;
use crate::datatypes::Program;

mod datatypes;
mod hardware;
mod software;

fn main() {
    // Create a sample program and load it into memory
    let mut program = crate::datatypes::Program::new();
    test_cpu(&mut program);
}

fn test_cpu(program: &mut Program) {
    program.instructions.push(0x24080005);
    program.instructions.push(0x01084820);
    program.instructions.push(0x0109502a);
    program.instructions.push(0x290b0000);
    program.instructions.push(0x290c0009);
    program.instructions.push(0x15600002);
    program.instructions.push(0x15200003);
    program.instructions.push(0x11800003);
    program.instructions.push(0x2401ffff);
    program.instructions.push(0x0800100b);
    program.instructions.push(0x24020007);
    program.instructions.push(0xffffffff);

    let mut cpu = CPU::new();
    cpu.load_program(program.clone());
    cpu.start();
}

