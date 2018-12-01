use std::collections::HashSet;
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
    let mut visited_frequencies = HashSet::new();
    visited_frequencies.insert(frequency);

    let mut found_frequency = false;
    loop {
        for frequency_change in all_inputs.lines() {
            let frequency_change_int = frequency_change.parse::<i32>().expect("not an integer");
            frequency += frequency_change_int;
            if visited_frequencies.contains(&frequency) {
                println!("Visited frequencies contains {}, breaking.", frequency);
                found_frequency = true;
                break;
            }
            visited_frequencies.insert(frequency);
        }
        if found_frequency {
            break;
        }
    }

    println!("{}", frequency);
}
