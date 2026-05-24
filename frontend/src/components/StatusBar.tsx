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
