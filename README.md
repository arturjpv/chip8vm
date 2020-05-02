# Build manual
mdbook build

# Build crate reference
cargo doc --no-default-features --no-deps --target-dir manual/book/reference

# Build test coverage report
cargo tarpaulin --ignore-tests -o Html --output-dir manual/book/coverage