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
