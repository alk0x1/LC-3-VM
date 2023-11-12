use core::fmt;

pub struct Memory {
  memory: Vec<MemoryContent>
}

#[derive(Debug, Clone)]
pub struct MemoryContent {
  value: u16,
  address: u16
}

impl Memory {
  pub fn new(size: usize) -> Self {
    let mut memory = Vec::with_capacity(size);
    for address in 0..size as u16 {
      memory.push(MemoryContent {
        value: 0,
        address,
      });
    }
    println!("Memory initialized with {} locations", size);

    Memory { memory }
  }

  pub fn read(&self, address: u16) -> u16 {
    if let Some(content) = self.memory.iter().find(|&content| content.address == address) {
      content.value
    } else {
      panic!("Memory read out of bounds");
    }
  }

  pub fn write(&mut self, address: u16, value: u16) {
    if let Some(content) = self.memory.iter_mut().find(|content| content.address == address) {
      content.value = value;
    } else {
      panic!("Memory write out of bounds");
    }
  }

  pub fn print_memory(&self) {
    for content in &self.memory {
      println!("Memory at address 0x{:04X}: {}", content.address, content.value);
    } 
  }
}

impl fmt::Debug for Memory {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Memory:\n{:#?}", self.memory)
  }
}