//! Add one to an `Option<u32>`. Propagate `None`.
//!
//! Solution highlights:
//! - `Option::map` is exactly "transform the inner value, keep `None` as `None`".
//! - No early return, no temporary, no match.
//! - For longer bodies, `let Some(x) = n else { return None };` is the statement-level form.

pub fn increment(n: Option<u32>) -> Option<u32> {
    n.map(|x| x + 1)
}

fn is_prime(n: &u32) -> bool {
    // dummy implementation
    let n = *n;
    n == 2 || n == 3 || n == 5 || n == 7
}

fn starts_with_1(n: &u32) -> bool {
    n.to_string().starts_with('1')
}

pub fn increment_more(n: Option<u32>) -> Option<u32> {
    // check if n is even
    // check if n is prime
    // check if n starts with 1

    // if let Some(x) = n {
    //     if x % 2 == 0 && is_prime(x) && starts_with_1(x) {
    //         Some(x + 1)
    //     } else {
    //         None
    //     }
    // } else {
    //     None
    // }

    // use early returns instead and `let Some` for the rest

    n
        // do stuff
        .filter(|x| x % 2 == 0)
        // do more stuff
        .filter(is_prime)
        // do even more stuff
        .filter(starts_with_1)
        // at the end, transform the value if it exists
        .map(|x| x + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_is_incremented() {
        assert_eq!(increment(Some(1)), Some(2));
    }

    #[test]
    fn none_propagates() {
        assert_eq!(increment(None), None);
    }
}
