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
      role="alert"
      aria-live="assertive"
      aria-atomic="true"
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
