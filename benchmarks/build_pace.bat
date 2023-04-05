setx RUSTFLAGS "-C embed-bitcode -C target-cpu=native"
cargo build --release --bin pace_bench
