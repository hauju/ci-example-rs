
pub fn add(a: isize, b: isize) -> isize {
    a + b
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case(0, 1, 1)]
    #[case(-1, 1, 0)]
    #[case(2, 1, 3)]
    #[case(4, 2, 6)]
    fn test_add(#[case] a: isize, #[case] b: isize, #[case] expected: isize) {
        // Act
        let result = add(a, b);
        // Assert
        assert_eq!(expected, result);
    }

}