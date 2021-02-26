use super::consts;
use std::ops::Index;
use std::convert::From;
use std::fmt;



// Utils functions
fn _shift_right(value: u8) -> u8 {
    value >> 1
}

fn _shift_left(value: u8) -> u8 {
    value << 1
}

fn _as_array(value: u8) -> [bool; consts::BYTE_SIZE] {
    let mut arr: [bool ;8] = [false; consts::BYTE_SIZE];
    let mut new_value = value;

    for i in 0..consts::BYTE_SIZE {
        arr[i] = new_value % 2 != 0;
        new_value = _shift_right(new_value);
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
        self[7]
    }

    pub fn as_array(&self) -> [bool; consts::BYTE_SIZE] {
        self.value_arr
    }

    pub fn shift_right(&mut self) {
        self.value = _shift_right(self.value);
        self.value_arr = _as_array(self.value);
    }

    pub fn shift_left(&mut self) {
        self.value = _shift_left(self.value);
        self.value_arr = _as_array(self.value);
    }
}

impl Index<usize> for Byte {
    type Output = bool;
    fn index<'a>(&'a self, i: usize) -> &'a bool {
        if i >= consts::BYTE_SIZE {
            panic!("Requested invalid index in byte")
        }
        &self.value_arr[i]
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
fn shift_right() {
    let mut b: Byte = 0xBD.into();
    assert_eq!(b.as_array(), [true, false, true, true, true, true, false, true]);
    b.shift_right();
    assert_eq!(b.as_array(), [false, true, true, true, true, false, true, false]);
}

#[test]
fn shift_left() {
    let mut b: Byte = 0x26.into();
    assert_eq!(b.as_array(), [false, true, true, false, false, true, false, false]);
    b.shift_left();
    assert_eq!(b.as_array(), [false, false, true, true, false, false, true, false]);
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
    b.shift_right();
    assert_eq!(b.as_array(), [false, true, true, true, true, false, true, false]);
}

#[test]
fn format_string() {
    assert_eq!(format!("{}", Byte::new(0xA6)), "0xA6");
    assert_eq!(format!("{}", Byte::new(0x06)), "0x06");
    assert_eq!(format!("{}", Byte::new(0x55)), "0x55");
}