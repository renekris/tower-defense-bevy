# Tower Defense Game - Claude Instructions

## Project Overview
This is a tower defense game built with Bevy Engine and Rust. The project emphasizes learning Rust through game development with a focus on performance and cross-platform support.

## Environment Information
- **Platform**: Windows (MINGW64_NT) with Proxmox Container Environment
- **Language**: Rust (1.89.0) with Cargo
- **Game Engine**: Bevy 0.14
- **Working Directory**: G:\_Creativity\tower-defense-bevy

## Development Standards
- **Planning**: Always use TodoWrite tool for task tracking
- **Testing**: Test-driven development with `cargo test` - ALWAYS write failing tests first, then implement features to make them pass
- **Code Quality**: Run `cargo check`, `cargo clippy`, `cargo fmt` before commits
- **File Size**: Keep modules under 200 lines, use modular architecture
- **Comments**: Add extensive comments for learning purposes (this is a learning project)

## Key Commands
- `cargo run` - Run the game
- `cargo test` - Run tests
- `cargo check` - Check for compilation errors
- `cargo clippy` - Lint code
- `cargo fmt` - Format code
- `cargo build --release` - Build optimized release

## Bevy ECS Architecture
- **Entities**: Game objects (towers, enemies, projectiles)
- **Components**: Data attached to entities (Position, Health, Damage)
- **Systems**: Logic that operates on entities with specific components
- **Resources**: Global data shared across systems

## Learning Goals
- Understand Rust ownership and borrowing
- Learn ECS (Entity Component System) architecture
- Explore game development patterns
- Practice modular code organization
- Implement performance-critical game systems

## Important Notes
- This project is for learning Rust - add extensive comments
- Focus on clean, readable code over optimization initially
- Use Bevy's built-in systems when possible
- Test each feature as it's implemented
- Keep commits small and focused

## Unicode Handling
- Avoid Unicode emojis in console output due to Windows cp932 encoding
- Use ASCII alternatives: [OK], [ERROR], [WARN] instead of emoji symbols