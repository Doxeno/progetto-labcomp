use crate::benchmark::{Access, Successor};
use qwt::{BitVector, BitVectorMut, RSNarrow, SelectBin, SpaceUsage};

#[derive(Debug, Default)]
pub struct EliasFano {
    n: usize,
    low_bits_count: usize,
    high: RSNarrow,
    low: BitVector,
}

impl EliasFano {
    #[inline(always)]
    fn get_low(&self, pos: usize) -> Option<u64> {
        self.low
            .get_bits(pos * self.low_bits_count, self.low_bits_count)
    }

    #[inline(always)]
    fn get_low_unchecked(&self, pos: usize) -> u64 {
        unsafe {
            self.low
                .get_bits_unchecked(pos * self.low_bits_count, self.low_bits_count)
        }
    }
}

impl Access for EliasFano {
    fn get(&self, pos: usize) -> Option<u64> {
        if let Some(low_bits) = self.get_low(pos) {
            let high_bits_c = self.high.select1(pos).unwrap();
            let high_bits = (high_bits_c - pos) << self.low_bits_count;
            Some(high_bits as u64 | low_bits)
        } else {
            None
        }
    }
}

impl EliasFano {
    pub fn get_unchecked(&self, pos: usize) -> u64 {
        unsafe {
            let high_bits = self.high.select1_unchecked(pos) << self.low_bits_count;
            let low_bits = self.get_low_unchecked(pos);
            high_bits as u64 | low_bits
        }
    }

    pub fn lower_bound(&self, x: u64) -> Option<u64> {
        let x_high_bits = x >> self.low_bits_count;
        if x_high_bits as usize > self.high.n_zeros() {
            return None;
        }
        let x_low_bits = x ^ (x_high_bits << self.low_bits_count);

        let upper_0 = self.high.select0(x_high_bits as usize);
        let mut upper = if let Some(u0) = upper_0 {
            u0 - x_high_bits as usize
        } else {
            self.n
        };

        let mut lower = if x_high_bits == 0 {
            0
        } else {
            if let Some(lower_0) = self.high.select0(x_high_bits as usize - 1) {
                lower_0 + 1 - x_high_bits as usize
            } else {
                return None;
            }
        };

        if lower >= upper {
            return self.get(lower);
        }

        let lower_l_bits = self.get_low(lower).unwrap();
        if lower_l_bits >= x_low_bits {
            return self.get(lower);
        }

        while lower + 1 < upper {
            let mid = (lower + upper) / 2;
            let mid_low_bits = self.get_low(mid).unwrap();
            if mid_low_bits < x_low_bits {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        lower += 1;
        self.get(lower)
    }

    pub fn lower_bound_id(&self, x: u64) -> Option<usize> {
        let x_high_bits = x >> self.low_bits_count;
        let x_low_bits = x ^ (x_high_bits << self.low_bits_count);

        let upper_0 = self.high.select0(x_high_bits as usize);
        let mut upper = if let Some(u0) = upper_0 {
            u0 - x_high_bits as usize
        } else {
            self.n
        };

        let mut lower = if x_high_bits == 0 {
            0
        } else {
            if let Some(lower_0) = self.high.select0(x_high_bits as usize - 1) {
                lower_0 - x_high_bits as usize + 1
            } else {
                return None;
            }
        };

        if lower == upper {
            return Some(lower);
        }

        let lower_l_bits = self.get_low(lower).unwrap();
        if lower_l_bits >= x_low_bits {
            return Some(lower);
        }

        while lower + 1 < upper {
            let mid = (lower + upper) / 2;
            let mid_low_bits = self.get_low(mid).unwrap();
            if mid_low_bits < x_low_bits {
                lower = mid;
            } else {
                upper = mid;
            }
        }

        lower += 1;
        if lower < self.n { Some(lower) } else { None }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.n
    }

    pub fn new(v: &Vec<u64>) -> Self {
        if v.is_empty() {
            return EliasFano::default();
        }

        assert!(v.is_sorted());

        let m = v.last().unwrap();
        let n = v.len() as u64;
        let low_bits_c: usize = 64 - ((m + 1) / n).leading_zeros() as usize;
        let mut low_bv = BitVectorMut::new();
        let mut high_bv = BitVectorMut::new();
        let mut cur_high = 0;
        let low_mask: u64 = (1 << low_bits_c) - 1;

        for x in v.as_slice() {
            let low_half = x & low_mask;
            low_bv.append_bits(low_half, low_bits_c);
            while ((cur_high + 1) << low_bits_c) <= *x {
                high_bv.append_bits(0, 1);
                cur_high += 1;
            }
            high_bv.append_bits(1, 1);
        }

        EliasFano {
            n: v.len(),
            low_bits_count: low_bits_c,
            low: low_bv.into(),
            high: RSNarrow::new(high_bv.into()),
        }
    }
}

impl SpaceUsage for EliasFano {
    fn space_usage_byte(&self) -> usize {
        self.high.space_usage_byte()
            + self.low.space_usage_byte()
            + self.n.space_usage_byte()
            + self.low_bits_count.space_usage_byte()
    }
}

impl Successor for EliasFano {
    #[inline(always)]
    fn successor(&self, x: u64) -> Option<u64> {
        self.lower_bound(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_vec(v: Vec<u64>) {
        let ef = EliasFano::new(&v);
        for i in 0..v.len() {
            assert_eq!(ef.get(i), Some(v[i]));
        }

        for i in 0..v[0] {
            assert_eq!(ef.lower_bound(i), Some(v[0]));
        }
        for i in 0..v.len() - 1 {
            assert_eq!(ef.lower_bound(v[i]), Some(v[i]));
            for x in (v[i] + 1)..v[i + 1] {
                assert_eq!(ef.lower_bound(x), Some(v[i + 1]));
            }
        }
        for i in v[v.len() - 1] + 1..v[v.len() - 1] + 20 {
            assert_eq!(ef.lower_bound(i), None);
        }
    }

    #[test]
    fn test_ef_small() {
        let v = vec![1, 4, 9, 12, 27];
        test_vec(v);
    }

    #[test]
    fn test_ef_mid() {
        let v = vec![1, 4, 9, 12, 27, 32, 34, 35, 40, 44, 49, 70, 71];
        test_vec(v);
    }

    #[test]
    fn test_ef_big() {
        use crate::utils::gen_seq;
        let v = gen_seq(200000, 10000000);
        test_vec(v);
    }
}
