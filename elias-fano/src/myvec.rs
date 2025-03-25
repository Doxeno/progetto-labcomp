use crate::benchmark::{Access, Successor};
use qwt::{BitVector, BitVectorMut, SpaceUsage};

#[derive(Default, Debug)]
pub struct MyVec {
    n: usize,
    width: usize,
    bv: BitVector,
}

impl MyVec {
    pub fn new(v: Vec<u64>) -> Self {
        let w = 64 - v.last().unwrap_or(&0).leading_zeros() as usize;
        let mut mbv = BitVectorMut::new();
        for x in v.as_slice() {
            mbv.append_bits(*x, w);
        }
        MyVec {
            n: v.len(),
            width: w,
            bv: mbv.into(),
        }
    }
}

impl Access for MyVec {
    fn get(&self, i: usize) -> Option<u64> {
        self.bv.get_bits(i * self.width, self.width)
    }
}

impl Successor for MyVec {
    fn successor(&self, x: u64) -> Option<u64> {
        if x > self.get(self.n - 1).unwrap_or(0) {
            return None;
        }
        let mut l = 0;
        let mut r = self.n;
        while l + 1 < r {
            let mid = (l + r) / 2;
            if self.get(mid).unwrap() < x {
                l = mid;
            } else {
                r = mid;
            }
        }
        if self.get(l).unwrap() < x {
            l += 1;
        }
        self.get(l)
    }
}

impl SpaceUsage for MyVec {
    fn space_usage_byte(&self) -> usize {
        self.bv.space_usage_byte() + self.n.space_usage_byte() + self.width.space_usage_byte()
    }
}
