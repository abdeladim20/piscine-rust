#[derive(Copy, Clone, Debug)]
pub struct Collatz {
    pub v: u64,
}

impl Collatz {
	pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 || self.v == 0 {
            return None;
        }

        if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = 3 * self.v + 1;
        }

        Some(self.v)
    }
}

pub fn collatz(n: u64) -> usize {
    Collatz::new(n).count()
}