


// fn parse_instruction(line: &str) -> Option<Instruction> {
//   let parts: Vec<&str> = line.trim().split_whitespace().collect();

//   if parts.len() < 2 {
//     return None; // Invalid instruction format
//   }

//   let opcode = match parts[0] {
//     "ADD" => Opcode::ADD,
//     "AND" => Opcode::AND,
//     "NOT" => Opcode::NOT,
//     "BR" => Opcode::BR,
//     "JMP" => Opcode::JMP,
//     "JSR" => Opcode::JSR,
//     "JSRR" => Opcode::JSRR,
//     "LD" => Opcode::LD,
//     "LDR" => Opcode::LDR,
//     "LDI" => Opcode::LDI,
//     "LEA" => Opcode::LEA,
//     "RET" => Opcode::RET,
//     "RTI" => Opcode::RTI,
//     "ST" => Opcode::ST,
//     "STR" => Opcode::STR,
//     "STI" => Opcode::STI,
//     "TRAP" => Opcode::TRAP,
//     _ => return None, 
//   };

//   let operands: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

//   println!("opcode: {:?}", opcode);
//   println!("operands: {:?}", operands);
//   Some(Instruction { opcode, operands })
// }


// fn execute_instruction(opcode: Opcode, registers: &mut [u16; 8]) {

//   // match 


//   match opcode {
//       Opcode::ADD(dr, sr1, sr2) => {
//           // Example: registers[dr as usize] = registers[sr1 as usize] + registers[sr2 as usize];
//       }
//       Opcode::AND(dr, sr1, sr2) => {
//           // Example: registers[dr as usize] = registers[sr1 as usize] & registers[sr2 as usize];
//       }
//       Opcode::NOT(dr, sr) => {
//           // Example: registers[dr as usize] = !registers[sr as usize];
//       }
//       Opcode::BR(n, pc_offset9) => {
//           // Example: if (registers[n as usize] & 0x8000) != 0 {
//           //              pc.jump(pc.get_address().wrapping_add((pc_offset9 as i16) << 7));
//           //          }
//       }
//       Opcode::JMP(base_r) => {
//           // Example: pc.jump(registers[base_r as usize]);
//       }
//       Opcode::JSR(pc_offset11) => {
//           // Example: pc.jump(pc.get_address().wrapping_add((pc_offset11 as i16) << 4));
//       }
//       Opcode::JSRR(base_r) => {
//           // Example: pc.jump(registers[base_r as usize]);
//       }
//       Opcode::LD(dr, pc_offset9) => {
//           // Example: registers[dr as usize] = memory.read(pc.get_address().wrapping_add(pc_offset9));
//       }
//       Opcode::LDR(dr, base_r, offset6) => {
//           // Example: registers[dr as usize] = memory.read(registers[base_r as usize].wrapping_add(offset6));
//       }
//       Opcode::LDI(dr, pc_offset9) => {
//           // Example: registers[dr as usize] = memory.read(memory.read(pc.get_address().wrapping_add(pc_offset9)));
//       }
//       Opcode::LEA(dr, pc_offset9) => {
//           // Example: registers[dr as usize] = pc.get_address().wrapping_add(pc_offset9);
//       }
//       Opcode::RET => {
//           // Example: pc.increment();
//       }
//       Opcode::RTI => {
//           // Handle RTI
//       }
//       Opcode::ST(sr, pc_offset9) => {
//           // Example: memory.write(pc.get_address().wrapping_add(pc_offset9), registers[sr as usize]);
//       }
//       Opcode::STR(sr, base_r, offset6) => {
//           // Example: memory.write(registers[base_r as usize].wrapping_add(offset6), registers[sr as usize]);
//       }
//       Opcode::STI(sr, pc_offset9) => {
//           // Example: memory.write(memory.read(pc.get_address().wrapping_add(pc_offset9)), registers[sr as usize]);
//       }
//       Opcode::TRAP(trapvect8) => {
//           // Handle TRAP
//       }
//   }
// }

