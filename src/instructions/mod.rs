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

  
  // let unused = operand & 0b111;
  // let register2 = (operand >> 3) & 0b111;
  // let register1  = (operand >> 6) & 0b111;

  println!("opcode: {:04b}", opcode);
  println!("addressing_mode: {:03b}", addressing_mode);
  println!("operand: {:09b}", operand);


  (opcode, addressing_mode, operand)
}

pub struct Instruction {
  opcode: u16,
  addressing_mode: u16,
  operand: u16,
  register_value_map: HashMap<u16, u16>
}

impl Instruction {
  pub fn new(binary: u16, register_value_map: HashMap<u16, u16>) -> Instruction {
    let (opcode, addressing_mode, operand) = decode_instruction(binary);
    Instruction {
      opcode,
      addressing_mode,
      operand,
      register_value_map
    }
  }

  pub fn execute_instruction(&mut self) {
    match self.opcode {
      0b0001 => self.decode_add(),
      0b0101 => self.decode_and(),
      0b1001 => self.decode_not(),
      0b0000 => self.decode_br(),
      0b1100 => self.decode_jmp(),
      0b0100 => {
          // Handle JSR or JSRR, since they have the same opcode
          // Example: self.decode_jsr();
      }
      0b0010 => self.decode_ld(),
      0b0110 => self.decode_ldr(),
      0b1010 => self.decode_ldi(),
      0b1110 => self.decode_lea(),
      // 0b1100 => self.decode_ret(),
      0b1000 => self.decode_rti(),
      0b0011 => self.decode_st(),
      0b0111 => self.decode_str(),
      0b1011 => self.decode_sti(),
      0b1111 => self.decode_trap(),
      _ => println!("Unknown opcode"),
    }
  }

  pub fn decode_add(&mut self) {
    let register1  = (self.operand >> 6) & 0b111;
    let register2 = (self.operand >> 3) & 0b111;
    let register1_val = self.register_value_map.get(&register1).unwrap();
    let register2_val = self.register_value_map.get(&register2).unwrap();

    println!("register1_val {:?}", register1_val);
    println!("register2_val {:?}", register2_val);

    let sum = register1_val + register2_val;
    self.register_value_map.insert(self.addressing_mode, sum);
    println!("addressing_mode - value: {:?}", self.register_value_map.get(&self.addressing_mode));
  }

  fn decode_and(&mut self) {
    let register1 = (self.operand >> 6) & 0b111;
    let register2 = (self.operand >> 3) & 0b111;
    let register1_val = self.register_value_map.get(&register1).unwrap();
    let register2_val = self.register_value_map.get(&register2).unwrap();

    println!("register1_val {:?}", register1_val);
    println!("register2_val {:?}", register2_val);

    // Perform AND operation
    let result = register1_val & register2_val;

    // Update registers or flags
    self.register_value_map.insert(self.addressing_mode, result);
    println!("addressing_mode - value: {:?}", self.register_value_map.get(&self.addressing_mode));
  }

  fn decode_not(&mut self) {
    let source_register = (self.operand >> 6) & 0b111; // Extract source register
    let destination_register = (self.operand >> 9) & 0b111; // Extract destination register

    let source_register_value = *self.register_value_map.get(&source_register).unwrap();
    let inverted_value = !source_register_value; // Invert the bits

    self.register_value_map.insert(destination_register, inverted_value); // Store the result in the destination register

    println!("Source Register Value: {}", source_register_value);
    println!("Inverted Value: {}", inverted_value);
    println!("Stored in Destination Register: {}", destination_register);
  }

  fn decode_br(&self) {
  }

  fn decode_jmp(&self) {
  }

  fn decode_jsr(&self) {
  }

  fn decode_ld(&self) {
  }

  fn decode_ldr(&self) {
  }

  fn decode_ldi(&self) {
  }

  fn decode_lea(&self) {
  }

  fn decode_ret(&self) {
  }

  fn decode_rti(&self) {
  }

  fn decode_st(&self) {
  }

  fn decode_str(&self) {
  }

  fn decode_sti(&self) {
  }

  fn decode_trap(&self) {
  }

  fn decode_instruction(&self, instruction: u16) -> (u16, u16, u16) {
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

fn get_key_by_value<'a>(map: &'a HashMap<&'a str, u16>, value: u16) -> Option<&'a str> {
  for (key, &val) in map {
    if val == value {
      return Some(key);
    }
  }
  None
}