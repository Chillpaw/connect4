# Connect 4 Frontend — v1 Design

**Date:** 2026-05-20
**Status:** Approved for implementation planning
**Scope:** v1 web frontend for the Connect 4 game, talking to the WIP Axum server. Server-owned game state; single global game (no per-game ids in v1).

## 1. Goals

Build a React + TypeScript frontend that:

- Lets a human play Connect 4 against the existing minimax AI via the Axum server.
- Lets a human watch AI-vs-AI games at configurable depth.
- Is portfolio-quality (classic arcade visual direction: blue plastic frame, red & yellow discs).
- Has a clear seam for future hints and persistence features without redesign.

## 2. Architecture

Server-owned state. The Axum server is the source of truth for every position. The frontend holds the last `GameStateResponse` returned by the API and re-renders from it. No optimistic updates in v1.

```
┌─────────── React (Vite + TS + Tailwind v4) ─────────────┐    ┌──────── Axum ─────────┐
│  GameSetup ─► useGame() hook ─► api/client.ts           │HTTP►  POST /games          │
│                       ▼                                  │    │  POST /games/move    │
│   <Board /> <StatusBar /> <Controls /> <HintOverlay/>    │    │  POST /games/ai-step │
└──────────────────────────────────────────────────────────┘    │  GET  /games/hints   │
                                                                 │  (uses core minimax) │
                                                                 └──────────────────────┘
```

## 3. Wire types

Mirror the server's existing types exactly. Defined in `frontend/src/game/types.ts`:

```ts
export type CellState = "red" | "blue" | "empty";
export type GameStatus = "in_progress" | "red_wins" | "blue_wins" | "draw";

export type GameStateResponse = {
  moves: string;            // 1-indexed column sequence, e.g. "4434..."
  board: CellState[][];     // board[row][col]; row orientation TBD on first integration —
                            // core uses row 0 = bottom, but server JSON ordering must be
                            // confirmed by inspecting a response. Render order may need
                            // a reverse() in Board.tsx.
  state: GameStatus;
  next_player: CellState;   // Red | Blue in practice; typed permissively
};
```

Client-only setup config (not echoed by the server today):

```ts
export type Mode = "human_vs_ai" | "ai_vs_ai";
export type Color = "red" | "blue";

export type SetupConfig = {
  mode: Mode;
  human_color?: Color;     // human_vs_ai only
  red_depth?: number;      // 1–10 plies
  blue_depth?: number;     // 1–10 plies
};
```

### Server additions needed later (not implemented as part of this spec)

These are **not** required for v1 UI to work — the frontend derives them locally or holds them client-side — but listing for the server's planning:

- A `game_id` field if/when multiple games are supported.
- `winning_line: [row, col][]` — for win-highlight without re-running win detection client-side.
- `last_move: { column, row, player }` — convenience for animations.
- Echo of `mode` / depths so a refresh restores the game type.
- `GET /games/hints` endpoint returning `{ evaluations: [{ column, score }] }` for the current position.

Frontend will feature-flag the hints toggle off until that endpoint exists.

## 4. API surface (assumed)

| Method | Path | Body | Returns | Notes |
|---|---|---|---|---|
| `POST` | `/games` | `SetupConfig` | `GameStateResponse` | Resets the single global game. |
| `POST` | `/games/move` | `{ column: 1..=7 }` | `GameStateResponse` | Server validates, applies, and — in `human_vs_ai` mode — also plays the AI's response before returning. |
| `POST` | `/games/ai-step` | `{}` | `GameStateResponse` | AI-vs-AI watch mode: advance one ply. |
| `GET`  | `/games/hints` | — | `{ evaluations: [{ column: 1..=7, score: number }] }` | **Future.** Hint toggle is hidden/disabled until this exists. |

Column numbers on the wire are **1-indexed** to match the existing `moves` string format. The board UI works internally in 0-indexed columns and converts at the API boundary.

## 5. Frontend structure

```
frontend/src/
├── api/
│   └── client.ts          # fetch wrappers, typed request/response
├── game/
│   ├── types.ts           # CellState, GameStatus, GameStateResponse, Mode, Color
│   ├── derive.ts          # last-move, winning-line, legal-columns from board + moves
│   └── useGame.ts         # React hook: state + actions (newGame, playColumn, aiStep)
├── components/
│   ├── App.tsx            # top-level layout; routes between Setup and Play
│   ├── GameSetup.tsx      # mode + color + depth pickers
│   ├── Board.tsx          # 6×7 grid, column hover preview, click handler
│   ├── Disc.tsx           # one cell (red/blue/empty + winning-highlight prop)
│   ├── StatusBar.tsx      # whose turn / winner / draw banner
│   ├── Controls.tsx       # New Game, Step / Auto (AI-vs-AI), Hints toggle
│   └── HintOverlay.tsx    # per-column eval bars (feature-flagged off in v1)
└── index.css              # Tailwind entry; classic arcade palette as CSS vars
```

