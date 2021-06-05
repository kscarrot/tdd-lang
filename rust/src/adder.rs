fn adder(a: i32, b: i32) -> i32 {
    return a + b;
}


#[cfg(test)]
mod tests {
    use crate::adder::adder;

    #[test]
    fn test_adder() {
        assert_eq!(adder(2, 2), 4)
    }
}
