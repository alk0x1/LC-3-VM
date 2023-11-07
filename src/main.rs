mod definitions;
mod instrucions;
mod hardware;
use definitions::Register;
use instrucions::add;
use hardware::Memory;

fn main() {
  let mut memory: Memory = Memory::new();

  memory.print_memory();
  memory.write(0x0234, 0x111);
  memory.print_memory();

}
