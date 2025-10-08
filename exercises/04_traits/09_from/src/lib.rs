// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

#[derive(Debug, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        Self { value: value }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}

mod tests {
    use super::*;
    #[test]
    fn from_test() {
        let wrapping1: WrappingU32 = 42.into();
        let wrapping2 = WrappingU32::from(42);
        assert_eq!(wrapping1, wrapping2);
    }
}