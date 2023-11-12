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
  let mut memory: Memory = Memory::new(65535);
  memory = read_file_and_insert_instructions_in_memory("src/files_for_test/file.alk", memory);
  let register_values_hashmap = register_values_hashmap();

  let bin_instr = fetch_instructions_in_memory(&mut 0x3000, &memory);
  let mut instr: Instruction = Instruction::new(bin_instr, register_values_hashmap);
  instr.execute_instruction();
}


fn fetch_instructions_in_memory(pc: &mut u16, memory: &Memory) -> u16 {
  let instruction = memory.read(*pc);
  *pc += 1;

  instruction
}


fn read_file_and_insert_instructions_in_memory(path: &str, mut memory: Memory) -> Memory {
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
    return memory;
  } else {
    println!("Error opening file");

    memory
  }
}
