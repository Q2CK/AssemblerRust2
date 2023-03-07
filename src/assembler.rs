use std::collections::HashMap;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::Read;

const ISA_ERR_MSG: &str = "Couldn't read isa file";
const ASM_ERR_MSG: &str = "Couldn't read asm file";

#[allow(dead_code)]
#[derive(/*Debug,*/ Deserialize)]
struct CpuData {
    cpu_name: String,
    instruction_length: usize,
    program_memory_lines: usize
}

#[allow(dead_code)]
#[derive(/*Debug,*/ Deserialize)]
struct Instruction {
    opcode: String,
    operands: Vec<String>,
    keywords: Vec<String>
}

#[allow(dead_code)]
#[derive(/*Debug,*/ Deserialize)]
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
    info: Vec<String>,
    fails: Vec<Error>
}

impl AssemblerResult {
    fn report(&self) {
        match self.fails.len(){
            0 =>
                for element in &self.info {
                    println!("{}", element);
                },
            _ =>
                for element in &self.fails {
                    match element.line {
                        Some(nr) => println!(r#"File "{}", line {}: {}"#, element.file, nr, element.message),
                        None => println!(r#"File "{}": {}"#, element.file, element.message)
                    }
                }
        }
    }
}

fn deserialize_json_file(file_name: &str) -> Result<ISA, String> {
    let mut file;
    match File::open(file_name) {
        Ok(v) => file = v,
        Err(_) => return Err(ISA_ERR_MSG.to_string())
    }

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(ISA_ERR_MSG);

    match serde_json::from_str(&contents) {
        Ok(v) => Ok(v),
        Err(_) => Err(ISA_ERR_MSG.to_string())
    }
}

pub fn assemble() {
    loop {
        let mut assembler_result = AssemblerResult {
            info: Vec::new(),
            fails: Vec::new()
        };

        let isa_file_name = String::from("AnPUNano.json");
        let isa;
        let isa_result = deserialize_json_file(&isa_file_name);

        let assembly_file_name = String::from("test.asm");
        let assembly;

        match File::open(&assembly_file_name) {
            Ok(v) => assembly = v,
            Err(e) => assembler_result.fails.push(Error {
                file: assembly_file_name,
                line: None,
                message: ASM_ERR_MSG.to_string()
            })
        }

        if isa_result.is_ok() {
            isa = isa_result.unwrap();
            assembler_result.info.push(isa.cpu_data.cpu_name);
            assembler_result.info.push("------------------------".to_string());
        }
        else {
            let serde_error_msg = isa_result.err().unwrap();
            assembler_result.fails.push(Error {
                file: isa_file_name,
                line: None,
                message: format!("Invalid ISA specification - {}", serde_error_msg)
            });
            assembler_result.report();
            break;
        }



        assembler_result.report();
        break;
    }
}