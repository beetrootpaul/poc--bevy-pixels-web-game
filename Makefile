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
	cargo install --locked wasm-bindgen-cli # required by `trunk`
	cargo install --locked trunk

# # # # # # # # #
# main commands
#

format:
	cargo fmt

check: test clippy

run: run_debug_host

web: run_debug_web

# TODO: describe in README
dist: dist_itch_io

# # # # # # # # # # # # #
# specialized commands
#

update_rust_toolchain:
	rustup update stable

clean_up:
	trunk clean
	trunk --config ./Trunk.release.toml clean
	trunk --config ./Trunk.itch_io.toml clean
	rm -rf ./dist/
	cargo clean

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

# # # # # # # # # #
# build commands
#
build_release_host:
	$(rust_flags_release) cargo build --release
	rm -rf ./target/release/assets/
	mkdir -p ./target/release/assets/
	cp ./assets/*.ogg ./target/release/assets/

# # # # # # # # #
# run commands
#

run_debug_host:
	$(rust_log_debug) cargo run --features bevy/dynamic_linking

# TODO: [ERROR] Route /favicon.ico could not be found
# TODO: does rust_log_debug work here?
# `trunk` docs: https://trunkrs.dev
run_debug_web:
	mkdir -p ./dist/web_debug/
	$(rust_log_debug) trunk serve

run_release_host: build_release_host
	./target/release/bevy_pixels_web_game_poc

# TODO: [ERROR] Route /favicon.ico could not be found
# TODO: check app size after build, wonder how heavy file would it be for web
# TODO: does rust_flags_release work here?
run_release_web:
	$(rust_flags_release) trunk --config ./Trunk.release.toml serve

# # # # # # # # #
# dist commands
#

# TODO: create itch.io package on CI as well

# TODO: is it possible to zip correctly without "cd"-ing into the folder?
dist_itch_io:
	trunk --config ./Trunk.itch_io.toml clean
	$(rust_flags_release) trunk --config ./Trunk.itch_io.toml build
	rm -f ./dist/bevy_pixels_web_game_poc__itch_io.zip
	rm -rf ./dist/bevy_pixels_web_game_poc__itch_io/ # in case ZIP was extracted there
	cd ./dist/itch_io/ && zip -r ../bevy_pixels_web_game_poc__itch_io.zip ./
	echo "Dist package is ready: ./dist/bevy_pixels_web_game_poc__itch_io.zip"
