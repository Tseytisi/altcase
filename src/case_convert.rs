use std::collections::HashSet;

// These need to all be lowercase
static KEEP_LOWERCASE: [char; 1] = ['i'];
static KEEP_UPPERCASE: [char; 1] = ['l'];

pub fn convert(input: &str, simple_mode: bool) -> String {
    return if simple_mode {
        simple_convert(input, true)
    } else {
        advanced_convert(input)
    }
}

fn simple_convert(input: &str, start_with_uppercase: bool) -> String {
    let mut output = String::with_capacity(input.len());
    let mut next_uppercase = start_with_uppercase;
    for c in input.chars() {
        // Check if the current character has a upper-lower case distinction
        let lowercase = c.to_lowercase().collect::<String>();
        let uppercase = c.to_uppercase().collect::<String>();
        if lowercase == uppercase {
            // No upper/lowercase distinction, so we skip this character and add it as-is
            output.push(c);
        } else {
            if next_uppercase {
                output.push_str(&uppercase);
            } else {
                output.push_str(&lowercase);
            }
            next_uppercase = !next_uppercase;
        }
    }
    return output;
}

// Normal mode
fn advanced_convert(input: &str) -> String {
    // Build sets of the special characters
    let keep_lowercase = HashSet::from(KEEP_LOWERCASE);
    let keep_uppercase = HashSet::from(KEEP_UPPERCASE);

    // First we search for the first special character
    let mut special_char_index: usize = 0;
    let mut special_char_is_lowercase = false;
    for (index, c) in input.chars().enumerate() {
        if let Some(lowercase_c) = c.to_lowercase().nth(0) {
            if keep_lowercase.contains(&lowercase_c) {
                special_char_index = index;
                special_char_is_lowercase = true;
                break;
            } else if keep_uppercase.contains(&lowercase_c) {
                special_char_index = index;
                break;
            }
        }
    }

    // We go through the first part until the first special character, backwards
    let mut next_char_is_uppercase = special_char_is_lowercase;
    let mut first_part: String = String::with_capacity(special_char_index);
    for c in input[0..special_char_index].chars().rev() {
        // Check if the current character has a upper-lower case distinction
        let lowercase = c.to_lowercase().collect::<String>();
        let uppercase = c.to_uppercase().collect::<String>();
        if lowercase == uppercase {
            // No upper/lowercase distinction, so we skip this character and add it as-is
            first_part.push(c);
        } else {
            if next_char_is_uppercase {
                // Not my proudest moment, but otherwise it won't work when a character turns into
                // multiple when turned uppercase.
                // What characters do that anyway?!
                first_part.push_str(&uppercase.chars().rev().collect::<String>());
            } else {
                // Basically we reverse these parts, so they'll be in the right order when we
                // reverse this string again in the next part of the code
                first_part.push_str(&lowercase.chars().rev().collect::<String>());
            }
            next_char_is_uppercase = !next_char_is_uppercase;
        }
    }

    let mut output = String::with_capacity(input.len());
    // Reverse that first_part string again and add it to the actual output string
    for c in first_part.chars().rev() {
        output.push(c);
    }

    for c in input[special_char_index..].chars() {
        // Check if the current character has a upper-lower case distinction
        let lowercase = c.to_lowercase().collect::<String>();
        let uppercase = c.to_uppercase().collect::<String>();
        if lowercase == uppercase {
            // No upper/lowercase distinction, so we skip this character and add it as-is
            output.push(c);
        } else {
            // There's no way the character does not have a first character, I think
            if let Some(first_character) = lowercase.chars().nth(0) {
                if keep_lowercase.contains(&first_character) {
                    // If the character should be kept lowercase, we add it as lowercase
                    output.push_str(&lowercase);
                    next_char_is_uppercase = true;
                } else if keep_uppercase.contains(&first_character) {
                    // Same for uppercase, but with uppercase ofc
                    output.push_str(&uppercase);
                    next_char_is_uppercase = false;
                } else {
                    // If there is no such distinction, we check the variable
                    if next_char_is_uppercase {
                        output.push_str(&uppercase);
                    } else {
                        output.push_str(&lowercase);
                    }
                    next_char_is_uppercase = !next_char_is_uppercase;
                }
            } else {
                panic!("The lowercase of the given character {} is nothing", &c);
            }
        }
    }

    return output;
}