### Module responsibilities

- **`api/client.ts`** — only place that calls `fetch`. Functions: `createGame(SetupConfig)`, `playColumn(col1to7)`, `aiStep()`, (future) `getHints()`. Throws on non-2xx; returns typed `GameStateResponse`.
- **`game/useGame.ts`** — only place that holds mutable game state. Exposes `{ snapshot, config, status, error, newGame, playColumn, aiStep }`. Components are pure renderers.
- **`game/derive.ts`** — pure functions:
  - `lastMove(snapshot) → { column0, row, player } | null` (parses last char of `moves`, scans column for top filled cell).
  - `winningLine(snapshot) → [row, col][] | null` (runs a 4-in-a-row scan when `state` is a win).
  - `legalColumns(snapshot) → number[]` (columns whose top row is `empty`).
- **Components** — stateless beyond local UI state (hover column, hint-toggle on/off). Render from `snapshot` + `config`.

## 6. UX flow

1. **First visit / after New Game** — Setup screen:
   - Mode toggle: "Play vs AI" / "Watch AI vs AI"
   - If `human_vs_ai`: color picker (Red / Blue / Random). AI plays the other color.
   - Depth slider(s): one for AI in human-vs-AI; two in AI-vs-AI.
   - "Start" → `POST /games` → Play screen.
2. **Play screen** (classic arcade visual: deep-blue board, red & yellow discs):
   - **StatusBar** above the board: "Red's turn" / "Blue's turn" / "🏆 Red wins" / "Draw".
   - **Board**: hover a column → ghost disc shown in the column's landing slot (computed via `legalColumns` + column scan). Click a legal column in human-vs-AI when it's the human's turn → `POST /games/move`.
   - **Winning line**: when `state` becomes a win, the 4 cells from `winningLine(snapshot)` pulse / glow.
   - **Controls** below the board:
     - "New Game" (always available) — returns to Setup.
     - "Hints" toggle — visible but disabled with "coming soon" tooltip until server ships `/games/hints`.
     - AI-vs-AI mode only: "Step" (one ply) and "Auto" (loops ~700 ms between calls until terminal).
3. **AI-vs-AI watch mode** — board does not accept clicks; columns don't show hover ghost. "Step"/"Auto" drive `POST /games/ai-step`.

## 7. Error handling

Only at API boundaries:

- **Illegal move / 4xx** — small toast: "That column is full" (or server-provided message). UI does not change state.
- **Network / 5xx** — toast: "Couldn't reach the server. Try again." Last successful snapshot remains rendered.
- **No client-side validation** beyond disabling clicks on full columns. The server is authoritative.

There are no other internal error states because the client never holds derived state independently of `snapshot`.

## 8. Visual direction

Classic arcade (selected from mockups):

- Board frame: deep blue (`#1e40af`), rounded corners, subtle inset shadow.
- Empty cells: darker blue circles (`#0b1d52`) with a soft inner shadow.
- Red disc: `#ef4444` with inset highlight for depth.
- Yellow disc: `#facc15` with inset highlight for depth.
- Ghost-preview disc on hover: same color as the player to move, at ~30% opacity.
- Winning-line highlight: animated outer glow on the 4 discs.

Palette will live as CSS variables in `index.css` so it can be themed later without touching components.

## 9. Out of scope for v1

- Persistence / database / replays.
- Multiple concurrent games / accounts / auth.
- Mobile-optimized layout (desktop-first; usable on mobile but not tuned).
- Sound effects.
- Animations beyond disc-drop transition (CSS `transform`) and winning-line glow.
- Hints UI rendering live data (toggle is shipped disabled and wired up; rendering lights up when `/games/hints` exists).

## 10. Testing

Manual / smoke for v1:

- Start a game in each mode and play to win, loss, and draw.
- Verify illegal-column clicks are blocked client-side and the server is not called.
- Verify that refreshing the page calls `GET`-equivalent (or re-creates) without crashing — current single-global-game model means the server's last state is shown.

No automated frontend tests in v1. `game/derive.ts` is structured as pure functions so unit tests can be added later without restructuring.
