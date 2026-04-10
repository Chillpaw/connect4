# Connect Four

Rust-first Connect Four with a bitboard-backed rules engine, terminal client, and minimax search (alpha–beta pruning). This repo is also the **core library** for a planned full-stack portfolio slice.

## What works today

- **Rules and state**: dual bitboards per player, column heights, [`try_play`](src/position.rs) returning `Result` for illegal moves
- **Win detection**: horizontal, vertical, and both diagonals via bit-twiddling
- **Search**: [`best_move`](src/minimax.rs) for the side to move, with center-first move ordering
- **CLI**: `cargo run` for a two-player terminal game

## Planned full-stack direction (portfolio)

The goal is to show a **common, interview-friendly stack** on top of this crate:

| Layer | Planned choice |
|--------|----------------|
| HTTP API | **Axum** (JSON REST, CORS, optional OpenAPI) |
| Persistence | **PostgreSQL** with **sqlx** (games / moves) |
| SPA | **React** + **Vite** + **TypeScript**, **TanStack Query** for server state |
| Local orchestration | **Docker Compose** (API + DB + static frontend in dev) |
| CI | **GitHub Actions** (`cargo test` / `clippy`, frontend `lint` / `build`) |

*Rust engine and HTTP API in one monorepo keeps a single source of truth for game logic.*

## Bitboard

The board uses a `u64` wrapper ([`Bitboard`](src/board.rs)): two instances per [`Position`](src/position.rs) (one per player). Bitboards implement `& | ^ !` and shifts for fast collision and pattern checks.

## Running

```bash
cargo test
cargo run
```

## Crate API (library)

```rust
use connect4::{best_move, Player, Position};

let mut pos = Position::new();
pos.try_play(3)?;
let m = best_move(&pos, 6);
```

## Licence

See [LICENSE](LICENSE).
