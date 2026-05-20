# Connect 4 Frontend v1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the v1 Connect 4 web frontend — React + TS + Tailwind v4 — that talks to the WIP Axum server (single global game) and supports Human-vs-AI, AI-vs-AI watch mode, classic-arcade visuals, hover preview, and a feature-flagged hints toggle.

**Architecture:** Server-owned state. Frontend holds the last `GameStateResponse` and re-renders from it. One `useGame` hook owns mutable state; all `fetch` is isolated to `api/client.ts`; pure functions in `game/derive.ts` compute `lastMove`, `winningLine`, `legalColumns`.

**Tech Stack:** React 19, TypeScript, Vite 8, Tailwind CSS v4 (already installed via `@tailwindcss/vite`).

**Testing note:** The approved spec ([docs/superpowers/specs/2026-05-20-connect4-frontend-design.md](docs/superpowers/specs/2026-05-20-connect4-frontend-design.md)) explicitly defers automated tests for v1 ("No automated frontend tests in v1"). This plan therefore uses **type-check + manual smoke** for verification at each task instead of TDD. `game/derive.ts` is structured as pure functions so tests can be added later without restructuring.

**Server status:** Axum server is WIP — the assumed endpoints (`POST /games`, `POST /games/move`, `POST /games/ai-step`, `GET /games/hints`) may not yet exist. The frontend is built against the approved wire contract; integration debugging against the live server happens after the UI is in place. Each manual-smoke step says "if the server is up" — skip the network step if not.

