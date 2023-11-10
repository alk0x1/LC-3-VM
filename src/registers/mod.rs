// •Registers
// - temporary storage, accessed in a single machine cycle
// - accessing memory generally takes longer than a single cycle
// - eight general-purpose registers: R0 - R7
// - each 16 bits wide
// - how many bits to uniquely identify a register?
// - other registers
// - not directly addressable, but used by (and affected by) instructions
// - PC (program counter), condition codes

pub enum Register {
  R0 = 0b000,
  R1 = 0b001,
  R2 = 0b010,
  R3 = 0b011,
  R4 = 0b100,
  R5 = 0b101,
  R6 = 0b110,
  R7 = 0b111,
}

pub struct ProgramCounter {
  address: u16
}

impl ProgramCounter {
  pub fn new() -> Self {
    ProgramCounter { address: 0 }
  }

  pub fn read(&self) -> u16 {
    self.address
  }
  
  pub fn write(&mut self, address: u16) {
    self.address = address;
  }

  pub fn increment(&mut self) {
    self.address = self.address.wrapping_add(1);
  }
}
