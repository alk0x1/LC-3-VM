use std::collections::HashMap;

pub fn opcode_hashmap() -> HashMap<&'static str, u16> {
    let mut opcode_map = HashMap::new();
    opcode_map.insert("ADD", 0b0001);
    opcode_map.insert("AND", 0b0101);
    opcode_map.insert("NOT", 0b1001);
    opcode_map.insert("BR", 0b0000);
    opcode_map.insert("JMP", 0b1100);
    opcode_map.insert("JSR", 0b0100);
    opcode_map.insert("JSRR", 0b0100);
    opcode_map.insert("LD", 0b0010);
    opcode_map.insert("LDR", 0b0110);
    opcode_map.insert("LDI", 0b1010);
    opcode_map.insert("LEA", 0b1110);
    opcode_map.insert("RET", 0b1100);
    opcode_map.insert("RTI", 0b1000);
    opcode_map.insert("ST", 0b0011);
    opcode_map.insert("STR", 0b0111);
    opcode_map.insert("STI", 0b1011);
    opcode_map.insert("TRAP", 0b1111);

    opcode_map
}

pub fn register_hashmap() -> HashMap<&'static str, u16> {
    let mut register_map: HashMap<&str, u16> = HashMap::new();
    register_map.insert("R0", 0b000);
    register_map.insert("R1", 0b001);
    register_map.insert("R2", 0b010);
    register_map.insert("R3", 0b011);
    register_map.insert("R4", 0b100);
    register_map.insert("R5", 0b101);
    register_map.insert("R6", 0b110);
    register_map.insert("R7", 0b111);

    register_map
}

pub fn register_values_hashmap() -> HashMap<u16, u16> {
    let mut register_map = HashMap::new();
    register_map.insert( 0b000, 0);
    register_map.insert( 0b001, 0);
    register_map.insert( 0b010, 0);
    register_map.insert( 0b011, 0);
    register_map.insert( 0b100, 0);
    register_map.insert( 0b101, 0);
    register_map.insert( 0b110, 0);
    register_map.insert( 0b111, 0);

    register_map
}