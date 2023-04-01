# poc--bevy-pixels-web-game

Personal PoC of a web (desktop&amp;mobile) game setup based on [Bevy](https://github.com/bevyengine/bevy) game engine
combined with [pixels](https://github.com/parasyte/pixels) library for drawing.

Based on https://github.com/beetrootpaul/avoid-your-past (partial rewrite).

Deployed to https://beetrootpaul.itch.io/bevy-pixels-web-game-poc

## Quick start

- `make setup` – initial Rust toolchain setup
- `make run` – build the app for your host OS and run it
- `make web` – build the app for the web and run it (requires opening a printed URL in a browser)

On a touch-enabled devices (e.g. a smartphone) additional arrows GUI appears on the left (in landscape screen
orientation) or below the game (in a portrait one).

The main game loop is based on a fixed timestamp, running at 30 FPS.

## License

Distributed under the terms of the MIT license.

See [LICENSE](LICENSE).
