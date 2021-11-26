fn validate_nice_string(input: &str) -> bool {
    let mut num_vowels = 0;
    let mut has_repeat = false;
    let mut has_forbidden_text = false;
    let input = input.as_bytes();
    for (i, &c) in input.iter().enumerate() {
        if c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u' {
            num_vowels += 1;
        }
        if i != 0 {
            let prev_char = &input[i-1];
            if c == *prev_char {
                has_repeat = true;
            }
            if (*prev_char == b'a' && c == b'b') || (*prev_char == b'c' && c == b'd') || (*prev_char == b'p' && c == b'q') || (*prev_char == b'x' && c == b'y') {
                has_forbidden_text = true;
            }

        }
    }
    num_vowels >= 3 && has_repeat && !has_forbidden_text
}

fn main() {
    let input = include_str!("day5.txt");
    let mut total = 0;
    for line in input.split("\n") {
        total += if validate_nice_string(line) { 1 } else { 0 };
    }
    println!("ANSWER: {}", total)
}

mod tests {
    use super::*;

    #[test]
    fn test_validate_nice_string() {
        assert_eq!(validate_nice_string("ugknbfddgicrmopn"), true, "ugknbfddgicrmopn");
        assert_eq!(validate_nice_string("aaa"), true, "aaa");
        assert_eq!(validate_nice_string("jchzalrnumimnmhp"), false, "jchzalrnumimnmhp");
        assert_eq!(validate_nice_string("haegwjzuvuyypxyu"), false, "haegwjzuvuyypxyu");
        assert_eq!(validate_nice_string("dvszwmarrgswjxmb"), false, "dvszwmarrgswjxmb");
    }
}
