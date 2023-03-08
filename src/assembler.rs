use std::collections::HashMap;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::{Read, stdin};

const ISA_VALIDATION_ERR_MSG: &str = "Invalid ISA file structure";
const ISA_READ_ERR_MSG: &str = "Couldn't read ISA file";
const ASM_ERR_MSG: &str = "Couldn't read ASM file";

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CpuData {
    cpu_name: String,
    instruction_length: usize,
    program_memory_lines: usize
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Instruction {
    opcode: String,
    operands: Vec<String>,
    keywords: Vec<String>
}

#[allow(dead_code)]
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

impl Error {
    fn no_line(file: &String, message: String) -> Error {
        Error {
            file: file.to_string(), line: None, message
        }
    }

    fn in_line(file: &String, line: &usize, message: &String) -> Error {
        Error {
            file: file.to_string(), line: Some(*line as u32), message: message.to_string()
        }
    }
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

fn deserialize_json_file(file_name: &String) -> Result<ISA, String> {
    let mut file = match File::open(file_name) {
        Ok(v) => v,
        Err(_) => return Err(ISA_READ_ERR_MSG.to_string())
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => return Err(ISA_READ_ERR_MSG.to_string())
    }

    match serde_json::from_str(&contents) {
        Ok(v) => Ok(v),
        Err(_) => Err(ISA_VALIDATION_ERR_MSG.to_string())
    }
}

fn read_assembly(file_name: &String) -> Result<String, String> {
    let mut file = match File::open(file_name) {
        Ok(v) => v,
        Err(_) => return Err(ASM_ERR_MSG.to_string())
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(_) => Err(ASM_ERR_MSG.to_string())
    }
}

fn open_files(isa: &mut Option<ISA>, asm: &mut String, assembler_result: &mut AssemblerResult) -> (String, String) {
    let mut isa_file_name = String::new();
    println!("\nISA file name: ");
    stdin().read_line(&mut isa_file_name).unwrap();
    isa_file_name = format!("ISA/{}", isa_file_name[0..isa_file_name.len() - 1].to_string());
    let isa_result = deserialize_json_file(&isa_file_name);

    let mut asm_file_name = String::new();
    println!("ASM file name: ");
    stdin().read_line(&mut asm_file_name).unwrap();
    asm_file_name = format!("ASM/{}", asm_file_name[0..asm_file_name.len() - 1].to_string());
    let asm_result = read_assembly(&asm_file_name);

    match asm_result {
        Ok(v) => *asm = v,
        Err(e) => assembler_result.fails.push(Error::no_line(&asm_file_name, e))
    }

    match isa_result {
        Ok(v) => {
            assembler_result.info.push(v.cpu_data.cpu_name.clone());
            assembler_result.info.push("------------------------".to_string());
            *isa = Some(v);
        }
        Err(e) => {
            assembler_result.fails.push(Error::no_line(&isa_file_name, e));
            *isa = None;
        }
    }

    return (isa_file_name, asm_file_name)
}

pub fn assemble() {
    loop {
        let mut assembler_result = AssemblerResult {
            info: Vec::new(),
            fails: Vec::new()
        };

        let mut isa = None;
        let mut asm = String::new();

        let (isa_file_name, asm_file_name) = open_files(&mut isa, &mut asm, &mut assembler_result);

        if assembler_result.fails.len() != 0 {
            assembler_result.report();
            continue;
        }

        let isa = isa.unwrap();

        let mut asm_lines: Vec<String> = asm.split("\n").map(str::to_string).collect();
        for i in 0..asm_lines.len() {
            let line = asm_lines[i].trim();
            let mut tokens: Vec<&str> = line.split(|c| c == ',' || c == ' ').collect();

            let mut j = 0;
            while j < tokens.len() {
                if tokens[j] == "" {
                    tokens.remove(j);
                }
                else {
                    j += 1;
                }
            }

            println!("{:?}", tokens);

            let mnemonic = tokens[0];

            if isa.instructions.contains_key(mnemonic) {
                let instruction: &Instruction = &isa.instructions[mnemonic];

                let mut binary = instruction.opcode.clone();

                for j in 1..tokens.len() {
                    let operand = tokens[j].trim();

                    let operand_parsed: usize;
                    match operand.parse() {
                        Ok(v) => operand_parsed = v,
                        Err(_) => {
                            assembler_result.fails.push(Error::in_line(&asm_file_name, &i,
                            &"Failed to parse operand into number".to_string()));
                            continue;
                        }
                    }

                    let operand_bin_template = instruction.operands[j - 1].trim();
                    let operand_bin_len = operand_bin_template.len();

                    let operand_bin;

                    if operand_bin_template.starts_with("-") {
                        operand_bin = format!("{operand_parsed:b}");
                    }
                    else {
                        let zero = "0";
                        operand_bin = format!("{zero:0>0$}", operand_bin_len)
                    }

                    binary += &format!("{operand_bin:0>0$}", operand_bin_len);
                }

                println!("{}", binary);

            }
            else {
                assembler_result.fails.push(Error::in_line(&isa_file_name, &i,&asm_file_name))
            }
        }
    }
}
