use std::collections::HashMap;
use regex::Regex;

pub fn parse_mnemonic(line: &str, opcode_hashmap: &HashMap<&str, u16>, register_hashmap: &HashMap<&str, u16>) -> u16 {
  let re = Regex::new(r#"\s|,\s*"#).unwrap(); // Expressão regular para dividir por espaços ou vírgulas seguidas de espaços
  let parts: Vec<&str> = re.split(line).collect();
  
  match opcode_hashmap.get(parts[0]) {
    Some(opcode) => {
      println!("parts: {:?}", parts);

      let mut operands_binary: u16 = 0;
      for operand in parts[1..].iter() {
        if let Some(&bin) = register_hashmap.get(*operand) {
          operands_binary = (operands_binary << 3) | bin;
        }
      }
      let instruction: u16 = (opcode << 12) | (operands_binary << 3); // Shift the operands to the left to create the desired format
      
      instruction
    }
    None => {
      println!("Empty mnemonic");
      0
    }
  }
}