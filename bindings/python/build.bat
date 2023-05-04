maturin build --manifest-path bindings/python/Cargo.toml -o build --interpreter python3.11 --release
pip install build/nersent_pace_py-0.0.1-cp311-none-win_amd64.whl --force-reinstall

maturin build --manifest-path bindings/python/Cargo.toml -o build --interpreter python3.9 --release
