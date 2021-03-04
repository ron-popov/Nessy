use super::byte::Byte;

use std::fmt;
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy)]
pub struct Double {
    value: u16,
    least_significant: Byte,
    most_significant: Byte,
}

impl Double {
    // Constructors
    pub fn new_from_u16(value: u16) -> Double {
        let mut d = Double{value: value, least_significant: Byte::new(0x00), most_significant: Byte::new(0x00)};
        d.update_significant();

        return d;
    }

    pub fn new_from_significant(least: Byte, most: Byte) -> Double {
        let mut d = Double{value: 0x00, least_significant: least, most_significant: most};
        d.update_u16();

        return d;
    }

    // Inner update self from other value
    fn update_significant(&mut self) {
        self.least_significant = Byte::new((self.value % 0x0100) as u8);
        self.most_significant = Byte::new((self.value / 0x0100) as u8);
    }

    fn update_u16(&mut self) {
        self.value = self.least_significant.get_value() as u16 + self.most_significant.get_value() as u16 * 0x0100 as u16;
    }

    // Getters
    pub fn get_value(&self) -> u16 {
        self.value
    }

    pub fn get_least_significant(&self) -> Byte {
        self.least_significant
    }

    pub fn get_most_significant(&self) -> Byte {
        self.most_significant
    }
}

impl fmt::Display for Double {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04X}", self.value)
    }
}

impl fmt::Debug for Double {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04X}", self.value)
    }
}

impl Add::<u16> for Double {
    type Output = Double;
    fn add(self, other: u16) -> Double{
        Double::new_from_u16(self.value + other)
    }
}

impl AddAssign::<u16> for Double {
    fn add_assign(&mut self, other: u16) {
        self.value += other;
        self.update_significant();
    }
}

impl PartialEq for Double {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl From<Byte> for Double {
    fn from(item: Byte) -> Double {
        Double::new_from_u16(item.get_value() as u16)
    }
}


#[test]
fn double_initialization() {
    let d = Double::new_from_u16(0xABCD);

    assert_eq!(d.get_value(), 0xABCD);
    assert_eq!(d.get_least_significant(), Byte::new(0xCD));
    assert_eq!(d.get_most_significant(), Byte::new(0xAB));
}