#[macro_export]
macro_rules! check {
    (is_alnum $value: expr) => {
        $value.len() <= 20 && $value.chars().all(|ch| char::is_alphanumeric(ch))
    };
    (alnum $value: expr) => {
        debug_assert!(check!(is_alnum $value));
    }
}
