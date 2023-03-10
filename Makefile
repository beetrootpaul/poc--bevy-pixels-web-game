# TODO: learn Makefile

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

check:
	cargo clippy --all-targets

run: run_desktop_debug

# # # # # # # # # # # # #
# specialized commands
#

run_desktop_debug:
	cargo run

run_desktop_release:
	cargo run --release
