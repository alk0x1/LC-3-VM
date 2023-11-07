pub struct Memory {
  memory: Vec<MemoryContent>
}

#[derive(Debug)]
pub struct MemoryContent {
  value: u16,
  address: u16
}


impl Memory {
  pub fn new() -> Self {
    Self {
      memory: Vec::new(),
    }
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
    println!("{:?}", self.memory);
  }
}
