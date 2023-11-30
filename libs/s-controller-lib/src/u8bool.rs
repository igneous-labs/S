//! Pod boolean type derived from a single byte where:
//! false = all zeros
//! true = anything other bit pattern

#[derive(Clone, Copy, Debug)]
pub struct U8Bool(pub u8);

impl U8Bool {
    pub const fn is_false(&self) -> bool {
        self.0 == 0
    }

    pub const fn is_true(&self) -> bool {
        !self.is_false()
    }
}

#[derive(Debug)]
pub struct U8BoolMut<'a>(pub &'a mut u8);

impl<'a> U8BoolMut<'a> {
    pub fn set_true(&mut self) {
        *self.0 = 1;
    }

    pub fn set_false(&mut self) {
        *self.0 = 0;
    }
}

impl<'a> From<U8BoolMut<'a>> for U8Bool {
    fn from(U8BoolMut(v): U8BoolMut) -> Self {
        Self(*v)
    }
}
