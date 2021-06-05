fn repeat(char: &str, count: i32) -> String {
    let mut result = String::new();
    for _ in 0..count {
        result.push_str(char)
    }
    return result;
}


#[cfg(test)]
mod tests {
    use crate::repeat::repeat;

    #[test]
    fn test_repeat() {
        assert_eq!(repeat("a", 3), "aaa")
    }
}
