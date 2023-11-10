mod registers;
mod instrucions;
mod hardware;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use registers::Register;
use hardware::Memory;

fn main() {
  // less than 65535
  // let mut memory: Memory = Memory::new(65535);

  // memory.print_memory();
  // write the opcodes in memory
  // memory.write(0xAE50, 0x111);
  
  // execute_instruction(0001000100100011)
  read_file();
  // memory.print_memory();
  // println!("{:?}", memory.read(0xAE50));
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
