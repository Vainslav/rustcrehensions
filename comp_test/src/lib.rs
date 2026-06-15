///
/// ```compile_fail
/// let test_vec = vec![1, 2, 3];
/// let v: Vec<_> = comp!(x for x in test_vec);
/// assert_eq!(v, test_vec); // shouldn't compile because of borrowing
/// ```
/// 
#[cfg(test)]
mod tests {
    use comp_macro::comp;

    #[test]
    fn test_simple() {
        let v: Vec<_> = comp!(x for x in vec![1, 2, 3]);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_simple_from_var() {
        let test_vec = vec![1, 2, 3];
        let v: Vec<_> = comp!(x for x in test_vec);
        assert_eq!(v, vec![1, 2, 3]);
    }
}
