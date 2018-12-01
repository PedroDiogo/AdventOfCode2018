use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename;
    match args.len() {
        1 => filename = "input",
        _ => filename = &args[1],
    }

    let mut file = File::open(filename).expect("file not found");

    let mut all_inputs = String::new();
    file.read_to_string(&mut all_inputs)
        .expect("something went wrong reading the file");

    let mut frequency = 0;
    all_inputs.lines().for_each(|frequency_change| {
        let frequency_change_int = frequency_change.parse::<i32>().expect("not an integer");

        frequency += frequency_change_int;
    });

    println!("{}", frequency);
}
