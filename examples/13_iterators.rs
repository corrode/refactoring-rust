//! Generate the Fibonacci sequence.
//!
//! This works, but make it nicer.
//! (Don't worry about the signature; you have free rein to change that code)

pub fn fib(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}

pub fn first_n(n: usize) -> Vec<u64> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < n {
        result.push(fib(i as u32));
        i = i + 1;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_ten() {
        let fib: Vec<u64> = first_n(10);
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn single_values() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(10), 55);
    }
}
