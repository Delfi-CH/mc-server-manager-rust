pub fn sanity_check() -> String {
    return "This works".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sanity_check();
        assert_eq!(result, "This works");
    }
}
