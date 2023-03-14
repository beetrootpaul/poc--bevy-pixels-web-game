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
	# TODO: cargo install wasm-server-runner
	# TODO: cargo install wasm-bindgen-cli
	# TODO: cargo install miniserve

# # # # # # # # #
# main commands
#

format:
	cargo fmt

check: test clippy

run: run_debug_host

web: run_debug_web

# # # # # # # # # # # # #
# specialized commands
#

update_rust_toolchain:
	rustup update stable

clean_up:
	rm -rf ./target/

test:
	cargo test --all-targets
	cargo test --all-targets --features visualize_schedule_main
	cargo test --all-targets --features visualize_schedule_fixed_update
	cargo test --all-targets --release

clippy:
	cargo clippy --all-targets
	cargo clippy --all-targets --features visualize_schedule_main
	cargo clippy --all-targets --features visualize_schedule_fixed_update
	cargo clippy --all-targets --release

visualize_schedule_main:
	cargo run --quiet --features visualize_schedule_main | pbcopy
	echo "Graph data is in your clipboard now. Visit https://edotor.net/ and paste it there 🙂"

visualize_schedule_fixed_update:
	cargo run --quiet --features visualize_schedule_fixed_update | pbcopy
	echo "Graph data is in your clipboard now. Visit https://edotor.net/ and paste it there 🙂"

# TODO: .gitignore update
# TODO: make RUST_LOG work for web. Should it be applied on serving instead of building?
build_debug_web:
	$(rust_log_debug) cargo build --target wasm32-unknown-unknown
	rm -rf ./wasm/release/

build_release_host:
	$(rust_flags_release) cargo build --release

# TODO: .gitignore update
# TODO: describe in README a simple command needed to generate a final production ready dist package for itch.io
# TODO: make RUST_LOG work for web. Should it be applied on serving instead of building?
build_release_web:
	$(rust_flags_release) cargo build --target wasm32-unknown-unknown --release
	rm -rf ./wasm/release/

run_debug_host:
	$(rust_log_debug) cargo run --features bevy/dynamic_linking

run_debug_web: build_debug_web
	# TODO: rm -rf ./target/wasm32-unknown-unknown/debug/
	# TODO: wasm-bindgen \
	# TODO:   --target no-modules \
	# TODO:   --no-modules-global game_loader \
	# TODO:   --out-dir ./wasm/debug \
	# TODO:   --out-name avoid_your_past \
	# TODO:   --no-demangle \
	# TODO:   --no-typescript \
	# TODO:   target/wasm32-unknown-unknown/debug/avoid_your_past_rust_bevy.wasm
	# TODO: cp ./wasm/template/index.html ./wasm/debug/index.html
	# TODO: mkdir -p ./wasm/debug/assets/
	# TODO: cp ./assets/* ./wasm/debug/assets/
	# TODO: miniserve --port 8081 --index index.html ./wasm/debug/

run_release_host: build_release_host
	./target/release/bevy_pixels_web_game_poc

# TODO: check app size after build, wonder how heavy file would it be for web
run_debug_web: build_release_web
	# TODO: rm -rf ./target/wasm32-unknown-unknown/release/
	# TODO: wasm-bindgen \
	# TODO:   --target no-modules \
	# TODO:   --no-modules-global game_loader \
	# TODO:   --out-dir ./wasm/release \
	# TODO:   --out-name avoid_your_past \
	# TODO:   --no-demangle \
	# TODO:   --no-typescript \
	# TODO:   target/wasm32-unknown-unknown/release/avoid_your_past_rust_bevy.wasm
	# TODO: cp ./wasm/template/index.html ./wasm/release/index.html
	# TODO: mkdir -p ./wasm/release/assets/
	# TODO: cp ./assets/* ./wasm/release/assets/
	# TODO: miniserve --port 8080 --index index.html ./wasm/release/

# TODO: package command?
# rm -f ./wasm/dist/avoid_your_past_itch_io.zip
# cd ./wasm/release/
# zip ../dist/avoid_your_past_itch_io.zip \
#   ./index.html \
#   ./avoid_your_past.js \
#   ./avoid_your_past_bg.wasm \
#   ./assets/*