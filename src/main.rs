mod registers;
mod instructions;
mod hardware;
mod parser;
mod hashmaps;
use std::fs::File;
use std::io::{BufRead, BufReader};
use hardware::Memory;
use hashmaps::{opcode_hashmap, register_hashmap, register_values_hashmap};
use instructions::Instruction;
use parser::parse_mnemonic;

fn main() {
  // less than 65535

  // let register_values_hashmap = register_values_hashmap();
  

  read_file_and_insert_instructions_in_memory("src/files_for_test/file.alk");

  // println!("{:016b}", memory.read(start_address));
  
  // instrucions::decode_instruction(0b001010011100000);

  // let mut instr = Instruction::new(0b101010011100000, register_values_hashmap);
  // instr.execute_instruction();

}


fn read_file_and_insert_instructions_in_memory(path: &str) {
  let mut memory: Memory = Memory::new(65535);
  let mut address = 0x3000;
  let opcode_hashmap = opcode_hashmap();
  let register_hashmap = register_hashmap();

  if let Ok(file) = File::open(path) {
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
      if let Ok(instruction) = line {
        let instruction_binaries = parse_mnemonic(&instruction, &opcode_hashmap, &register_hashmap);
        memory.write(address, instruction_binaries);
        println!("address, value: {:?}, {:?}", address, memory.read(address));
        address = address.wrapping_add(1);  // Incrementing the memory address
      } else {
        println!("Error reading line from file");
      }
    }
  } else {
    println!("Error opening file");
  }
}
