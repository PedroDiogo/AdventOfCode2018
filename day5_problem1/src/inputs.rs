use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn get_input_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    let filename;
    match args.len() {
        1 => filename = String::from("input"),
        _ => filename = args[1].clone(),
    }
    return filename;
}

pub fn read_input_file(file_name: String) -> String {
    let mut file = File::open(file_name).expect("file not found");
    let mut all_inputs = String::new();
    file.read_to_string(&mut all_inputs)
        .expect("something went wrong reading the file");

    return all_inputs;
}
