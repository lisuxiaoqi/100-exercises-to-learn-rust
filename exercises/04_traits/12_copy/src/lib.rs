// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl Add for WrappingU32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        WrappingU32 {
            value: self.value.wrapping_add(rhs.value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }

    #[test]
    fn test_round() {
        let x = u8::MAX;
        println!("max u8: {}", x);
        let x1 = x.wrapping_add(1u8);
        println!("x1 u8: {}", x1);
        let x2 = x.wrapping_add(2u8);
        println!("x2 u8: {}", x2);
    }
}
