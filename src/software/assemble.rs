use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use super::tokenize;



fn assemble(filepath: String) {

    let mut asm_file_string = read_file(filepath);
    let mut asm_tokens = tokenize::tokenize(asm_file_string);



}









// Read file into a mut String
fn read_file(filepath: String) -> String{
    let path = Path::new(&filepath);

    let mut file = match File::open(path){
        
        Ok(r) => r,
        Err(e) => panic!("couldn't open {}: {}", filepath, e)

    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => s,
        Err(_) => panic!("Couldn't read file {} to string", filepath)
    }
}






