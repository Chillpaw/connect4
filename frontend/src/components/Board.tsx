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
              {isGhost ? (
                <Disc
                  cell={cell}
                  ghost={true}
                  ghostColor={ghostColor}
                  winning={isWinning}
                />
              ) : (
                <Disc cell={cell} winning={isWinning} />
              )}
            </div>
          );
        }),
      )}
    </div>
  );
}
