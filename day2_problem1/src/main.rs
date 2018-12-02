use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file_name = get_input_file_name();
    let inputs = read_input_file(file_name);
    let inputs = inputs.lines();

    let checksum = get_checksum(inputs);
    println!("Checksum = {}", checksum);
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
fn get_checksum(box_ids: std::str::Lines) -> u32 {
    let mut twos_found = 0;
    let mut threes_found = 0;

    for box_id in box_ids {
        let (found_two, found_three) = get_letter_count_evaluations(box_id);

        twos_found += if found_two { 1 } else { 0 };
        threes_found += if found_three { 1 } else { 0 };
    }

    return twos_found * threes_found;
}

fn get_letter_count_evaluations(box_id: &str) -> (bool, bool) {
    let mut letters_hashmap = HashMap::new();
    for character in box_id.chars() {
        *letters_hashmap.entry(character).or_insert(0) += 1;
    }

    let mut found_two = false;
    let mut found_three = false;

    for (_, count) in letters_hashmap.iter() {
        match count {
            2 => found_two = true,
            3 => found_three = true,
            _ => {}
        }

        if found_two && found_three {
            break;
        }
    }

    (found_two, found_three)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_checksum_with_example() {
        let mut box_ids = String::new();
        box_ids.push_str("abcdef\n");
        box_ids.push_str("bababc\n");
        box_ids.push_str("abbcde\n");
        box_ids.push_str("abcccd\n");
        box_ids.push_str("aabcdd\n");
        box_ids.push_str("abcdee\n");
        box_ids.push_str("ababab\n");

        assert_eq!(12, get_checksum(box_ids.lines()));
    }

    #[test]
    fn test_get_letter_count_evaluations_returns_false_when_no_letter_has_multiples() {
        let (found_two, found_three) = get_letter_count_evaluations("abcde");
        assert_eq!(false, found_two);
        assert_eq!(false, found_three);
    }

    #[test]
    fn test_get_letter_count_evaluations_returns_found_two_with_single_found_two() {
        let (found_two, _) = get_letter_count_evaluations("abcdea");
        assert_eq!(true, found_two);
    }

    #[test]
    fn test_get_letter_count_evaluations_returns_found_two_with_multiple_found_two() {
        let (found_two, _) = get_letter_count_evaluations("abcdeab");
        assert_eq!(true, found_two);
    }

    #[test]
    fn test_get_letter_count_evaluations_returns_found_three_with_single_found_three() {
        let (found_two, found_three) = get_letter_count_evaluations("abcdeaa");
        assert_eq!(false, found_two);
        assert_eq!(true, found_three);
    }

    #[test]
    fn test_get_letter_count_evaluations_returns_found_three_with_multiple_found_three() {
        let (found_two, found_three) = get_letter_count_evaluations("abcdeabab");
        assert_eq!(false, found_two);
        assert_eq!(true, found_three);
    }

    #[test]
    fn test_get_letter_count_evaluations_returns_found_two_and_three_with_multiples_of_both() {
        let (found_two, found_three) = get_letter_count_evaluations("abcdeababc");
        assert_eq!(true, found_two);
        assert_eq!(true, found_three);
    }
}
