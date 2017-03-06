#[cfg(test)]
mod chapter_1 {
    use std::collections::HashMap;

    /* 1.1 ------------------------------------------------------------------*/
    fn chars_are_unique(string: &String) -> bool {
        let mut present: [bool; 128] = [false; 128];

        for c in string.bytes() {
            if present[c as usize] {
                return false;
            } else {
                present[c as usize] = true;
            }
        }
        true
    }

    #[test]
    fn test_1_1() {
        let unique_string = "abcdefghijklmnopqrstuvwxyz".to_string();
        let non_unique_string = "abcdefghijklmnozqrstuvwxyza".to_string();
        assert!(chars_are_unique(&unique_string));
        assert!(!chars_are_unique(&non_unique_string));
    }

    /* 1.2 ------------------------------------------------------------------*/
    fn are_permutations(str_a: &String, str_b: &String) -> bool {
        let mut char_counts: [i32; 128] = [0; 128];

        for c in str_a.bytes() { char_counts[c as usize] += 1; }
        for c in str_b.bytes() { char_counts[c as usize] -= 1; }
        for i in 0..128 { if char_counts[i] != 0 { return false; } }
        true
    }

    #[test]
    fn test_1_2() {
        let str_a = "hey watdo".to_string();
        let str_b = "htydwoe a".to_string();
        let str_c = "hey wztdo".to_string();
        assert!(are_permutations(&str_a, &str_b));
        assert!(!are_permutations(&str_a, &str_c));
    }

    /* 1.3 ------------------------------------------------------------------*/
    fn urlify(string: String) -> String {
        let mut bytes = string.into_bytes();
        let bytes_len = bytes.len();

        // count spaces
        let mut space_count = 0;
        for c in &bytes {
            if *c == ' ' as u8 {
                space_count += 1;
            }
        }

        // because there's two spaces tacked onto the end for every one in the string
        space_count = space_count / 3;

        let mut i = bytes.len() - 1;
        loop {
            let effective_i = (i + bytes_len - (space_count*2)) % bytes_len;
            let c = bytes[effective_i];
            if c == ' ' as u8 {
                bytes[i] = '0' as u8;
                bytes[i - 1] = '2' as u8;
                bytes[i - 2] = '%' as u8;
                i -= 2;
                space_count -= 1;
            } else {
                bytes[i] = c;
            }

            if i > 0 { i -= 1; } else { break; }
        }

        String::from_utf8(bytes).unwrap()
    }

    #[test]
    fn test_1_3() {
        assert_eq!(urlify("test 1 2 3      ".to_string()),       "test%201%202%203");
        assert_eq!(urlify("urlify me captain!    ".to_string()), "urlify%20me%20captain!");
    }


    /* 1.4 ------------------------------------------------------------------*/
    fn has_palindromic_permutation(string: &str) -> bool {
        let mut character_counts: HashMap<char, u32> = HashMap::with_capacity(string.len());
        let mut odds: u32 = 0;
        for c in (*string).chars() {
            if c != ' ' {
                let lowercase = c.to_lowercase().next().unwrap();
                let counter = character_counts.entry(lowercase).or_insert(0);
                *counter += 1;
                if *counter % 2 == 1 {
                    odds += 1;
                } else if *counter > 1 {
                    odds -= 1;
                }
            }
        }

        odds < 2
    }

    #[test]
    fn test_1_4() {
        assert!(has_palindromic_permutation("Tact Coa"));
        assert!(has_palindromic_permutation("Seresstdsde serst"));
        assert!(!has_palindromic_permutation("definitely not a palindrome"));
    }

}
