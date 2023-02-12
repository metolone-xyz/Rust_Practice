pub fn add_two(i: i32) -> i32 {
    i + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let num = 2;
        assert_eq!(4, add_two(num));
    }
}
