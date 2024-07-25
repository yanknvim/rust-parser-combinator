pub trait Parser<T>: Fn(String) -> Option<(T, String)> {}
impl<T, U> Parser<T> for U where U: Fn(String) -> Option<(T, String)> {}

