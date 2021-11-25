use std::collections::HashSet;

fn count_houses(input: &str) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut set = HashSet::new();
    set.insert(format!("{}:{}", x, y));
    for c in input.chars() {
        let (delta_x, delta_y) = match c {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => (0, 0),
        };
        x = x + delta_x;
        y = y + delta_y;
        set.insert(format!("{}:{}", x, y));
    }
    set.len()
}

fn main() {
    let input = include_str!("day3.txt");
    println!("ANSWER: {}", count_houses(&input))
}

mod tests {
    use super::*;

    #[test]
    fn test_count_houses() {
        assert_eq!(count_houses(">"), 2);
        assert_eq!(count_houses("^>v<"), 4);
        assert_eq!(count_houses("^v^v^v^v^v"), 2);
    }
}