**Reference paths:**
- Spec: [docs/superpowers/specs/2026-05-20-connect4-frontend-design.md](docs/superpowers/specs/2026-05-20-connect4-frontend-design.md)
- Existing scaffold: [frontend/](frontend/) (Vite + React 19 + Tailwind v4)
- Server types to mirror: `crates/connect4-server/src/...` (`CellState`, `GameState`, `GameStateResponse` enums/struct from the user's draft).

---

## File map

| File | Created/Modified | Responsibility |
|---|---|---|
| `frontend/.env.development` | Create | `VITE_API_BASE` for dev server URL. |
| `frontend/vite.config.ts` | Modify | Add dev proxy for `/api` (optional convenience). |
| `frontend/src/game/types.ts` | Create | Wire types (mirror server) + client-only setup config. |
| `frontend/src/game/derive.ts` | Create | Pure helpers: `lastMove`, `winningLine`, `legalColumns`, `boardRowsTopDown`. |
| `frontend/src/api/client.ts` | Create | Typed fetch wrappers, error class. |
| `frontend/src/game/useGame.ts` | Create | React hook owning all mutable game state. |
| `frontend/src/components/Disc.tsx` | Create | Single cell render. |
| `frontend/src/components/Board.tsx` | Create | 6×7 grid, hover preview, click → playColumn. |
| `frontend/src/components/StatusBar.tsx` | Create | Whose turn / winner / draw banner. |
| `frontend/src/components/Controls.tsx` | Create | New Game, Step/Auto (AI-vs-AI), Hints toggle (disabled). |
| `frontend/src/components/GameSetup.tsx` | Create | Mode/color/depth pickers. |
| `frontend/src/components/HintOverlay.tsx` | Create | Stub component, feature-flagged off. |
| `frontend/src/App.tsx` | Replace | Routes between Setup and Play, mounts hook. |
| `frontend/src/App.css` | Delete | Demo styles, no longer needed. |
| `frontend/src/index.css` | Modify | Replace scaffold theme with classic-arcade palette as CSS vars; remove `#root` width cap. |
| `frontend/src/assets/` | Delete (contents) | Demo PNG/SVG. |
| `frontend/public/icons.svg`, `vite.svg` | Delete | Scaffold leftovers referenced nowhere after App rewrite. |

---

## Task 1: Remove scaffold demo content and set up env

**Files:**
- Delete: `frontend/src/App.css`, `frontend/src/assets/hero.png`, `frontend/src/assets/react.svg`, `frontend/src/assets/vite.svg` (any others in assets/)
- Delete: `frontend/public/icons.svg`, `frontend/public/vite.svg` (if present)
- Create: `frontend/.env.development`
- Modify: `frontend/src/App.tsx` (replace with placeholder so the app still builds)

- [ ] **Step 1: Inspect what's in the scaffold to remove**

Run: `ls frontend/src/assets/ frontend/public/`
Expected: confirm filenames before deletion.

- [ ] **Step 2: Delete scaffold demo files**

```bash
rm -f frontend/src/App.css \
      frontend/src/assets/hero.png \
      frontend/src/assets/react.svg \
      frontend/src/assets/vite.svg \
      frontend/public/icons.svg \
      frontend/public/vite.svg
rmdir frontend/src/assets 2>/dev/null || true
```

- [ ] **Step 3: Create `frontend/.env.development`**

```
VITE_API_BASE=http://localhost:3000
```

(3000 is the conventional Axum dev port; adjust later if the server uses a different one.)

- [ ] **Step 4: Replace `frontend/src/App.tsx` with a placeholder**

```tsx
export default function App() {
  return (
    <main className="app-shell">
      <h1>Connect 4</h1>
      <p>Frontend scaffold in progress…</p>
    </main>
  );
}
```

- [ ] **Step 5: Verify the project still type-checks and builds**

Run: `cd frontend && npx tsc -b && npm run build`
Expected: build succeeds; no references to deleted files.

- [ ] **Step 6: Commit**

```bash
git add -A frontend/
git commit -m "frontend: strip Vite scaffold demo, add VITE_API_BASE"
```

---

## Task 2: Wire types (`game/types.ts`)

**Files:**
- Create: `frontend/src/game/types.ts`

- [ ] **Step 1: Create the types file**

```ts
// frontend/src/game/types.ts
// Wire types — mirror the server's CellState / GameState / GameStateResponse.

export type CellState = "red" | "blue" | "empty";

export type GameStatus = "in_progress" | "red_wins" | "blue_wins" | "draw";

export type GameStateResponse = {
  /** 1-indexed column sequence, e.g. "4434...". Empty string at game start. */
  moves: string;
  /**
   * board[row][col]. Row orientation is TBD — verified against the live server
   * at integration time. If row 0 turns out to be bottom (matching core),
   * Board.tsx will reverse before rendering.
   */
  board: CellState[][];
  state: GameStatus;
  /** Red | Blue in practice; the server typing permits Empty. */
  next_player: CellState;
};

// ----- Client-only setup config (not echoed by the server in v1) -----

export type Mode = "human_vs_ai" | "ai_vs_ai";
export type PlayerColor = "red" | "blue";

export type SetupConfig = {
  mode: Mode;
  /** Required for human_vs_ai; undefined for ai_vs_ai. */
  human_color?: PlayerColor;
  /** 1–10 plies. */
  red_depth?: number;
  blue_depth?: number;
};
```

- [ ] **Step 2: Type-check**

Run: `cd frontend && npx tsc -b`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add frontend/src/game/types.ts
git commit -m "frontend: add wire types mirroring server GameStateResponse"
```

---

## Task 3: Pure derive helpers (`game/derive.ts`)

**Files:**
- Create: `frontend/src/game/derive.ts`

- [ ] **Step 1: Create the file**

```ts
// frontend/src/game/derive.ts
// Pure helpers over GameStateResponse — no React, no fetch.

import type { CellState, GameStateResponse, PlayerColor } from "./types";

export const WIDTH = 7;
export const HEIGHT = 6;

/**
 * Normalize the board so row 0 is the BOTTOM of the physical board.
 * v1 assumption: server returns rows top-down (row 0 = top). If that turns out
 * to be reversed, flip the body of this function and Board.tsx still works.
 */
export function boardRowsBottomUp(board: CellState[][]): CellState[][] {
  return [...board].reverse();
}

/** Inverse of boardRowsBottomUp — for rendering top-down in the DOM. */
export function boardRowsTopDown(board: CellState[][]): CellState[][] {
  return board;
}

/** Columns (0-indexed) whose top row is empty — i.e. still playable. */
export function legalColumns(snapshot: GameStateResponse): number[] {
  const topRow = snapshot.board[0]; // top row in server orientation
  const legal: number[] = [];
  for (let c = 0; c < WIDTH; c++) {
    if (topRow[c] === "empty") legal.push(c);
  }
  return legal;
}

/**
 * Compute which row a disc would land in if dropped into `col` (0-indexed).
 * Returns null if the column is full. Works on the board as returned by the
 * server (row 0 = top in this implementation).
 */
export function landingRow(
  board: CellState[][],
  col: number,
): number | null {
  // Walk from bottom row upward; first empty cell is the landing slot.
  for (let r = HEIGHT - 1; r >= 0; r--) {
    if (board[r][col] === "empty") return r;
  }
  return null;
}

/** Parse the last move out of the moves string. Returns null on empty. */
export function lastMove(
  snapshot: GameStateResponse,
): { col: number; row: number; player: PlayerColor } | null {
  const { moves, board } = snapshot;
  if (moves.length === 0) return null;
  const lastChar = moves[moves.length - 1];
  const col1 = Number(lastChar);
  if (!Number.isInteger(col1) || col1 < 1 || col1 > WIDTH) return null;
  const col = col1 - 1;
  // Find the topmost filled cell in this column (lowest row index that's not empty).
  for (let r = 0; r < HEIGHT; r++) {
    if (board[r][col] !== "empty") {
      // Last move was placed by the player who is NOT next_player.
      // Derive from moves.length parity (odd = red just played, even = blue just played,
      // assuming red moves first — which matches connect4-core).
      const player: PlayerColor = moves.length % 2 === 1 ? "red" : "blue";
      return { col, row: r, player };
    }
  }
  return null;
}

/**
 * If the game has been won, return the 4 (row, col) cells of the winning line.
 * Otherwise null. Scans horizontals, verticals, and both diagonals.
 */
export function winningLine(
  snapshot: GameStateResponse,
): Array<[number, number]> | null {
  if (snapshot.state !== "red_wins" && snapshot.state !== "blue_wins") {
    return null;
  }
  const winner: PlayerColor = snapshot.state === "red_wins" ? "red" : "blue";
  const b = snapshot.board;

  const dirs: Array<[number, number]> = [
    [0, 1],  // horizontal
    [1, 0],  // vertical
    [1, 1],  // diag down-right
    [1, -1], // diag down-left
  ];

  for (let r = 0; r < HEIGHT; r++) {
    for (let c = 0; c < WIDTH; c++) {
      if (b[r][c] !== winner) continue;
      for (const [dr, dc] of dirs) {
        const cells: Array<[number, number]> = [];
        for (let k = 0; k < 4; k++) {
          const rr = r + dr * k;
          const cc = c + dc * k;
          if (rr < 0 || rr >= HEIGHT || cc < 0 || cc >= WIDTH) break;
          if (b[rr][cc] !== winner) break;
          cells.push([rr, cc]);
        }
        if (cells.length === 4) return cells;
      }
    }
  }
  return null;
}
```

- [ ] **Step 2: Type-check**

Run: `cd frontend && npx tsc -b`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add frontend/src/game/derive.ts
git commit -m "frontend: add pure derive helpers (lastMove, winningLine, legalColumns)"
```

---

## Task 4: API client (`api/client.ts`)

**Files:**
- Create: `frontend/src/api/client.ts`

- [ ] **Step 1: Create the file**

```ts
// frontend/src/api/client.ts
// Only place in the codebase that calls fetch. Returns typed responses; throws ApiError on non-2xx.

import type { GameStateResponse, SetupConfig } from "../game/types";

const BASE = import.meta.env.VITE_API_BASE ?? "";

export class ApiError extends Error {
  status: number;
  constructor(status: number, message: string) {
    super(message);
    this.status = status;
    this.name = "ApiError";
  }
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE}${path}`, {
    headers: { "Content-Type": "application/json" },
    ...init,
  });
  if (!res.ok) {
    let msg = res.statusText;
    try {
      const body = await res.text();
      if (body) msg = body;
    } catch {
      // ignore
    }
    throw new ApiError(res.status, msg);
  }
  return res.json() as Promise<T>;
}

