use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file_name = get_input_file_name();
    let inputs = read_input_file(file_name);
    let inputs = inputs.lines();

    let common_letters_for_correct_box_ids = get_common_letters_for_correct_box_ids(inputs);
    println!(
        "Common Letters for Correct Box IDs = {}",
        common_letters_for_correct_box_ids
    );
}

// Boilerplate code
fn get_input_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    let filename;
    match args.len() {
        1 => filename = String::from("input"),
        _ => filename = args[1].clone(),
    }
    return filename;
}

fn read_input_file(file_name: String) -> String {
    let mut file = File::open(file_name).expect("file not found");
    let mut all_inputs = String::new();
    file.read_to_string(&mut all_inputs)
        .expect("something went wrong reading the file");

    return all_inputs;
}

// Problem code
fn get_common_letters_for_correct_box_ids(box_ids: std::str::Lines) -> String {
    let box_ids: Vec<&str> = box_ids.collect();

    let mut found_correct_boxes = false;
    let mut common_letters: String = String::new();
    for string1_idx in 0..box_ids.len() {
        for string2_idx in string1_idx..box_ids.len() {
            let string1 = box_ids[string1_idx];
            let string2 = box_ids[string2_idx];
            if hamming_distance(string1, string2) == 1 {
                common_letters = get_common_letters(string1, string2);
                found_correct_boxes = true;
            }
        }
        if found_correct_boxes {
            break;
        }
    }
    return common_letters;
}

fn hamming_distance(string1_idx: &str, string2_idx: &str) -> u32 {
    let mut distance = 0;
    let string1_chars = string1_idx.chars();
    let string2_chars = string2_idx.chars();
    for (char1, char2) in string1_chars.zip(string2_chars) {
        if char1 != char2 {
            distance += 1;
        }
    }
    return distance;
}

fn get_common_letters(string1_idx: &str, string2_idx: &str) -> String {
    let mut common_letters = String::new();
    let string1_chars = string1_idx.chars();
    let string2_chars = string2_idx.chars();
    for (char1, char2) in string1_chars.zip(string2_chars) {
        if char1 == char2 {
            common_letters.push(char1);
        }
    }
    return common_letters;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_common_letters_for_correct_box_ids_with_example() {
        let mut inputs = String::new();
        inputs.push_str("abcde\n");
        inputs.push_str("fghij\n");
        inputs.push_str("klmno\n");
        inputs.push_str("pqrst\n");
        inputs.push_str("fguij\n");
        inputs.push_str("axcye\n");
        inputs.push_str("wvxyz\n");

        assert_eq!(
            "fgij",
            get_common_letters_for_correct_box_ids(inputs.lines())
        );
    }

    #[test]
    fn test_get_common_letters() {
        let string1 = "abcd";
        let string2 = "abed";
        assert_eq!("abd", get_common_letters(string1, string2));
    }

    #[test]
    fn test_hamming_distance_0_away() {
        let string1 = "abcd";
        let string2 = "abcd";
        assert_eq!(0, hamming_distance(string1, string2));
    }

    #[test]
    fn test_hamming_distance_1_away() {
        let string1 = "abcd";
        let string2 = "abed";
        assert_eq!(1, hamming_distance(string1, string2));
    }

    #[test]
    fn test_hamming_distance_4_away() {
        let string1 = "abcd";
        let string2 = "efgh";
        assert_eq!(4, hamming_distance(string1, string2));
    }
}
