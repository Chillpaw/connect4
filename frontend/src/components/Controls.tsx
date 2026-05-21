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
