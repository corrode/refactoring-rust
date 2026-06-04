//! Return the even numbers below `max`.
//!
//! This works right now, but it eagerly builds a whole `Vec` and hand-rolls
//! the loop. Can you return something lazier and more idiomatic instead?
//! (You have free rein to change the signature.)

pub fn evens(max: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut n = 0;
    while n < max {
        if n % 2 == 0 {
            result.push(n);
        }
        n += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_few() {
        assert_eq!(evens(10), vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn empty() {
        assert_eq!(evens(0), Vec::<u32>::new());
    }

    #[test]
    fn odd_bound() {
        assert_eq!(evens(7), vec![0, 2, 4, 6]);
    }
}
