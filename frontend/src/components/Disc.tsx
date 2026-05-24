// frontend/src/components/Disc.tsx
import type { CSSProperties } from "react";
import type { CellState } from "../game/types";

// Discriminated union: ghostColor is required when ghost is true, and
// disallowed otherwise. Prevents the "ghost-but-no-color" silent-fallback bug.
type Props =
  | {
      cell: CellState;
      ghost: true;
      ghostColor: "red" | "blue";
      winning?: boolean;
    }
  | {
      cell: CellState;
      ghost?: false;
      ghostColor?: never;
      winning?: boolean;
    };

export function Disc(props: Props) {
  const { cell, winning = false } = props;
  const ghost = props.ghost === true;
  const ghostColor = ghost ? props.ghostColor : undefined;
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
