#[derive(PartialEq, Eq, Debug)]
pub struct Value {}

#[cfg(test)]
mod tests {
    #[test]
    pub fn mismatched_types() {
        assert_eq!(crate2::return_value(), crate::Value {});
    }
}