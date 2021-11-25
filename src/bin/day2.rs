fn calculate_paper(input: &str) -> i32 {
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

fn calculate_ribbon(input: &str) -> i32 {
    let mut v: Vec<i32> = input.trim().split("x").map(|val| val.parse().unwrap() ).collect();
    v.sort();

    let mut i = 0;
    let mut total = 0;
    let smallest_perimeter = 2 * v[0] + 2 * v[1];
    let ribbon = v[0] * v[1] * v[2];
    smallest_perimeter + ribbon

}

fn main() {
    let input = include_str!("day2.txt");
    let mut subtotal = 0;
    for line in input.split("\n") {
        subtotal += calculate_ribbon(line);
    }
    println!("ANSWER: {}", subtotal)
}

mod tests {
    use super::*;

    #[test]
    fn test_calculate_paper() {
        assert_eq!(calculate_paper("2x3x4"), 58);
        assert_eq!(calculate_paper("1x1x10"), 43);
    }

    #[test]
    fn test_calculate_ribbon() {
        assert_eq!(calculate_ribbon("2x3x4"), 34);
        assert_eq!(calculate_ribbon("1x1x10"), 14);
    }
}
