use std::collections::HashSet;

struct Visitor {
    current_x: i32,
    current_y: i32,
    visited: HashSet<String>,
}

impl Visitor {
    fn visit(&mut self, delta_x: i32, delta_y: i32) {
        self.current_x = self.current_x + delta_x;
        self.current_y = self.current_y + delta_y;
        self.visited.insert(format!("{}:{}", self.current_x, self.current_y));
    }
}

fn build_visitor() -> Visitor {
    let mut visitor = Visitor {
        current_x: 0,
        current_y: 0,
        visited: HashSet::new()
    };
    visitor.visit(0, 0);
    visitor
}

fn count_houses(input: &str) -> usize {
    let mut santa = build_visitor();
    let mut robo_santa = build_visitor();


    for (i, c) in input.chars().enumerate() {
        let (delta_x, delta_y) = match c {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => (0, 0),
        };
        if i % 2 == 0 {
            santa.visit(delta_x, delta_y);
        } else {
            robo_santa.visit(delta_x, delta_y);

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
