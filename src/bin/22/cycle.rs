use crate::Sol;

pub struct Cycle<T: Sized, const N: usize> {
    data: [T; N],
    i: usize,
}

impl<T: Sized, const N: usize> Cycle<T, N> {
    pub fn new(data: [T; N]) -> Cycle<T, N> {
        Cycle { i: 0, data }
    }

    pub fn current(&self) -> &T {
        &self.data[self.i]
    }

    pub fn turn(&mut self, t: Sol) {
        self.i = Self::modulo(self.i as Sol + t, N as Sol)
    }

    fn modulo(s: Sol, o: Sol) -> usize {
        (((s % o) + o) % o) as usize
    }
}
