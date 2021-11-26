use md5;

const SECRET_KEY: &str = "iwrupvqb";

fn get_hash(secret_key: &str, number: u32) -> md5::Digest {
    md5::compute(format!("{}{}", secret_key, number))
}

fn get_earliest_number(secret_key: &str) -> u32 {
    let mut number = 0;
    loop {
        let md5_hash = format!("{:x}", get_hash(secret_key, number));
        if md5_hash.starts_with("000000") {
            break number;
        }
        number += 1;
    }

}

fn main() {
    let answer = get_earliest_number(SECRET_KEY);
    println!("ANSWER: {}", answer)
}

mod tests {
    use super::*;

    #[test]
    fn test_get_earliest_number() {
        assert_eq!(get_earliest_number("abcdef"), 609043);
        assert_eq!(get_earliest_number("pqrstuv"), 1048970);
    }
}
