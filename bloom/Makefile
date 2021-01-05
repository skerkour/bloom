NAME = bloom
COMMIT = $(shell git rev-parse HEAD)
DATE := $(shell date +"%Y-%m-%d")
DIST_DIR = dist
TARGET_DIR = target
RELEASE_BINARY = target/release/$(NAME)
PUBLIC_DIR = public

EMAILS_BLOCKLIST_FILE = email_domains_blocklist.txt


.PHONY: all
all: build

.PHONY: dev
dev:
	RUST_LOG=debug cargo watch -x 'run -- server'

.PHONY: build
build:
	cargo build --release
	cp -r $(RELEASE_BINARY) $(DIST_DIR)/
	cp $(EMAILS_BLOCKLIST_FILE) $(DIST_DIR)/$(EMAILS_BLOCKLIST_FILE)


.PHONY: build_dev
build_dev:
	cargo build


.PHONY: build_static
build_static:
	cargo build --target x86_64-unknown-linux-musl --release
	mv target/x86_64-unknown-linux-musl/release/$(NAME) $(RELEASE_BINARY)
	cp -r $(RELEASE_BINARY) $(DIST_DIR)/
	cp $(EMAILS_BLOCKLIST_FILE) $(DIST_DIR)/$(EMAILS_BLOCKLIST_FILE)


.PHONY: check
check:
	cargo check


.PHONY: fmt
fmt:
	cargo fmt


.PHONY: lint
lint:
	cargo fmt -- --check
	cargo clippy -- -D warnings


.PHONY: vendor
vendor:
	cargo vendor vendor


# Update dependencies
.PHONY: update
update:
	rm -rf vendor && mv .cargo/config.toml .cargo/config_tmp.toml
	cargo update
	mv .cargo/config_tmp.toml .cargo/config.toml
	make vendor


.PHONY: test
test:
	cargo test --doc --all


.PHONY: clean
clean:
	rm -rf $(DIST_DIR) $(TARGET_DIR)


.PHONY: public
public:
	rm -rf $(PUBLIC_DIR)
	mkdir -p $(PUBLIC_DIR)
	cp -r ../webapp/dist/* $(PUBLIC_DIR)
