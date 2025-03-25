use elias_fano::benchmark::*;
use elias_fano::utils::{gen_queries_access, gen_queries_succ, gen_seq};
use qwt::{BitVector, RSNarrow, SpaceUsage};

fn main() {
    for log_len in 25..28 {
        let n = 1 << log_len;
        let num_q = 1 << 20;
        for log_val in log_len..36 {
            let max_val = 1 << log_val;
            let v = gen_seq(n, max_val);
            let access_queries = gen_queries_access(num_q, n);
            let successor_queries = gen_queries_succ(num_q, max_val);
            let bv: BitVector = v.into_iter().collect();
            let rsn = RSNarrow::new(bv);
            let time_access = access_benchmark(&rsn, &access_queries);
            let time_succ = successor_benchmark(&rsn, &successor_queries);
            let space = rsn.space_usage_MiB();
            println!("N: {} U: {}", n, max_val);
            println!("Space usage: {} MiB", space);
            println!("Average access time: {} ns", time_access);
            println!("Average successor time: {} ns", time_succ);
        }
    }
}
