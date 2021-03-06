use super::consts;
use std::ops::{Index, BitAnd, BitAndAssign, Shr, ShrAssign, Shl, ShlAssign, Sub, SubAssign, Add, AddAssign};
use std::cmp::Ordering;
use std::convert::{From, TryInto};
use std::num::Wrapping;
use std::fmt;

fn _as_array(value: u8) -> [bool; consts::BYTE_SIZE] {
    let mut arr: [bool ;8] = [false; consts::BYTE_SIZE];
    let mut new_value = value;

    for i in 0..consts::BYTE_SIZE {
        arr[i] = new_value % 2 != 0;
        new_value >>= 1;
    }

    return arr;
}

// Byte struct
#[derive(Copy, Clone)]
pub struct Byte {
    value: u8,
    value_arr: [bool; consts::BYTE_SIZE],
}

impl Byte {
    pub fn new(value: u8) -> Byte {
        Byte{value: value, value_arr: _as_array(value)}
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
        self.value_arr = _as_array(value);
    }

    pub fn is_negative(&self) -> bool {
        self[0x07]
    }

    // pub fn get_i8(&self) -> i8 {
    //     if self.is_negative() { // Value is atleast 0x80
    //         let val = self.get_value();
    //         // -1 * (val - 128) as i8
    //         val - 0x100
    //     } else {
    //         self.get_value() as i8
    //     }
    // }

    pub fn get_i8(&self) -> i8 {
        if self.value >= 0x80u8 {
            (self.value as i16 - 0x100i16).try_into().unwrap()
        } else {
            self.value.try_into().unwrap()
        }
    }

    pub fn as_array(&self) -> [bool; consts::BYTE_SIZE] {
        self.value_arr
    }
}

// Trait implementation
impl Index<usize> for Byte {
    type Output = bool;
    fn index<'a>(&'a self, i: usize) -> &'a bool {
        if i >= consts::BYTE_SIZE {
            panic!("Requested invalid index in byte")
        }
        &self.value_arr[i as usize]
    }
}

impl From<u8> for Byte {
    fn from(item: u8) -> Byte {
        Byte::new(item)
    }
}

impl fmt::Display for Byte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02X}", self.get_value())
    }
}

impl fmt::Debug for Byte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02X}", self.get_value())
    }
}

impl PartialEq for Byte {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Byte {}

impl BitAnd for Byte {
    type Output = Byte;

    fn bitand(self, rhs: Byte) -> Byte {
        Byte::new(self.get_value() & rhs.get_value())
    }
}

impl BitAndAssign for Byte {
    fn bitand_assign(&mut self, rhs: Byte) {
        self.set_value(self.get_value() & rhs.get_value());
    }
}

impl Add for Byte {
    type Output = Byte;
    fn add(self, rhs: Byte) -> Byte {
        Byte::new(self.get_value() + rhs.get_value())
    }
}

impl Sub for Byte {
    type Output = Byte;
    fn sub(self, rhs: Byte) -> Byte {
        Byte::new((Wrapping(self.get_value()) - Wrapping(rhs.get_value())).0)
    }
}

impl AddAssign for Byte {
    fn add_assign(&mut self, rhs: Byte) {
        self.set_value(self.get_value() + rhs.get_value());
    }
}

impl SubAssign for Byte {
    fn sub_assign(&mut self, rhs: Byte) {
        self.set_value((Wrapping(self.get_value()) - Wrapping(rhs.get_value())).0);
    }
}

impl Ord for Byte {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_value().cmp(&other.get_value())
    }
}

impl PartialOrd for Byte {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_value().cmp(&other.get_value()))
    }
}


impl Shr<usize> for Byte {
    type Output = Byte;

    fn shr(self, _: usize) -> Byte {
        Byte::new(self.get_value() >> 1)
    }
}

impl Shl<usize> for Byte {
    type Output = Byte;

    fn shl(self, _: usize) -> Byte {
        Byte::new(self.get_value() << 1)
    }
}

impl ShrAssign<usize> for Byte {
    fn shr_assign(&mut self, rhs: usize) {
        self.set_value(self.get_value() >> rhs);
    }
}

impl ShlAssign<usize> for Byte {
    fn shl_assign(&mut self, rhs: usize) {
        self.set_value(self.get_value() << rhs);
    }
}


// Tests

#[test]
fn array_convertion_1() {
    assert_eq!(Byte::new(0x01).as_array(), [true, false, false, false, false, false, false, false]);
}

#[test]
fn array_convertion_2() {
    assert_eq!(Byte::new(0x05).as_array(), [true, false, true, false, false, false, false, false]);
}

#[test]
fn array_convertion_3() {
    assert_eq!(Byte::new(0x09).as_array(), [true, false, false, true, false, false, false, false]);
}

#[test]
fn array_convertion_4() {
    assert_eq!(Byte::new(u8::MAX).as_array(), [true, true, true, true, true, true, true, true]);
}

#[test]
fn array_convertion_5() {
    assert_eq!(Byte::new(0x00).as_array(), [false, false, false, false, false, false, false, false]);
}

#[test]
fn byte_index() {
    let b: Byte = 0x26.into();
    assert_eq!(b.as_array(), [false, true, true, false, false, true, false, false]);
    assert_eq!(b[0], false);
    assert_eq!(b[5], true);
}

#[test]
fn update_value() {
    let mut b: Byte = 0x26.into();
    assert_eq!(b.as_array(), [false, true, true, false, false, true, false, false]);
    assert_eq!(b[0], false);
    assert_eq!(b[5], true);

    b.set_value(0xBD);
    assert_eq!(b.as_array(), [true, false, true, true, true, true, false, true]);
    b >>= 1;
    assert_eq!(b.as_array(), [false, true, true, true, true, false, true, false]);
}

#[test]
fn format_string() {
    assert_eq!(format!("{}", Byte::new(0xA6)), "0xA6");
    assert_eq!(format!("{}", Byte::new(0x06)), "0x06");
    assert_eq!(format!("{}", Byte::new(0x55)), "0x55");
}

#[test]
fn to_i8() {
    let mut b: Byte = 0xFF.into();
    assert_eq!(b.get_value(), 255);
    assert_eq!(b.get_i8(), -1);

    b = 0x80.into();
    assert_eq!(b.get_value(), 128);
    assert_eq!(b.get_i8(), -128);

    b = 0x10.into();
    assert_eq!(b.get_value(), 16);
    assert_eq!(b.get_i8(), 16);
}

#[test]
fn bitwise_and() {
    let byte_one = Byte::new(2);
    let byte_two = Byte::new(3);

    assert_eq!(byte_one.get_value() & byte_two.get_value(), 2);
    assert_eq!(byte_one & byte_two, Byte::new(2));
    assert_eq!((byte_one & byte_two).get_value(), byte_one.get_value() & byte_two.get_value());
}

#[test]
fn shift_right() {
    let mut b = Byte::new(0x20);

    b <<= 1;

    assert_eq!(b.get_value(), 0x40);

    assert_eq!((b << 1).get_value(), 0x80);
    assert_eq!(b.get_value(), 0x40);
}

#[test]
fn shift_left() {
    let mut b = Byte::new(0x0A);

    b >>= 1;

    assert_eq!(b.get_value(), 0x05);

    assert_eq!((b >> 1).get_value(), 0x02);
    assert_eq!(b.get_value(), 0x05);
}