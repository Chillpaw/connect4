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
