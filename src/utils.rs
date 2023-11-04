pub trait Title {
    type Output;

    fn title(&self) -> Self::Output;
}

impl<T: ToString> Title for T {
    type Output = String;

    fn title(&self) -> Self::Output {
        let mut s = self.to_string();
        s[..1].make_ascii_uppercase();
        s
    }
}
