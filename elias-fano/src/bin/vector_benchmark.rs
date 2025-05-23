use elias_fano::benchmark::*;
use elias_fano::utils::{gen_queries_access, gen_queries_succ, gen_seq};
use qwt::SpaceUsage;
use std::env::args;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage: benchmark resultfile");
        return;
    }
    let filename = &args[1];
    let mut file = File::create(filename).expect("Failed to create file");
    writeln!(
        file,
        "\\log n, \\log u, Space(MiB), Access(ns), Successor(ns)"
    )
    .expect("Failed to write file");

    for log_len in 26..28 {
        let n = 1 << log_len;
        let num_q = 1 << 20;
        for log_val in log_len..34 {
            let max_val = 1 << log_val;
            let v = gen_seq(n, max_val);
            let access_queries = gen_queries_access(num_q, n);
            let successor_queries = gen_queries_succ(num_q, max_val);
            let time_access = access_benchmark(&v, &access_queries);
            let time_succ = successor_benchmark(&v, &successor_queries);
            let space = v.space_usage_MiB();
            writeln!(
                file,
                "{},{},{:.2},{},{}",
                log_len, log_val, space, time_access, time_succ
            )
            .expect("Failed to write file");
        }
    }
}
