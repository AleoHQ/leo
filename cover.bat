cargo clean
cd compiler/parser
set RUSTFLAGS=-Cinstrument-coverage
cargo +nightly build
set LLVM_PROFILE_FILE=../../target/out/leo_coverage-%%p-%%m.profraw
cargo +nightly test -- test::parser_tests --exact --nocapture
set RUSTUP_TOOLCHAIN=nightly
grcov ../../target/out -s . --binary-path ../../target/debug/ -t html --branch --ignore-not-existing -o ../../target/debug/coverage/
cd ../..
del default.profraw
set RUSTFLAGS=
set LLVM_PROFILE_FILE=
set RUSTUP_TOOLCHAIN=