//! Given a string of space-separated integers, return the highest
//! and lowest value.

pub fn highest_and_lowest(input: &str) -> (i64, i64) {
    let parts: Vec<&str> = input.split(' ').collect();
    let mut max: i64 = parts[0].parse().unwrap();
    let mut min: i64 = parts[0].parse().unwrap();
    for part in &parts[1..] {
        let n: i64 = part.parse().unwrap();
        if n > max {
            max = n;
        }
        if n < min {
            min = n;
        }
    }
    (max, min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typical() {
        assert_eq!(
            highest_and_lowest("123 3534 534 0 100000 44 222"),
            (100_000, 0)
        );
    }

    #[test]
    fn single_number() {
        assert_eq!(highest_and_lowest("42"), (42, 42));
    }

    #[test]
    fn negatives() {
        assert_eq!(highest_and_lowest("-1 -2 -3"), (-1, -3));
    }
}
