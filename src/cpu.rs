

const I_MEM_SIZE: i32 = 5;
const D_MEM_SIZE: i32 = 5;

/*
 *  
 * 
 */

type RegFile [i32; 32];
type IMem [i32, I_MEM_SIZE];
type DMem [i32, D_MEM_SIZE];
struct Cpu {
    registers: RegFile,
    instruction_memory: IMem,
    data_memory: DMem,
    program_counter: i32
}


impl Cpu {

    fn (&self) -> Cpu {
        Cpu cpu;



    }


}

