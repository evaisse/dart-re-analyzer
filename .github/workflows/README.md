# GitHub Actions Workflows

## rust-ci.yml

Comprehensive Rust CI pipeline that runs on every push and pull request to main/master branches.

### Jobs

1. **Test** - Runs all tests including unit tests and integration tests
   - Runs `cargo test --verbose`
   - Runs `cargo test --all-features --verbose`

2. **Build** - Builds the project in both debug and release modes
   - Runs `cargo build --verbose`
   - Runs `cargo build --release --verbose`

3. **Format** - Checks code formatting
   - Uses `rustfmt` to ensure consistent code style
   - Fails if code is not properly formatted

4. **Clippy** - Runs the Clippy linter
   - Checks for common mistakes and improvements
   - Configured to fail on warnings (`-D warnings`)

5. **Coverage** - Generates code coverage reports
   - Uses `cargo-tarpaulin` to generate coverage
   - Uploads results to Codecov
   - Uploads coverage artifacts to GitHub

### Caching

The workflow uses GitHub Actions caching to speed up builds:
- Cargo registry cache
- Cargo git index cache
- Build artifacts cache

This significantly reduces CI run times after the first build.

### Code Coverage

The workflow automatically generates code coverage reports using `cargo-tarpaulin` and uploads them to Codecov. To view coverage reports:

1. Ensure the `CODECOV_TOKEN` secret is set in the repository settings
2. Coverage reports will be available at https://codecov.io after each push
3. Coverage artifacts are also available in the GitHub Actions run

### Local Development

To run the same checks locally before pushing:

```bash
# Run tests
cargo test

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Generate coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --verbose --all-features --workspace --timeout 300
```
