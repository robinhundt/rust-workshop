fn multiplication_table(n: usize) -> Vec<Vec<usize>> {
    todo!()
}

// === Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(multiplication_table(3), [[1,2,3], [2,4,6], [3,6,9]]);
        assert_eq!(multiplication_table(4), [[1,2,3,4], [2,4,6,8], [3,6,9,12], [4,8,12,16]]);
        assert_eq!(multiplication_table(1), [[1]]);
    }
}