mod registers;
mod instrucions;
mod hardware;
mod parser;
mod hashmaps;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use parser::parse_instruction;
use registers::Register;
use hardware::Memory;
use hashmaps::{opcode_hashmap, register_hashmap};

fn main() {
  // less than 65535
  let mut memory: Memory = Memory::new(65535);
  let opcode_hashmap = opcode_hashmap();
  let register_hashmap = register_hashmap();
  let start_address = 0x3000;
  
  let instruction = parse_instruction("ADD R2, R3, R4", opcode_hashmap, register_hashmap);
  memory.write(start_address, instruction);
}


fn read_file() {
  if let Ok(file) = File::open("src/files_for_test/file.alk") {
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
      if let Ok(instruction) = line {
        println!("instruction {}", instruction);
        
      } else {
        println!("Error reading line from file");
      }
    }
  } else {
    println!("Error opening file");
  }
}
