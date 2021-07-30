pub fn hello() -> &'static str {
    "Hello!"
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!("Hello!", super::hello());
    }
}
