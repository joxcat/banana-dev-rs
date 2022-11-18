# Source: Just documentation https://just.systems/man/en

_nextest features: _ensure_nextest
	cargo nextest run --no-default-features --features {{features}}

test-ureq:
	just _nextest sync_ureq_rustls
	just _nextest sync_ureq_native-tls

# Check for memory leakage using miri
miri: _ensure_miri _ensure_nextest
	cargo +nightly miri nextest run

# Check for code coverage using llvm-cov
coverage: _ensure_llvm-cov _ensure_nextest
	cargo llvm-cov nextest

_ensure_nextest:
	if ! command -v cargo-nextest >/dev/null; then cargo install cargo-nextest; fi
_ensure_miri:
	if ! cargo-miri -v >/dev/null 2>&1; then rustup +nightly component add miri; fi
_ensure_llvm-cov:
	if ! command -v cargo-llvm-cov >/dev/null; then cargo install cargo-llvm-cov; fi