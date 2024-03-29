fn run_on_sample_a() {
    let mut sample_input = Vec::new();
    sample_input.push(("(())", 0));
    sample_input.push(("()()", 0));
    sample_input.push(("(((", 3));
    sample_input.push(("(()(()(", 3));
    sample_input.push(("))(((((", 3));
    sample_input.push(("())", -1));
    sample_input.push(("))(", -1));
    sample_input.push((")))", -3));
    sample_input.push((")())())", -3));

    for input in &sample_input {
        let result = find_floor(input.0);
        if result == input.1 {
            println!("✅ result={}; expected={}", result, input.1)
        } else {
            println!("❌ result={}; expected={}", result, input.1)
        }
    }
}

fn run_on_actual_a() {
    let input = include_str!("day1.txt");
    println!("{}", find_floor(input));
}

fn run_on_sample_b() {
    let mut sample_input = Vec::new();
    sample_input.push((")", 1));
    sample_input.push(("()())", 5));
    for input in &sample_input {
        let result = find_basement_index(input.0);
        if result == input.1 {
            println!("✅ result={}; expected={}", result, input.1)
        } else {
            println!("❌ result={}; expected={}", result, input.1)
        }
    }
}

fn main() {
    run_on_sample_b();
    let input = include_str!("day1.txt");
    println!("{}", find_basement_index(input));
}

fn find_floor(input: &str) -> i32 {
    let mut floor = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1
        } else if c == ')' {
            floor -= 1
        }
    }
    floor
}

fn find_basement_index(input: &str) -> usize {
    let mut floor = 0;
    let mut final_index = 0;
    for (index, c) in input.char_indices() {
        if c == '(' {
            floor += 1
        } else if c == ')' {
            floor -= 1
        }
        if floor == -1 {
            final_index = index + 1;
            break;
        }
    }
    final_index
}
