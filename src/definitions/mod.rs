// •Registers
// - temporary storage, accessed in a single machine cycle
// - accessing memory generally takes longer than a single cycle
// - eight general-purpose registers: R0 - R7
// - each 16 bits wide
// - how many bits to uniquely identify a register?
// - other registers
// - not directly addressable, but used by (and affected by) instructions
// - PC (program counter), condition codes

pub struct RegisterType {
  size: u16,
  value: u16
}

pub enum Register {
  R0(RegisterType),
  R1(RegisterType),
  R2(RegisterType),
  R3(RegisterType),
  R4(RegisterType),
  R5(RegisterType),
  R6(RegisterType),
  R7(RegisterType),
  PC(RegisterType),
  COND(RegisterType),
}

pub enum OperateInstructions{
  ADD,
  AND,
  NOT
}

pub enum DataMovement {
  // LOAD memory to register
  LD,
  LDR,
  LDI,
  //STORE register to memory
  ST,
  STR,
  STI,
  // LOAD EFFECTIVE ADDRESS
  LEA
}


pub struct ProgramCounter {
  value: u16,
}

impl ProgramCounter {
  pub fn new() -> Self {
      ProgramCounter { value: 0 }
  }

  pub fn read(&self) -> u16 {
      self.value
  }

  pub fn write(&mut self, value: u16) {
      self.value = value;
  }

  pub fn increment(&mut self) {
      self.value = self.value.wrapping_add(1);
  }

  // Additional methods for handling jumps, branching, etc.
}
