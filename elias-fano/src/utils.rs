use std::cmp::min;
use std::usize;

use rand::random_range;

pub fn gen_seq(len: usize, val: u64) -> Vec<u64> {
    assert!(len <= val as usize);
    let mut v: Vec<u64> = (0..len)
        .map(|_| random_range(..(val + 1 - len as u64)))
        .collect();
    v.sort_unstable();
    for (i, x) in v.iter_mut().enumerate() {
        *x += i as u64;
    }
    v
}

pub fn gen_seq_skewed(len: usize, val: u64) -> Vec<u64> {
    assert!(len <= val as usize);
    let dense_l = min((len as u64) * 2, val) - 1;
    let mut v = gen_seq(len - 1, dense_l);
    v.push(val);
    assert!(v.len() == len);
    for i in 0..len - 1 {
        assert!(v[i] < v[i + 1]);
    }
    v
}

pub fn gen_queries_access(num_queries: usize, seq_len: usize) -> Vec<usize> {
    (0..num_queries).map(|_| random_range(..seq_len)).collect()
}

pub fn gen_queries_succ(num_queries: usize, max: u64) -> Vec<u64> {
    (0..num_queries).map(|_| random_range(..max)).collect()
}
