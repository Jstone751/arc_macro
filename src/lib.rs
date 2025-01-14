#[macro_export]
macro_rules! arc {
    ($($elem:expr),*) => {{
        Arc::new([$($elem),*])
    }};
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;

    #[test]
    pub fn test_macro() {
        let macro_arc = arc![1, 2, 3];
        let reg_arc = Arc::new([1, 2, 3]);
        assert_eq!(reg_arc, macro_arc)
    }
}
