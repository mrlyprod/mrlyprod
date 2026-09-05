/// The one function of the placeholder release.
pub fn hello() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod tests {
    #[test]
    fn hello() {
        assert_eq!(super::hello(), "Hello, World!");
    }
}
