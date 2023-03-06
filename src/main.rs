use std::collections::HashMap;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct CpuData {
    instruction_length: usize,
    program_memory_lines: usize
}

#[derive(Debug, Deserialize)]
struct Instruction {
    opcode: String,
    operands: Vec<String>,
    keywords: Vec<String>
}

#[derive(Debug, Deserialize)]
struct ISA {
    cpu_data: CpuData,
    define: HashMap<String, HashMap<String, String>>,
    instructions: HashMap<String, Instruction>
}

fn deserialize_json(file_name: &str) -> serde_json::Result<ISA> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Couldn't read to string");
    return serde_json::from_str(&contents);
}

fn main() {
    let isa: serde_json::Result<ISA> = deserialize_json("AnPUNano.json");
    if isa.is_ok() {
        println!("{:#?}", isa.unwrap());
    }
    else {
        println!("JSON: {}", isa.err().unwrap());
    }
}
