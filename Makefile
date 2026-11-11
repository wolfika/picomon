PREFIX ?= /usr/local
DESTDIR ?=

.PHONY: all web build dev test install uninstall clean

all: build

web/node_modules: web/package.json web/bun.lock
	cd web && bun install --frozen-lockfile
	touch web/node_modules

# The frontend is embedded into the binary, so it must be built first.
web: web/node_modules
	cd web && bun run build

build: web
	cargo build --release

# Backend on :7340 plus Vite dev server with hot reload on :5173.
dev: web/node_modules
	trap 'kill 0' EXIT; cargo run & cd web && bun run dev

test: web/node_modules
	cargo test
	cd web && bun run check

install:
	@[ -f target/release/picomon ] || { echo "target/release/picomon not found. Run 'make build' first." >&2; exit 1; }
	install -Dm755 target/release/picomon $(DESTDIR)$(PREFIX)/bin/picomon
	install -Dm644 contrib/picomon.service $(DESTDIR)/etc/systemd/system/picomon.service
	[ -e $(DESTDIR)/etc/picomon/config.toml ] || \
		install -Dm600 config.example.toml $(DESTDIR)/etc/picomon/config.toml

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/picomon $(DESTDIR)/etc/systemd/system/picomon.service

clean:
	cargo clean
	rm -rf web/dist
