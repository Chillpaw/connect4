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
