.PHONY: all
all: clean codegen lint test doc

.PHONY: doc
doc:
	cargo doc --no-deps

.PHONY: codegen
codegen:
	cd docker-api-stubs && ./build.sh

.PHONY: test
test:
	cargo test --all-targets --all-features
	cargo test --doc

.PHONY: lint
lint:
	cargo fmt -- --check
	cargo clippy --all-targets --all-features -- -Dclippy::all

.PHONY: clean
clean:
	rm -rf target docker-api-stubs/target

.PHONY: fmt
fmt:
	cargo +nightly fmt --all