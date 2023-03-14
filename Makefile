# TODO: learn Makefile

# # # # # # #
# variables
#

rust_log_debug := RUST_LOG=warn,bevy=debug,bevy_pixels_web_game_poc=debug

rust_flags_release := RUSTFLAGS="-D warnings -A dead_code -A unused-imports -A unused_mut -A unused-variables"

# # # # # # # # # # #
# initial commands
#

setup:
	rustup default stable
	cargo install wasm-bindgen-cli
	cargo install miniserve

# # # # # # # # #
# main commands
#

format:
	cargo fmt

check: test clippy

run: run_debug_host

# TODO: describe in README a simple command needed to generate a final production ready dist package for itch.io
web: run_debug_web

# TODO: describe in README
dist: dist_release_web

# # # # # # # # # # # # #
# specialized commands
#

update_rust_toolchain:
	rustup update stable

clean_up:
	rm -rf ./target/

test:
	cargo test
	cargo test --features visualize_schedule_main
	cargo test --features visualize_schedule_fixed_update
	cargo test --release

clippy:
	cargo clippy
	cargo clippy --features visualize_schedule_main
	cargo clippy --features visualize_schedule_fixed_update
	cargo clippy --release
	cargo clippy --target wasm32-unknown-unknown
	cargo clippy --target wasm32-unknown-unknown --release

visualize_schedule_main:
	cargo run --quiet --features visualize_schedule_main | pbcopy
	echo "Graph data is in your clipboard now. Visit https://edotor.net/ and paste it there 🙂"

visualize_schedule_fixed_update:
	cargo run --quiet --features visualize_schedule_fixed_update | pbcopy
	echo "Graph data is in your clipboard now. Visit https://edotor.net/ and paste it there 🙂"

# TODO: make RUST_LOG work for web. Should it be applied on serving instead of building?
build_debug_web:
	$(rust_log_debug) cargo build --target wasm32-unknown-unknown
	rm -rf ./dist/wasm/debug/
	wasm-bindgen \
		--target web \
		--out-dir ./dist/wasm/debug/ \
		--out-name bevy_pixels_web_game_poc \
		--no-typescript \
		target/wasm32-unknown-unknown/debug/bevy_pixels_web_game_poc.wasm
	cp ./dist/wasm_template/index.html ./dist/wasm/debug/index.html

build_release_host:
	$(rust_flags_release) cargo build --release

# TODO: make RUST_LOG work for web. Should it be applied on serving instead of building?
build_release_web:
	$(rust_flags_release) cargo build --target wasm32-unknown-unknown --release
	rm -rf ./dist/wasm/release/
	wasm-bindgen \
		--target web \
		--out-dir ./dist/wasm/release/ \
		--out-name bevy_pixels_web_game_poc \
		--no-typescript \
		--no-demangle \
		target/wasm32-unknown-unknown/release/bevy_pixels_web_game_poc.wasm
	cp ./dist/wasm_template/index.html ./dist/wasm/release/index.html

run_debug_host:
	$(rust_log_debug) cargo run --features bevy/dynamic_linking

# TODO: [ERROR] Route /favicon.ico could not be found
run_debug_web: build_debug_web
	miniserve --port 8080 --index index.html ./dist/wasm/debug/

run_release_host: build_release_host
	./target/release/bevy_pixels_web_game_poc

# TODO: [ERROR] Route /favicon.ico could not be found
# TODO: check app size after build, wonder how heavy file would it be for web
run_release_web: build_release_web
	miniserve --port 9090 --index index.html ./dist/wasm/release/

dist_release_web: build_release_web
	rm -f ./dist/wasm/bevy_pixels_web_game_poc.zip
	mkdir -p ./dist/wasm/release/
	cd ./dist/wasm/release/ && zip ../bevy_pixels_web_game_poc.zip \
		./index.html \
	 	./bevy_pixels_web_game_poc.js \
	 	./bevy_pixels_web_game_poc_bg.wasm
	 echo "Dist package is ready: ./dist/wasm/bevy_pixels_web_game_poc.zip"
