use std::collections::HashSet;

struct Visitor {
    current_x: i32,
    current_y: i32,
    visited: HashSet<String>,
}

fn move_and_deliver_present(visitor: &mut Visitor, delta_x: i32, delta_y: i32)  {
    visitor.current_x = visitor.current_x + delta_x;
    visitor.current_y = visitor.current_y + delta_y;
    visitor.visited.insert(format!("{}:{}", visitor.current_x, visitor.current_y));
}

fn count_houses(input: &str) -> usize {
    let mut santa = Visitor {
        current_x: 0,
        current_y: 0,
        visited: HashSet::new(),
    };
    let mut robo_santa = Visitor {
        current_x: 0,
        current_y: 0,
        visited: HashSet::new(),
    };

    move_and_deliver_present(&mut santa, 0, 0);
    move_and_deliver_present(&mut robo_santa, 0, 0);

    for (i, c) in input.chars().enumerate() {
        let (delta_x, delta_y) = match c {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => (0, 0),
        };
        if i % 2 == 0 {
            move_and_deliver_present(&mut santa, delta_x, delta_y);
        } else {
            move_and_deliver_present(&mut robo_santa, delta_x, delta_y);
        }
    }
    let the_union: HashSet<_> = santa.visited.union(&robo_santa.visited).collect();
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
