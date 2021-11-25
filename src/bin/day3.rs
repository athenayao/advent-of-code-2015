use std::collections::HashSet;

fn count_houses(input: &str) -> usize {
    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;

    let mut santa_set = HashSet::new();
    let mut robo_set = HashSet::new();

    santa_set.insert(format!("{}:{}", santa_x, santa_y));
    robo_set.insert(format!("{}:{}", robo_x, robo_y));
    for (i, c) in input.chars().enumerate() {
        let (delta_x, delta_y) = match c {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => (0, 0),
        };
        if i % 2 == 0 {
            santa_x = santa_x + delta_x;
            santa_y = santa_y + delta_y;
            santa_set.insert(format!("{}:{}", santa_x, santa_y));
        } else {
            robo_x = robo_x + delta_x;
            robo_y = robo_y + delta_y;
            robo_set.insert(format!("{}:{}", robo_x, robo_y));
        }
    }
    let the_union: HashSet<_> = santa_set.union(&robo_set).collect();
    the_union.len()
}


fn main() {
    let input = include_str!("day3.txt");
    println!("ANSWER: {}", count_houses(&input))
}

mod tests {
    use super::*;

    #[test]
    fn test_count_houses() {
        assert_eq!(count_houses("^v"), 3);
        assert_eq!(count_houses("^>v<"), 3);
        assert_eq!(count_houses("^v^v^v^v^v"), 11);
    }
}
