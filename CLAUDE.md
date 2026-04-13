# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Quick Commands

```bash
cargo run              # Run the terminal client (two-player game)
cargo test             # Run all tests (77 unit tests across modules)
cargo test -p connect4 -- --nocapture  # Run tests with output
cargo build --release  # Build optimized binary
cargo clippy           # Lint code
```

Run a single test: `cargo test <test_name> -- --nocapture` (e.g., `cargo test bitboard_set --nocapture`)

## Architecture Overview

This is a **library-first** project: the core engine (`src/lib.rs`) exports a public API for game logic, with a standalone terminal client (`src/main.rs`) as the consumer. The planned full-stack direction (HTTP API, React frontend, PostgreSQL) will build on this crate as a shared dependency.

### Core Design: Bitboard Representation

The board uses **two `u64` integers** per `Position` (one per player) to represent disc placements:
- **Index mapping**: `index = row * WIDTH + column` with row 0 at the bottom
- **Valid play area**: 7 columns × 6 rows = 42 bits (using only the lower 42 bits of each `u64`)
- **Fast operations**: bitwise `&`, `|`, `^`, `!`, `<<`, `>>` for collision detection and pattern matching

Key insight: The bitboard design makes pattern-matching for wins (horizontal, vertical, diagonals) very efficient via bit-scanning, rather than iterating through the grid.

### Module Structure

| Module | Responsibility |
|--------|-----------------|
| **`board.rs`** | `Bitboard` wrapper: bit set/clear, bitwise ops, population count. Thin but complete encapsulation of `u64` logic. |
| **`position.rs`** | Game state: two bitboards per player, column heights for O(1) drop validation, `player_to_move` toggle, `try_play()` for move application. |
| **`win_detection.rs`** | Win detection using bit shifts and masks to scan horizontals, verticals, and both diagonals. |
| **`move_gen.rs`** | Legal moves: returns columns 0–6 where a disc can be placed; `legal_columns_ordered()` prioritizes center columns (heuristic for minimax pruning). |
| **`minimax.rs`** | Minimax search with alpha–beta pruning; `best_move()` is the public API. Heuristic evaluation counts "open pairs" (two adjacent discs with space to extend). Depth is counted in plies (half-moves). |
| **`engine.rs`** | Terminal driver: game loop, input parsing, win/draw detection, random first-player selection. |

### Key Public API

```rust
use connect4::{best_move, Position, PlayError, Player};

let mut pos = Position::new();           // Start game
pos.try_play(3)?;                        // Play column 3; Red moves first
let m = best_move(&pos, 6);              // Best move for Blue (depth=6 plies)
```

- **`Position`**: 56 bytes (two u64 bitboards + 7 column heights + player to move)
- **`try_play(col)`**: O(1) validation and state update; returns `Result<(), PlayError>`
- **`best_move(pos, depth)`**: Searches the minimax tree; `depth` is total plies from the position

### Search & Heuristics

- **Alpha–beta pruning**: Shallow pruning (center-first move ordering helps significantly)
- **Static evaluation** at leaf nodes: counts "open pairs" (adjacent discs with room to build)
  - Weighted differently for self vs. opponent (favors own progress)
  - Only applied when `depth == 0`; terminal states (win/loss/draw) are detected and return ±1.0 / 0.0

### Testing

- **Bitboard tests**: Set/clear, bitwise ops (77 tests total)
  - Bitboard logic is heavily tested due to correctness risk (De Morgan's laws, commutativity, shifts)
- **Position tests**: Implicit via minimax integration; no dedicated test module
- Tests use standard Rust `#[test]` and `#[should_panic]` attributes

## Planned Additions (Portfolio)

The codebase is designed to be extended with:
- **HTTP API** (Axum): JSON REST endpoints for move suggestions and game state
- **Persistence** (PostgreSQL + sqlx): store games and move history
- **Frontend** (React + Vite): interactive board, game replay, rating system
- **Orchestration** (Docker Compose): local development of API + database + static frontend
- **CI** (GitHub Actions): `cargo test` / `cargo clippy` on push and PR; frontend `lint` / `build`

The monorepo pattern (single source of truth for game rules in Rust) keeps the API and frontend in sync.

## Notes for Contributors

- **Bitboard logic is core**: changes to `board.rs` or bit patterns in win detection need careful review
- **Move ordering matters**: `legal_columns_ordered()` (center-first) is essential for alpha–beta performance
- **Depth terminology**: search depth is measured in **plies** (half-moves), not full moves; always clarify in comments
- **CI runs only on main**: `.github/workflows/rust.yml` triggers on push and PR to main; keep it passing
