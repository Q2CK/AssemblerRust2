use std::collections::HashMap;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct CpuData {
    cpu_name: String,
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

struct Error {
    file: String,
    line: Option<u32>,
    message: String
}

struct AssemblerResult{
    successes: Vec<String>,
    fails: Vec<Error>
}

impl AssemblerResult {
    fn report(&self) {
        if self.fails.len() == 0 {
            for element in &self.successes {
                println!("{}", element);
            }
        }
        else {
            for element in &self.fails {
                match element.line {
                    Some(nr) => println!(r#"File "{}", line {}: {}"#, element.file, nr, element.message),
                    None => println!(r#"File "{}": {}"#, element.file, element.message)
                }
            }
        }
    }
}

fn deserialize_json(file_name: &str) -> serde_json::Result<ISA> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read to string");
    return serde_json::from_str(&contents);
}

pub fn assemble() {
    loop {
        let isa_file_name = String::from("AnPUNano.json");
        let isa;
        let isa_result: serde_json::Result<ISA> = deserialize_json(&isa_file_name);

        let mut assembler_result = AssemblerResult {
            successes: Vec::new(),
            fails: Vec::new()
        };

        if isa_result.is_ok() {
            isa = isa_result.unwrap();
        }
        else {
            let serde_error_msg = isa_result.err().unwrap();
            assembler_result.fails.push(Error {
                file: isa_file_name,
                line: None,
                message: format!("Invalid .json ISA specification - {}", serde_error_msg)
            });
        }
    }
}