# TODO: learn Makefile

# # # # # # #
# variables
#

rust_log_debug := RUST_LOG=warn,bevy=debug,bevy_pixels_web_game_poc=debug

# # # # # # # # # # #
# initial commands
#

# to be used once
setup:
	rustup update stable
	rustup default stable

# # # # # # # # #
# main commands
#

format:
	cargo fmt

check: test clippy

run: run_desktop_debug

# # # # # # # # # # # # #
# specialized commands
#

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

run_desktop_debug:
	$(rust_log_debug) cargo run --features bevy/dynamic_linking

run_desktop_release:
	cargo run --release
