# Tower Defense Game - Bevy Engine

A high-performance tower defense game built with Rust and the Bevy game engine. This project serves as a learning platform for Rust programming and game development using the Entity Component System (ECS) architecture.

## Features

- **High Performance**: Built with Rust for maximum performance
- **Cross-Platform**: Runs on Windows, macOS, and Linux
- **ECS Architecture**: Uses Bevy's Entity Component System for clean, modular code
- **Modular Design**: Organized code structure for easy maintenance and learning

## Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.89.0 or later)
- Git

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd tower-defense-bevy
   ```

2. Run the game:
   ```bash
   cargo run
   ```

3. Run tests:
   ```bash
   cargo test
   ```

### Development

- **Check code**: `cargo check`
- **Lint code**: `cargo clippy`
- **Format code**: `cargo fmt`
- **Build release**: `cargo build --release`

## Project Structure

```
tower-defense-bevy/
├── src/
│   ├── components/     # ECS Components (Health, Position, etc.)
│   ├── systems/        # ECS Systems (Movement, Combat, etc.)
│   ├── resources/      # Global Resources (GameState, Score, etc.)
│   ├── game/          # Core game logic
│   └── utils/         # Utility functions
├── assets/            # Game assets (sprites, sounds, fonts)
├── tests/             # Unit tests
├── examples/          # Example code
└── docs/              # Documentation
```

## Learning Goals

This project is designed to teach:

- Rust programming fundamentals
- Entity Component System (ECS) patterns
- Game development concepts
- Performance optimization techniques
- Code organization and modularity

## Controls

- **ESC**: Exit game
- (More controls to be added as development progresses)

## Contributing

This is a learning project. Feel free to:

- Add new features
- Improve performance
- Fix bugs
- Add documentation
- Write tests

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments

- Built with [Bevy Engine](https://bevyengine.org/)
- Inspired by classic tower defense games