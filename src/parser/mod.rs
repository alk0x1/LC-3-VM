use std::collections::HashMap;
use regex::Regex;

pub fn parse_instruction(line: &str, opcode_hashmap: HashMap<&str, u16>, register_hashmap: HashMap<&str, u16>) -> u16 {
  let re = Regex::new(r#"\s|,\s*"#).unwrap(); // Expressão regular para dividir por espaços ou vírgulas seguidas de espaços
  let parts: Vec<&str> = re.split(line).collect();

  let opcode =  opcode_hashmap.get(parts[0]).unwrap();
  let mut operands_binary: u16 = 0;
  for operand in parts[1..].iter() {
    if let Some(&bin) = register_hashmap.get(*operand) {
      operands_binary = (operands_binary << 3) | bin;
    }
  }

  let instruction: u16 = (opcode << 9) | operands_binary;

  instruction
}
