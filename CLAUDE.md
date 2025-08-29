# Claude Code Development Configuration

## Project Overview
**Tower Defense Game with Bevy + Rust**
- Location: `G:\_Creativity\tower-defense-bevy`
- Purpose: Learning Rust through game development with maximum performance
- Engine: Bevy 0.14 with ECS architecture
- Current Phase: Phase 1 Complete (Enemy System) - Ready for Phase 2 (Tower System)

## Current Project State
- **Branch**: `feature/basic-tower-defense` (merged to master)
- **Status**: Phase 1 Complete - Functional Enemy System implemented
- **Game Controls**: SPACE to spawn enemies, ESC to exit
- **Tests**: 48 tests passing with comprehensive TDD coverage
- **Architecture**: Clean ECS design ready for expansion

## Development Environment
- **Platform**: Windows (MINGW64_NT) with Proxmox Container Environment
- **Rust**: 1.89.0
- **Python**: 3.11.9 with uv package manager (preferred over pip)
- **Git**: Available (v2.49.0), no GitHub CLI
- **User Preference**: NO Claude credits in commit messages (keep clean)
- **Commit Style**: NO emojis in commit messages (keep professional)
- **Attribution Policy**: NEVER give credit to Claude in any form - commits, documentation, or comments
- **AI Assistance**: AI tools are internal development aids only - no external attribution required

## Project Structure
```
tower-defense-bevy/
├── src/
│   ├── game/           # Core game logic
│   ├── systems/        # ECS systems
│   ├── components/     # ECS components  
│   ├── resources/      # Game resources
│   └── utils/          # Utility functions
├── tests/              # Test files
├── assets/             # Game assets
│   ├── sprites/
│   ├── sounds/
│   ├── fonts/
│   └── levels/
└── docs/               # Documentation
```

## Development Standards
- **Testing**: Test-driven development with comprehensive test coverage
- **Code Quality**: Run `cargo test`, `cargo check`, `cargo clippy` before committing
- **File Size**: Keep files modular and under 200 lines
- **Planning**: Always use TodoWrite tool with clear progress tracking
- **Architecture**: Clean ECS patterns, separation of concerns

## Commit Requirements
- **NO AI Attribution**: Never include "Co-Authored-By: Claude" or similar AI credits
- **Clean History**: All commits should appear as if written entirely by the user
- **Professional Tone**: Focus on technical implementation details, not AI assistance
- **Content Only**: Commit messages should describe what was implemented, not how it was created

## MCP Servers Available
- **time**: mcp-server-time (globally installed)
- **git**: mcp-server-git (locally installed) 
- **memory**: @modelcontextprotocol/server-memory via npx
- **sequentialthinking**: https://remote.mcpservers.org/sequentialthinking/mcp
- **context7**: https://mcp.context7.com/mcp
- **filesystem**: @modelcontextprotocol/server-filesystem via npx
- **fetch**: mcp_server_fetch (Python module)

## Key Development Rules
1. **Reference existing code patterns** in repo
2. **Ask for clarification** if requirements unclear
3. **Test-driven development** - write failing tests first
4. **Run quality checks** before marking tasks complete
5. **Use absolute paths** for file operations
6. **Follow Rust/Bevy best practices** and ECS patterns

## Current Implementation Features
- Enemy spawning system with wave mechanics
- Path-based movement system
- Health and cleanup systems
- Visual rendering with sprites
- Comprehensive test suite
- Clean architecture ready for towers and projectiles

## Next Development Phase
**Phase 2**: Tower placement, projectile firing, collision detection
- Tower placement system on mouse click
- Projectile spawning and movement
- Collision detection between projectiles and enemies
- Resource management (money/score system)

## Important Notes
- User is complete beginner to Rust/Bevy - learning through this project
- Emphasis on educational comments and learning resources
- Windows console has Unicode issues - use ASCII characters only
- Session limits require saving progress to memory under 'setup' keyword
- Project emphasizes maximum performance with cross-platform support