/** Reset the single global game with the given setup. */
export function createGame(config: SetupConfig): Promise<GameStateResponse> {
  return request("/games", {
    method: "POST",
    body: JSON.stringify(config),
  });
}

/** Play a column (column is 1-indexed, matching the moves string). */
export function playColumn(column1: number): Promise<GameStateResponse> {
  return request("/games/move", {
    method: "POST",
    body: JSON.stringify({ column: column1 }),
  });
}

/** Advance one ply in AI-vs-AI watch mode. */
export function aiStep(): Promise<GameStateResponse> {
  return request("/games/ai-step", { method: "POST" });
}
```

- [ ] **Step 2: Type-check**

Run: `cd frontend && npx tsc -b`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add frontend/src/api/client.ts
git commit -m "frontend: add typed API client (createGame, playColumn, aiStep)"
```

---

## Task 5: `useGame` hook

**Files:**
- Create: `frontend/src/game/useGame.ts`

- [ ] **Step 1: Create the hook**

```ts
// frontend/src/game/useGame.ts
import { useCallback, useState } from "react";
import * as api from "../api/client";
import type { GameStateResponse, SetupConfig } from "./types";

export type UseGame = {
  snapshot: GameStateResponse | null;
  config: SetupConfig | null;
  loading: boolean;
  error: string | null;
  newGame: (config: SetupConfig) => Promise<void>;
  playColumn: (col0: number) => Promise<void>;
  aiStep: () => Promise<void>;
  reset: () => void;
};

export function useGame(): UseGame {
  const [snapshot, setSnapshot] = useState<GameStateResponse | null>(null);
  const [config, setConfig] = useState<SetupConfig | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const wrap = useCallback(async (fn: () => Promise<GameStateResponse>) => {
    setLoading(true);
    setError(null);
    try {
      const next = await fn();
      setSnapshot(next);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  const newGame = useCallback(
    async (cfg: SetupConfig) => {
      setConfig(cfg);
      await wrap(() => api.createGame(cfg));
    },
    [wrap],
  );

  const playColumn = useCallback(
    async (col0: number) => {
      await wrap(() => api.playColumn(col0 + 1));
    },
    [wrap],
  );

  const aiStep = useCallback(async () => {
    await wrap(() => api.aiStep());
  }, [wrap]);

  const reset = useCallback(() => {
    setSnapshot(null);
    setConfig(null);
    setError(null);
  }, []);

  return { snapshot, config, loading, error, newGame, playColumn, aiStep, reset };
}
```

