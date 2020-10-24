.PHONY: create-release build fmt

build:
	cargo build --release

fmt:
	cargo fmt

check-version:
ifndef VERSION
	$(error VERSION is undefined)
endif

create-release: check-version
	cargo build --release
	@mkdir -p tmp
	cp target/release/ccon tmp
	tar -cvzf ccon-${VERSION}.tar.gz -C tmp .
	@rm -rf tmp
	@echo "To extract, run: tar -xzf ccon-${VERSION}.tar.gz"
