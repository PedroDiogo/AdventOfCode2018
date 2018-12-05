mod inputs;

fn main() {
    let file_name = inputs::get_input_file_name();
    let initial_polymer = inputs::read_input_file(file_name);

    let resulting_polymer = get_reacted_polymer(&initial_polymer);
    println!("Resulting Reacted Polymer Size = {}", resulting_polymer.len());
}

fn get_reacted_polymer(initial_polymer: &str) -> String {

    let mut polymer = Vec::new();
    polymer.extend(initial_polymer.chars());

    let mut unit_idx = 1;
    while unit_idx < polymer.len() && polymer.len() > 0 {
        let current_unit = polymer.get(unit_idx)
                                .expect(format!("Couldn't get current polymer unit. Index: {}, Polymer Size: {}", unit_idx, polymer.len()).as_str())
                                .to_lowercase()
                                .to_string();
        let previous_unit = polymer.get(unit_idx - 1)
                                .expect(format!("Couldn't get previous polymer unit. Index: {}, Polymer Size: {}", unit_idx, polymer.len()).as_str())
                                .to_lowercase()
                                .to_string();
        let different_units = &polymer[unit_idx] != &polymer[unit_idx - 1];
        if different_units && previous_unit == current_unit {
            polymer.remove(unit_idx);
            polymer.remove(unit_idx-1);
            unit_idx -= 1;
            if unit_idx == 0 {
                unit_idx = 1;
            }
        } else {
            unit_idx += 1;
        }
    }

    let reacted_polymer = polymer.into_iter().collect();
    return reacted_polymer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_reacted_polymer_simple() {
        assert_eq!("", get_reacted_polymer("aA"));
    }

    #[test]
    fn test_get_reacted_polymer_simple_reaction() {
        assert_eq!("", get_reacted_polymer("abBA"));
    }

    #[test]
    fn test_get_reacted_polymer_no_reaction() {
        assert_eq!("abAB", get_reacted_polymer("abAB"));
    }

    #[test]
    fn test_get_reacted_polymer_other_no_reaction() {
        assert_eq!("aabAAB", get_reacted_polymer("aabAAB"));
    }

    #[test]
    fn test_get_reacted_polymer_input_example() {
        assert_eq!("dabCBAcaDA", get_reacted_polymer("dabAcCaCBAcCcaDA"));
    }
}