- [ ] **Step 2: Type-check**

Run: `cd frontend && npx tsc -b`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add frontend/src/game/useGame.ts
git commit -m "frontend: add useGame hook owning all mutable game state"
```

---

## Task 6: Classic-arcade theme in `index.css`

**Files:**
- Modify: `frontend/src/index.css` (full replacement)

- [ ] **Step 1: Replace `frontend/src/index.css`**

```css
@import "tailwindcss";

:root {
  /* Classic arcade palette */
  --c4-frame: #1e40af;
  --c4-frame-shadow: #1234a0;
  --c4-hole: #0b1d52;
  --c4-red: #ef4444;
  --c4-yellow: #facc15;
  --c4-ghost: 0.35;

  --bg: #f7f7f8;
  --text: #1f2937;
  --text-muted: #6b7280;
  --border: #e5e7eb;

  --sans: system-ui, "Segoe UI", Roboto, sans-serif;
  --mono: ui-monospace, Consolas, monospace;

  font: 16px/1.45 var(--sans);
  color: var(--text);
  background: var(--bg);
  color-scheme: light;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

@media (prefers-color-scheme: dark) {
  :root {
    --bg: #0b0f1a;
    --text: #e5e7eb;
    --text-muted: #9ca3af;
    --border: #1f2937;
  }
}

body { margin: 0; }
h1, h2, h3 { font-family: var(--sans); color: var(--text); margin: 0; }

.app-shell {
  max-width: 720px;
  margin: 0 auto;
  padding: 32px 16px 64px;
  display: flex;
  flex-direction: column;
  gap: 24px;
  align-items: center;
}
```

- [ ] **Step 2: Verify build**

Run: `cd frontend && npm run build`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add frontend/src/index.css
git commit -m "frontend: replace scaffold theme with classic-arcade palette"
```

---

## Task 7: `Disc` component

**Files:**
- Create: `frontend/src/components/Disc.tsx`

- [ ] **Step 1: Create the component**

```tsx
// frontend/src/components/Disc.tsx
import type { CSSProperties } from "react";
import type { CellState } from "../game/types";

type Props = {
  cell: CellState;
  ghost?: boolean;        // hover preview
  winning?: boolean;      // part of the winning line
  ghostColor?: "red" | "blue";
};

export function Disc({ cell, ghost = false, winning = false, ghostColor }: Props) {
  // Style decisions live here so Board.tsx stays layout-only.
  const baseStyle: CSSProperties = {
    width: "100%",
    aspectRatio: "1 / 1",
    borderRadius: "50%",
    boxShadow: "inset 3px 4px 0 rgba(255,255,255,0.18), inset -3px -4px 0 rgba(0,0,0,0.25)",
    transition: "background 120ms ease, opacity 120ms ease",
  };

  let bg: string;
  let opacity = 1;
  if (ghost) {
    bg = ghostColor === "blue" ? "var(--c4-yellow)" : "var(--c4-red)";
    opacity = 0.35;
  } else if (cell === "red") {
    bg = "var(--c4-red)";
  } else if (cell === "blue") {
    bg = "var(--c4-yellow)";  // "Blue" player renders as yellow disc (classic arcade)
  } else {
    bg = "var(--c4-hole)";
  }

  const winStyle: CSSProperties = winning
    ? {
        boxShadow:
          "0 0 0 3px #fff, 0 0 18px 6px currentColor, inset -3px -4px 0 rgba(0,0,0,0.25)",
        animation: "c4-pulse 1.1s ease-in-out infinite",
      }
    : {};

  return (
    <div
      style={{ ...baseStyle, background: bg, opacity, ...winStyle, color: bg }}
      aria-hidden="true"
    />
  );
}
```

- [ ] **Step 2: Add the pulse keyframes to `index.css`**

Append to `frontend/src/index.css`:

```css
@keyframes c4-pulse {
  0%, 100% { transform: scale(1.0); filter: brightness(1.0); }
  50%      { transform: scale(1.08); filter: brightness(1.25); }
}
```

- [ ] **Step 3: Type-check**

Run: `cd frontend && npx tsc -b`
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add frontend/src/components/Disc.tsx frontend/src/index.css
git commit -m "frontend: add Disc component with ghost + winning states"
```

---

## Task 8: `Board` component

**Files:**
- Create: `frontend/src/components/Board.tsx`

- [ ] **Step 1: Create the component**

```tsx
// frontend/src/components/Board.tsx
import { useState } from "react";
import type { GameStateResponse, PlayerColor } from "../game/types";
import { HEIGHT, WIDTH, landingRow, legalColumns, winningLine } from "../game/derive";
import { Disc } from "./Disc";

type Props = {
  snapshot: GameStateResponse;
  /** If null, the board is read-only (e.g. AI-vs-AI watch mode, AI thinking). */
  onPlay: ((col0: number) => void) | null;
  /** Color of the disc shown in the hover-preview ghost. */
  ghostColor: PlayerColor;
};

export function Board({ snapshot, onPlay, ghostColor }: Props) {
  const [hoverCol, setHoverCol] = useState<number | null>(null);
  const winCells = winningLine(snapshot);
  const winSet = new Set((winCells ?? []).map(([r, c]) => `${r},${c}`));

  const legal = new Set(legalColumns(snapshot));
  const ghostRow =
    hoverCol !== null && legal.has(hoverCol)
      ? landingRow(snapshot.board, hoverCol)
      : null;

  const playable = onPlay !== null;

  return (
    <div
      style={{
        background: "var(--c4-frame)",
        boxShadow: "0 8px 0 var(--c4-frame-shadow), 0 12px 32px rgba(0,0,0,0.25)",
        padding: "16px",
        borderRadius: "16px",
        display: "grid",
        gridTemplateColumns: `repeat(${WIDTH}, 1fr)`,
        gap: "8px",
        width: "min(560px, 92vw)",
        aspectRatio: `${WIDTH} / ${HEIGHT}`,
      }}
      onMouseLeave={() => setHoverCol(null)}
    >
      {Array.from({ length: HEIGHT }).map((_, r) =>
        Array.from({ length: WIDTH }).map((_, c) => {
          const cell = snapshot.board[r][c];
          const isGhost = ghostRow !== null && hoverCol === c && r === ghostRow;
          const isWinning = winSet.has(`${r},${c}`);
          const clickable = playable && legal.has(c);
          return (
            <div
              key={`${r}-${c}`}
              onMouseEnter={() => setHoverCol(c)}
              onClick={() => clickable && onPlay!(c)}
              style={{
                cursor: clickable ? "pointer" : "default",
              }}
            >
              <Disc
                cell={cell}
                ghost={isGhost}
                ghostColor={ghostColor}
                winning={isWinning}
              />
            </div>
          );
        }),
      )}
    </div>
  );
}
```

- [ ] **Step 2: Type-check**

Run: `cd frontend && npx tsc -b`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add frontend/src/components/Board.tsx
git commit -m "frontend: add Board with column hover preview and win highlight"
```

---

## Task 9: `StatusBar` component

**Files:**
- Create: `frontend/src/components/StatusBar.tsx`

- [ ] **Step 1: Create the component**

```tsx
// frontend/src/components/StatusBar.tsx
import type { GameStateResponse } from "../game/types";

type Props = { snapshot: GameStateResponse; loading: boolean };

export function StatusBar({ snapshot, loading }: Props) {
  let text: string;
  let tone: "red" | "yellow" | "neutral" = "neutral";

  switch (snapshot.state) {
    case "red_wins":
      text = "🏆 Red wins";
      tone = "red";
      break;
    case "blue_wins":
      text = "🏆 Yellow wins";
      tone = "yellow";
      break;
    case "draw":
      text = "Draw";
      break;
    case "in_progress":
    default:
      if (snapshot.next_player === "red") {
        text = "Red to move";
        tone = "red";
      } else {
        text = "Yellow to move";
        tone = "yellow";
      }
  }

  const color =
    tone === "red" ? "var(--c4-red)" : tone === "yellow" ? "var(--c4-yellow)" : "var(--text)";

  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        gap: 12,
        fontSize: 22,
        fontWeight: 600,
        color,
        minHeight: 32,
      }}
    >
      <span>{text}</span>
      {loading && (
        <span style={{ fontSize: 14, color: "var(--text-muted)" }}>thinking…</span>
      )}
    </div>
  );
}
```

- [ ] **Step 2: Type-check**

Run: `cd frontend && npx tsc -b`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add frontend/src/components/StatusBar.tsx
git commit -m "frontend: add StatusBar showing turn / winner / draw"
```

---

## Task 10: `Controls` component

**Files:**
- Create: `frontend/src/components/Controls.tsx`

- [ ] **Step 1: Create the component**

```tsx
// frontend/src/components/Controls.tsx
import type { CSSProperties } from "react";
import type { Mode } from "../game/types";

type Props = {
  mode: Mode;
  gameOver: boolean;
  loading: boolean;
  hintsOn: boolean;
  onToggleHints: () => void;
  onNewGame: () => void;
  onAiStep?: () => void;
};

const BTN: CSSProperties = {
  // shared button look

  padding: "8px 14px",
  borderRadius: 8,
  border: "1px solid var(--border)",
  background: "var(--bg)",
  color: "var(--text)",
  cursor: "pointer",
  fontSize: 15,
};

const HINTS_DISABLED_TITLE =
  "Hints will light up once the server exposes /games/hints";

export function Controls({
  mode,
  gameOver,
  loading,
  hintsOn,
  onToggleHints,
  onNewGame,
  onAiStep,
}: Props) {
  return (
    <div style={{ display: "flex", gap: 12, flexWrap: "wrap", justifyContent: "center" }}>
      <button style={BTN} onClick={onNewGame}>New Game</button>

      {mode === "ai_vs_ai" && onAiStep && (
        <button style={BTN} onClick={onAiStep} disabled={gameOver || loading}>
          Step
        </button>
      )}

      <button
        style={{ ...BTN, opacity: 0.5, cursor: "not-allowed" }}
        title={HINTS_DISABLED_TITLE}
        aria-disabled="true"
        onClick={(e) => {
          e.preventDefault();
          // Toggle is wired up but hint data isn't available yet.
          // Keeping this call so the wiring is exercised; HintOverlay reads `hintsOn`.
          onToggleHints();
        }}
      >
        Hints {hintsOn ? "(on)" : "(off)"}
      </button>
    </div>
  );
}
```

- [ ] **Step 2: Type-check**

Run: `cd frontend && npx tsc -b`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add frontend/src/components/Controls.tsx
git commit -m "frontend: add Controls (New Game, AI step, Hints toggle stub)"
```

---

## Task 11: `HintOverlay` stub

**Files:**
- Create: `frontend/src/components/HintOverlay.tsx`

- [ ] **Step 1: Create the file**

```tsx
// frontend/src/components/HintOverlay.tsx
// Placeholder. Will render per-column eval bars once `GET /games/hints` exists.

type Props = { enabled: boolean };

export function HintOverlay({ enabled }: Props) {
  if (!enabled) return null;
  return (
    <div
      style={{
        fontSize: 13,
        color: "var(--text-muted)",
        fontStyle: "italic",
        textAlign: "center",
      }}
    >
      Hints will appear here once the server ships <code>/games/hints</code>.
    </div>
  );
}
```

- [ ] **Step 2: Type-check**

Run: `cd frontend && npx tsc -b`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add frontend/src/components/HintOverlay.tsx
git commit -m "frontend: add HintOverlay placeholder"
```

---

## Task 12: `GameSetup` component

**Files:**
- Create: `frontend/src/components/GameSetup.tsx`

- [ ] **Step 1: Create the file**

```tsx
// frontend/src/components/GameSetup.tsx
import { useState, type CSSProperties, type ReactNode } from "react";
import type { Mode, PlayerColor, SetupConfig } from "../game/types";

type Props = {
  onStart: (cfg: SetupConfig) => void;
  busy: boolean;
};

const SECTION: CSSProperties = {
  display: "flex",
  flexDirection: "column",
  gap: 8,
  width: "100%",
  maxWidth: 420,
};

const LABEL: CSSProperties = {
  fontSize: 13,
  textTransform: "uppercase",
  letterSpacing: 0.5,
  color: "var(--text-muted)",
};

export function GameSetup({ onStart, busy }: Props) {
  const [mode, setMode] = useState<Mode>("human_vs_ai");
  const [humanColor, setHumanColor] = useState<PlayerColor | "random">("red");
  const [redDepth, setRedDepth] = useState(6);
  const [blueDepth, setBlueDepth] = useState(6);

  function resolveHumanColor(): PlayerColor {
    if (humanColor === "random") return Math.random() < 0.5 ? "red" : "blue";
    return humanColor;
  }

  function start() {
    if (mode === "human_vs_ai") {
      const hc = resolveHumanColor();
      // The AI's depth is whichever side the human isn't.
      const aiDepth = redDepth; // single slider is fine in v1
      onStart({
        mode,
        human_color: hc,
        red_depth: hc === "red" ? undefined : aiDepth,
        blue_depth: hc === "blue" ? undefined : aiDepth,
      });
    } else {
      onStart({ mode, red_depth: redDepth, blue_depth: blueDepth });
    }
  }

  return (
    <section style={{ display: "flex", flexDirection: "column", gap: 20, alignItems: "center" }}>
      <h2 style={{ fontSize: 28, fontWeight: 600 }}>New game</h2>

      <div style={SECTION}>
        <span style={LABEL}>Mode</span>
        <div style={{ display: "flex", gap: 8 }}>
          <ModeButton active={mode === "human_vs_ai"} onClick={() => setMode("human_vs_ai")}>
            Play vs AI
          </ModeButton>
          <ModeButton active={mode === "ai_vs_ai"} onClick={() => setMode("ai_vs_ai")}>
            Watch AI vs AI
          </ModeButton>
        </div>
      </div>

      {mode === "human_vs_ai" && (
        <div style={SECTION}>
          <span style={LABEL}>You play</span>
          <div style={{ display: "flex", gap: 8 }}>
            <ModeButton active={humanColor === "red"} onClick={() => setHumanColor("red")}>Red</ModeButton>
            <ModeButton active={humanColor === "blue"} onClick={() => setHumanColor("blue")}>Yellow</ModeButton>
            <ModeButton active={humanColor === "random"} onClick={() => setHumanColor("random")}>Random</ModeButton>
          </div>
        </div>
      )}

      <div style={SECTION}>
        <span style={LABEL}>
          {mode === "human_vs_ai" ? "AI depth" : "Red depth"}: {redDepth}
        </span>
        <input
          type="range"
          min={1}
          max={10}
          value={redDepth}
          onChange={(e) => setRedDepth(Number(e.target.value))}
        />
      </div>

      {mode === "ai_vs_ai" && (
        <div style={SECTION}>
          <span style={LABEL}>Yellow depth: {blueDepth}</span>
          <input
            type="range"
            min={1}
            max={10}
            value={blueDepth}
            onChange={(e) => setBlueDepth(Number(e.target.value))}
          />
        </div>
      )}

      <button
        onClick={start}
        disabled={busy}
        style={{
          marginTop: 8,
          padding: "10px 20px",
          fontSize: 16,
          fontWeight: 600,
          borderRadius: 10,
          border: "none",
          background: "var(--c4-frame)",
          color: "white",
          cursor: busy ? "wait" : "pointer",
        }}
      >
        {busy ? "Starting…" : "Start"}
      </button>
    </section>
  );
}

function ModeButton({
  active,
  onClick,
  children,
}: {
  active: boolean;
  onClick: () => void;
  children: ReactNode;
}) {
  return (
    <button
      onClick={onClick}
      style={{
        flex: 1,
        padding: "8px 12px",
        borderRadius: 8,
        border: `1px solid ${active ? "var(--c4-frame)" : "var(--border)"}`,
        background: active ? "var(--c4-frame)" : "var(--bg)",
        color: active ? "white" : "var(--text)",
        cursor: "pointer",
        fontSize: 14,
      }}
    >
      {children}
    </button>
  );
}
```

- [ ] **Step 2: Type-check**

Run: `cd frontend && npx tsc -b`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add frontend/src/components/GameSetup.tsx
git commit -m "frontend: add GameSetup screen (mode, color, depth pickers)"
```

---

## Task 13: Wire it all together in `App.tsx`

**Files:**
- Modify: `frontend/src/App.tsx` (full replacement)

- [ ] **Step 1: Replace `App.tsx`**

```tsx
// frontend/src/App.tsx
import { useState } from "react";
import { useGame } from "./game/useGame";
import { GameSetup } from "./components/GameSetup";
import { Board } from "./components/Board";
import { StatusBar } from "./components/StatusBar";
import { Controls } from "./components/Controls";
import { HintOverlay } from "./components/HintOverlay";
import type { PlayerColor } from "./game/types";

export default function App() {
  const game = useGame();
  const [hintsOn, setHintsOn] = useState(false);

  const gameOver =
    game.snapshot !== null && game.snapshot.state !== "in_progress";

  if (!game.snapshot || !game.config) {
    return (
      <main className="app-shell">
        <h1 style={{ fontSize: 36, fontWeight: 700 }}>Connect 4</h1>
        <GameSetup onStart={game.newGame} busy={game.loading} />
        {game.error && <ErrorBanner message={game.error} />}
      </main>
    );
  }

  const isHumanTurn =
    game.config.mode === "human_vs_ai" &&
    !gameOver &&
    !game.loading &&
    game.snapshot.next_player === game.config.human_color;

  const ghostColor: PlayerColor =
    (game.snapshot.next_player === "blue" ? "blue" : "red");

  return (
    <main className="app-shell">
      <h1 style={{ fontSize: 32, fontWeight: 700 }}>Connect 4</h1>
      <StatusBar snapshot={game.snapshot} loading={game.loading} />
      <Board
        snapshot={game.snapshot}
        onPlay={isHumanTurn ? game.playColumn : null}
        ghostColor={ghostColor}
      />
      <Controls
        mode={game.config.mode}
        gameOver={gameOver}
        loading={game.loading}
        hintsOn={hintsOn}
        onToggleHints={() => setHintsOn((v) => !v)}
        onNewGame={game.reset}
        onAiStep={game.config.mode === "ai_vs_ai" ? game.aiStep : undefined}
      />
      <HintOverlay enabled={hintsOn} />
      {game.error && <ErrorBanner message={game.error} />}
    </main>
  );
}

function ErrorBanner({ message }: { message: string }) {
  return (
    <div
      style={{
        padding: "8px 14px",
        borderRadius: 8,
        background: "rgba(239, 68, 68, 0.12)",
        color: "var(--c4-red)",
        fontSize: 14,
        maxWidth: 560,
        textAlign: "center",
      }}
    >
      {message}
    </div>
  );
}
```

- [ ] **Step 2: Type-check + build**

Run: `cd frontend && npx tsc -b && npm run build`
Expected: PASS, no warnings about missing files.

- [ ] **Step 3: Commit**

```bash
git add frontend/src/App.tsx
git commit -m "frontend: wire Setup + Board + Status + Controls in App"
```

---

## Task 14: Manual smoke verification

No code changes — exercise the app and confirm behavior. Skip network steps if the server isn't running yet; the goal is to surface UI bugs early.

- [ ] **Step 1: Start the dev server**

Run: `cd frontend && npm run dev`
Expected: Vite reports a local URL (typically `http://localhost:5173`).

- [ ] **Step 2: Verify the Setup screen renders**

Open the URL. Confirm:
- "Connect 4" heading
- Mode toggle with two options
- Color picker visible (Red / Yellow / Random) for "Play vs AI"
- AI depth slider (1–10), defaulting to 6
- Switching to "Watch AI vs AI" hides the color picker and reveals the second depth slider
- "Start" button is enabled

- [ ] **Step 3: Verify the Play screen renders (only if the server is up)**

Click Start. With server running:
- StatusBar shows "Red to move" (or "Yellow to move" if human chose blue/random landed there)
- Board renders 6×7 grid with the classic-arcade blue frame
- Hovering a column shows a translucent ghost disc in the bottom-most empty slot
- Clicking a legal column triggers a request; on response, a real disc appears and StatusBar updates
- Clicking a full column does nothing
- Win state pulses the 4 winning discs and locks further clicks

If the server is not yet up, expect: clicking Start surfaces a red error banner ("Failed to fetch" or similar). That is the expected v1 behavior; the spec defers any retry/offline UX.

- [ ] **Step 4: Verify Controls**

- "New Game" returns to Setup
- "Hints" button shows the disabled tooltip and toggles its label (data stays empty — HintOverlay shows the placeholder copy)
- In AI-vs-AI mode, a "Step" button appears

- [ ] **Step 5: Commit any small fixes**

If any issue surfaces and you patch it, commit as `frontend: fix <bug>`.

---

## Self-review notes

Reviewed against the spec [docs/superpowers/specs/2026-05-20-connect4-frontend-design.md](docs/superpowers/specs/2026-05-20-connect4-frontend-design.md):

- **Spec §2 architecture** — covered by Tasks 4 (api), 5 (useGame), 13 (App composition).
- **Spec §3 wire types** — Task 2 mirrors `CellState`, `GameStatus`, `GameStateResponse` exactly; client-only `SetupConfig` defined alongside.
- **Spec §4 API surface** — Task 4 implements `createGame`, `playColumn`, `aiStep`; `/games/hints` is intentionally not implemented (feature-flagged).
- **Spec §5 frontend structure** — Tasks 7–13 create every file listed in the spec's module table. `HintOverlay.tsx` is included as a stub so the wire-up exists.
- **Spec §6 UX flow** — Setup→Play transition (Task 13), hover preview (Task 8), win-line highlight (Tasks 3 + 8), New Game / Step / Hints (Task 10), AI-vs-AI watch mode (Tasks 10 + 13).
- **Spec §7 error handling** — single ApiError thrown from `client.ts`, caught in `useGame`, rendered as ErrorBanner.
- **Spec §8 visual direction** — Task 6 (palette as CSS vars) + Tasks 7–8 (board frame, discs).
- **Spec §9 out of scope** — no persistence, no mobile-specific code, no animations beyond pulse + CSS transition: confirmed not present in any task.
- **Spec §10 testing** — automated tests deferred per spec; Task 14 covers manual smoke.

Open implementation question, surfaced at integration time only (intentionally not blocking):
- Board row orientation — `game/derive.ts` documents the assumption (row 0 = top); flipping is a one-line change in `boardRowsBottomUp` if the server returns the opposite.
