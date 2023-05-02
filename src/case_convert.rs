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

#[cfg(test)]
mod tests {
    use super::*;

    mod simple {
        use super::*;

        #[test]
        /// One letters-only word to convert
        fn single_word() {
            {
                let result = simple_convert("simple", true);
                assert_eq!("SiMpLe", &result);
            }
            {
                let result = simple_convert("simple", false);
                assert_eq!("sImPlE", &result);
            }
        }

        #[test]
        /// Letters separated by non-letters
        fn non_letters() {
            {
                let result = simple_convert("s-i_m/p~l*e", true);
                assert_eq!("S-i_M/p~L*e", &result);
            }
            {
                let result = simple_convert("s-i_m/p~l*e", false);
                assert_eq!("s-I_m/P~l*E", &result);
            }
        }

        #[test]
        /// Words separated by whitespace
        fn whitespace() {
            {
                let result = simple_convert("simple test 123 one two three", true);
                assert_eq!("SiMpLe TeSt 123 OnE tWo ThReE", &result);
            }
            {
                let result = simple_convert("simple test 123 one two three", false);
                assert_eq!("sImPlE tEsT 123 oNe TwO tHrEe", &result);
            }
        }

        #[test]
        /// If no letters are present in the input string, the string should remain unchanged
        fn no_letters() {
            let input = "1984/*-  _+=";
            let result_upper = simple_convert(&input, true);
            let result_lower = simple_convert(&input, false);
            assert_eq!(&input, &result_upper);
            assert_eq!(&input, &result_lower);
        }

        #[test]
        /// No matter the case of the input, the output should be the same
        fn mixed_case_input() {
            {
                let result = simple_convert("WhAt iF wE gIvE iT ThIS", true);
                assert_eq!("WhAt If We GiVe It ThIs", &result);
            }
            {
                let result = simple_convert("WhAt iF wE gIvE iT ThIS", false);
                assert_eq!("wHaT iF wE gIvE iT tHiS", &result);
            }
        }
    }

    mod advanced {
        use super::*;

        #[test]
        /// Advanced convert without special characters should behave the same as [`simple_convert`]
        /// with the 'start with uppercase' parameter set to `false`
        fn no_special_letters() {
            {
                let input = "sentence sans spec chars";
                let result_adv = advanced_convert(&input);
                let result_smp = simple_convert(&input, false);
                assert_eq!(&result_adv, &result_smp);
            }
        }

        #[test]
        /// Non-letters should be skipped, where the case of the next letter should depend on the
        /// previous _letter_, not character.
        fn non_letters() {
            {
                let result = advanced_convert("words-delimited_by/non*whitespace&chars");
                assert_eq!("wOrDs-DeLiMiTeD_bY/nOn*WhiTeSpAcE&cHaRs", &result);
            }
            {
                let result = advanced_convert("m-i/l%l (i@o!n");
                assert_eq!("M-i/L%L (i@O!n", &result);
            }
        }

        #[test]
        /// If no letters are present in the input string, the string should remain unchanged
        fn no_letters() {
            {
                let input = "1984/*-  _+=";
                let result_upper = advanced_convert(&input);
                let result_lower = advanced_convert(&input);
                assert_eq!(&input, &result_upper);
                assert_eq!(&input, &result_lower);
            }
        }

        #[test]
        /// No matter the case of the input, the output should be the same
        fn mixed_case_input() {
            let result = advanced_convert("WhAt iF wE gIvE iT ThIS");
            assert_eq!("wHaT iF wE giVe iT tHiS", &result);
        }
    }

    mod combo {
        use super::*;

        #[test]
        fn choose_mode() {
            let input = "this is the function input";
            let result_smp = convert(&input, true);
            assert_eq!("ThIs Is ThE fUnCtIoN iNpUt", &result_smp);

            let result_adv = convert(&input, false);
            assert_eq!("tHiS iS tHe FuNcTiOn iNpUt", &result_adv);
        }
    }
}