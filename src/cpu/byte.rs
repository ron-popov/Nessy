use crate::cpu::consts;

pub struct Byte {
    value: u8,
}

impl Byte {
    pub fn new(value: u8) -> Byte {
        return Byte{value: value};
    }

    pub fn is_negative(&self) -> bool {
        self.as_array()[7]
    }

    pub fn as_array(&self) -> [bool; consts::BYTE_SIZE] {
        let mut arr: [bool ;8] = [false; consts::BYTE_SIZE];
        let mut new_value = Byte::new(self.value);

        for i in 0..consts::BYTE_SIZE {
            arr[i] = new_value.value % 2 != 0;
            new_value.shift_right();
        }

        return arr;
    }

    pub fn shift_right(&mut self) {
        self.value = self.value / 2;
    }

    pub fn shift_left(&mut self) {
        match self.value.checked_mul(2) {
            Some(val) => self.value = val,
            None => panic!("Overflow on shift left") // TODO : Handle this better
        };
    }
}

// Tests

#[test]
fn array_convertion_1() {
    assert_eq!(Byte::new(1).as_array(), [true, false, false, false, false, false, false, false]);
}

#[test]
fn array_convertion_2() {
    assert_eq!(Byte::new(5).as_array(), [true, false, true, false, false, false, false, false]);
}

#[test]
fn array_convertion_3() {
    assert_eq!(Byte::new(9).as_array(), [true, false, false, true, false, false, false, false]);
}

#[test]
fn array_convertion_4() {
    assert_eq!(Byte::new(u8::MAX).as_array(), [true, true, true, true, true, true, true, true]);
}

#[test]
fn array_convertion_5() {
    assert_eq!(Byte::new(0).as_array(), [false, false, false, false, false, false, false, false]);
}

#[test]
fn shift_right() {
    let mut b = Byte::new(189);
    assert_eq!(b.as_array(), [true, false, true, true, true, true, false, true]);
    b.shift_right();
    assert_eq!(b.as_array(), [false, true, true, true, true, false, true, false]);
}

#[test]
fn shift_left() {
    let mut b = Byte::new(38);
    assert_eq!(b.as_array(), [false, true, true, false, false, true, false, false]);
    b.shift_left();
    assert_eq!(b.as_array(), [false, false, true, true, false, false, true, false]);
}