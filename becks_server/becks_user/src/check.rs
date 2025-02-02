#[macro_export]
macro_rules! check {
    (is_alnum $value: expr) => {
        $value.chars().all(char::is_alphanumeric)
    };
    (alnum $value: expr) => {
        debug_assert!(check!(is_alnum $value));
    }
}
