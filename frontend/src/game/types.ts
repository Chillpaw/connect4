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
