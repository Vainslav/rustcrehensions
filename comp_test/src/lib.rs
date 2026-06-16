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

    #[test]
    fn test_two_for() {
        let v: Vec<(i32, i32)> = comp!((x, y) for x in vec![1, 2, 3] for y in vec![1, 2, 3]);
        assert_eq!(
            v,
            vec![
                (1, 1),
                (1, 2),
                (1, 3),
                (2, 1),
                (2, 2),
                (2, 3),
                (3, 1),
                (3, 2),
                (3, 3)
            ]
        );
    }
}
