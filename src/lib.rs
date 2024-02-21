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
    fn it_works() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(sub(3, 1), 2);
    }
}
