//! Add one to an `Option<u32>`. Propagate `None`.

pub fn increment(n: Option<u32>) -> Option<u32> {
    let x = match n {
        Some(x) => x,
        None => return None,
    };
    Some(x + 1)
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
