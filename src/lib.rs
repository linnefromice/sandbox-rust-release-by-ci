pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn sub(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(4, 3), 7);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(2, 2), 0);
        assert_eq!(sub(4, 3), 1);
    }
}
