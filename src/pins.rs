use core::marker::PhantomData;

pub struct Unknown;
pub struct Output;
pub struct Input;

pub struct Pin<const PORT: char, const PIN: u8, MODE = Unknown> {
    _mode: PhantomData<MODE>,
}

impl<const PORT: char, const PIN: u8> Pin<PORT, PIN, Unknown> {
    pub fn new() -> Self {
        Self { _mode: PhantomData }
    }

    pub fn into_mode<NEWMODE>(self) -> Pin<PORT, PIN, NEWMODE> {
        Pin { _mode: PhantomData }
    }
}
