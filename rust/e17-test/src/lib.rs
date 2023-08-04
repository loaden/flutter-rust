pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn devision(value: usize) -> usize {
    value / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_division() {
        let result = devision(5);
        assert_eq!(result, 3);
    }
}
