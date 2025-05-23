#!/bin/sh

cd elias-fano
cargo build --release
cd ..

./elias-fano/target/release/ef_benchmark ef_res.csv
./elias-fano/target/release/bv_benchmark bv_res.csv
./elias-fano/target/release/myvec_benchmark myvec_res.csv
./elias-fano/target/release/vector_benchmark vector_res.csv
./elias-fano/target/release/ef_benchmark_skewed ef_skewed.csv

python3 make_tables.py
