use std::collections::HashMap;

pub fn decode_instruction(instruction: u16) -> (u16, u16, u16) {
  // Define masks for each field
  let opcode_mask: u16 = 0b1111 << 12;
  let addressing_mode_mask: u16 = 0b111 << 9;
  let operand_mask: u16 = 0b111111111;

  // Extract each field using bit masks and shifting
  let opcode = (instruction & opcode_mask) >> 12;
  let addressing_mode = (instruction & addressing_mode_mask) >> 9;
  let operand = instruction & operand_mask;

  
  let unused = operand & 0b111;
  let register2 = (operand >> 3) & 0b111;
  let register1  = (operand >> 6) & 0b111;


  println!("opcode: {:04b}", opcode);
  println!("addressing_mode: {:03b}", addressing_mode);
  println!("operand: {:09b}", operand);

  println!("unused: {:03b}", unused);
  println!("register2: {:03b}", register2);
  println!("register1: {:03b}", register1);


  (opcode, addressing_mode, operand)
}

struct Instruction {
  opcode: u16,
  addressing_mode: u16,
  operand: u16,
  register_map: HashMap<u16, &'static str>
}

impl Instruction {
  pub fn decode_add(&self) {

    let unused = self.operand & 0b111;
    let register2 = (self.operand >> 3) & 0b111;
    let register1  = (self.operand >> 6) & 0b111;
  }

  pub fn decode_and(&self) {
  }

  pub fn decode_not(&self) {
  }

  pub fn decode_br(&self) {
  }

  pub fn decode_jmp(&self) {
  }

  pub fn decode_jsr(&self) {
  }

  pub fn decode_ld(&self) {
  }

  pub fn decode_ldr(&self) {
  }

  pub fn decode_ldi(&self) {
  }

  pub fn decode_lea(&self) {
  }

  pub fn decode_ret(&self) {
  }

  pub fn decode_rti(&self) {
  }

  pub fn decode_st(&self) {
  }

  pub fn decode_str(&self) {
  }

  pub fn decode_sti(&self) {
  }

  pub fn decode_trap(&self) {
  }

  pub fn decode_instruction(instruction: u16) -> (u16, u16, u16) {
    // Define masks for each field
    let opcode_mask: u16 = 0b1111 << 12;
    let addressing_mode_mask: u16 = 0b111 << 9;
    let operand_mask: u16 = 0b111111111;
  
    // Extract each field using bit masks and shifting
    let opcode = (instruction & opcode_mask) >> 12;
    let addressing_mode = (instruction & addressing_mode_mask) >> 9;
    let operand = instruction & operand_mask;
  
    

  
    (opcode, addressing_mode, operand)
  }
  
}