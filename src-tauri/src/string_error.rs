pub(crate) trait Nope<T> {
    fn nope(self) -> Result<T, String>;
}

impl<T, E: ToString> Nope<T> for Result<T, E> {
    fn nope(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}
