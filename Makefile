.PHONY: run run-debug release lint fix build clean web web-release yandex-pack \
       landing-dev landing-build landing-install full-build dev-both

# === GAME (Rust) ===

run:
	cargo run --features native

run-debug:
	cargo run --features remote_debug

release:
	cargo build --release

web:
	trunk serve --release --open

web-release:
	trunk build --release

# === LANDING (React/Vite) ===

landing-install:
	cd web && npm install

landing-dev:
	cd web && npm run dev

landing-build:
	cd web && npm run build

# === COMBINED ===

full-build: web-release
	mkdir -p web/public/game
	cp -r dist/* web/public/game/
	cd web && npm run build
	@echo "✓ Full build: web/dist/"

dev-both:
	npx concurrently "trunk serve" "cd web && npm run dev"

yandex-pack: full-build
	cd web/dist && zip -r ../../chertogon-yandex.zip .
	@echo "Ready: chertogon-yandex.zip"

# === UTILITY ===

lint:
	cargo fmt --check
	cargo clippy -- -D warnings

fix:
	cargo fmt

build:
	cargo build

clean:
	cargo clean
	rm -rf dist
	rm -rf web/dist
	rm -f chertogon-yandex.zip
