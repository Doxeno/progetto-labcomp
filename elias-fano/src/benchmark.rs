use qwt::{RSNarrow, RankBin, SelectBin, perf_and_test_utils::TimingQueries};

pub trait Access {
    fn get(&self, i: usize) -> Option<u64>;
}

pub trait Successor {
    fn successor(&self, x: u64) -> Option<u64>;
}

impl Access for RSNarrow {
    #[inline(always)]
    fn get(&self, i: usize) -> Option<u64> {
        if let Some(res) = self.select1(i as usize) {
            Some(res as u64)
        } else {
            None
        }
    }
}

impl Successor for RSNarrow {
    #[inline(always)]
    fn successor(&self, x: u64) -> Option<u64> {
        if let Some(res) = self.select1(self.rank1(x as usize).unwrap_or(self.n_ones())) {
            Some(res as u64)
        } else {
            None
        }
    }
}

impl Access for Vec<u64> {
    #[inline(always)]
    fn get(&self, i: usize) -> Option<u64> {
        if i < self.len() { Some(self[i]) } else { None }
    }
}

impl Successor for Vec<u64> {
    #[inline(always)]
    fn successor(&self, x: u64) -> Option<u64> {
        if x > *self.last().unwrap() {
            return None;
        }
        let mut l = 0;
        let mut r = self.len();
        while l + 1 < r {
            let mid = (l + r) / 2;
            if self[mid] < x {
                l = mid;
            } else {
                r = mid;
            }
        }
        if self[l] < x {
            l += 1;
        }
        Some(self[l])
    }
}

pub fn access_benchmark<T: Access>(bv: &T, queries: &Vec<usize>) -> u128 {
    let mut timer = TimingQueries::new(1, queries.len());
    timer.start();
    for x in queries {
        bv.get(*x);
    }
    timer.stop();
    timer.get().2
}

pub fn successor_benchmark<T: Successor>(bv: &T, queries: &Vec<u64>) -> u128 {
    let mut timer = TimingQueries::new(1, queries.len());
    timer.start();
    for x in queries {
        bv.successor(*x);
    }
    timer.stop();
    timer.get().2
}
