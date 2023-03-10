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
	cargo clippy --all-targets --features print_system_sets_diagram
	cargo clippy --all-targets --release

run: run_desktop_debug

# # # # # # # # # # # # #
# specialized commands
#

print_system_sets_diagram:
	cargo run --quiet --features print_system_sets_diagram | pbcopy
	echo "Graph data is in your clipboard now. Visit https://dreampuf.github.io/GraphvizOnline and paste it there 🙂"

run_desktop_debug:
	cargo run

run_desktop_release:
	cargo run --release
