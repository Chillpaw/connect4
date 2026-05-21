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
