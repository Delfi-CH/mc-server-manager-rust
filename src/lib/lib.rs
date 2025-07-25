pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

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
