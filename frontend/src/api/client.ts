// frontend/src/api/client.ts
// Only place in the codebase that calls fetch. Returns typed responses; throws ApiError on non-2xx.

import type { GameStateResponse, SetupConfig } from "../game/types";

const BASE = import.meta.env.VITE_API_BASE ?? "";

export class ApiError extends Error {
  status: number;
  constructor(status: number, message: string) {
    super(message);
    this.status = status;
    this.name = "ApiError";
  }
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE}${path}`, {
    headers: { "Content-Type": "application/json" },
    ...init,
  });
  if (!res.ok) {
    let msg = res.statusText;
    try {
      const body = await res.text();
      if (body) msg = body;
    } catch {
      // ignore
    }
    throw new ApiError(res.status, msg);
  }
  return res.json() as Promise<T>;
}

/** Reset the single global game with the given setup. */
export function createGame(config: SetupConfig): Promise<GameStateResponse> {
  return request("/games", {
    method: "POST",
    body: JSON.stringify(config),
  });
}

/** Play a column (column is 1-indexed, matching the moves string). */
export function playColumn(column1: number): Promise<GameStateResponse> {
  return request("/games/move", {
    method: "POST",
    body: JSON.stringify({ column: column1 }),
  });
}

/** Advance one ply in AI-vs-AI watch mode. */
export function aiStep(): Promise<GameStateResponse> {
  return request("/games/ai-step", { method: "POST" });
}
