fn calculate(input: &str) -> i32 {
    let v: Vec<i32> = input.trim().split("x").map(|val| val.parse().unwrap() ).collect();
    let mut i = 0;
    let mut min_area = i32::MAX;
    let mut total = 0;

    while i < v.len() {
        let area = v[i] * v[(i + 1) % v.len()];
        total += 2 * area;

        if area < min_area {
            min_area = area;
        }

        i+= 1;
    }

    total + min_area
}

fn main() {
    let input = include_str!("day2.txt");
    let mut subtotal = 0;
    for line in input.split("\n") {
        subtotal += calculate(line);
    }
    println!("ANSWER: {}", subtotal)
}

mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(calculate("2x3x4"), 58);
        assert_eq!(calculate("1x1x10"), 43);
    }
}
