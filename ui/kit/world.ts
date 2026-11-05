import type { Act, Envelope, Json, Manifest, Move, Obj, Who } from "@mrlyprod/os/kernel/index.ts";
import { canon } from "@mrlyprod/os/kernel/index.ts";
import { useCallback, useRef, useSyncExternalStore } from "react";

export type Outcome = { ok: boolean; note: string | null; envelope: Envelope | null };

export type Port = {
  subscribe: (heard: () => void) => () => void;
  read: (path?: string, shape?: Json) => Envelope;
  list: () => { moves: Move[]; apps: Manifest[] };
  call: (act: Act) => Outcome;
  who: () => Who;
  actor: () => boolean;
  at: () => number;
};

export type Signed = { who: Who; actor: boolean };

type Memo<T> = { version: number; key: string; text: string; value: T };

const NOWHERE: Outcome = { ok: false, note: "No world runs here", envelope: null };

let port: Port | null = null;
let version = 0;
let leave = () => {};
const heard = new Set<() => void>();

const ring = () => {
  version++;
  for (const one of heard) one();
};

export function plug(next: Port | null) {
  leave();
  port = next;
  leave = next ? next.subscribe(ring) : () => {};
  ring();
}

export const plugged = () => port;

const subscribe = (one: () => void) => {
  heard.add(one);
  return () => {
    heard.delete(one);
  };
};

function useSnap<T>(key: string, take: (port: Port) => T, none: T): T {
  const memo = useRef<Memo<T> | null>(null);
  const snap = () => {
    const was = memo.current;
    if (was && was.version === version && was.key === key) return was.value;
    const value = port ? take(port) : none;
    const text = canon(value);
    if (was && was.key === key && was.text === text) {
      memo.current = { ...was, version };
      return was.value;
    }
    memo.current = { version, key, text, value };
    return value;
  };
  return useSyncExternalStore(subscribe, snap, snap);
}

export const useWorld = (path = "", shape?: Json): Json | null =>
  useSnap(`${path}\n${shape === undefined ? "" : canon(shape)}`, (one) => one.read(path, shape).state, null);

export const useMoves = (app: string): Move[] =>
  useSnap(`moves:${app}`, (one) => one.list().moves.filter((move) => move.verb.startsWith(`${app}.`)), []);

export const useApp = (app: string): Manifest | null =>
  useSnap(`app:${app}`, (one) => one.list().apps.find((each) => each.id === app) ?? null, null);

export const useWho = (): Signed | null => useSnap("who", (one) => ({ who: one.who(), actor: one.actor() }), null);

export function useCall() {
  return useCallback((verb: string, args: Obj = {}): Outcome => {
    if (!port) return NOWHERE;
    return port.call({ verb, args, who: port.who(), at: Math.max(Date.now(), port.at()) });
  }, []);
}
