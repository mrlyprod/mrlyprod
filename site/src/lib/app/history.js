/* HISTORY */

export const MERGE = 500;
const DEPTH = 100;

export const fresh = () => ({ past: [], future: [], at: -Infinity });

export function record(was, before, now) {
  const merged = was.past.length > 0 && now - was.at < MERGE;
  return { past: merged ? was.past : [...was.past, before].slice(-DEPTH), future: [], at: now };
}

export function undo(was, current) {
  if (!was.past.length) return null;
  return {
    stack: { past: was.past.slice(0, -1), future: [current, ...was.future], at: -Infinity },
    to: was.past.at(-1),
  };
}

export function redo(was, current) {
  if (!was.future.length) return null;
  return { stack: { past: [...was.past, current], future: was.future.slice(1), at: -Infinity }, to: was.future[0] };
}

export function patch(from, to) {
  const keys = new Set([...Object.keys(from), ...Object.keys(to)]);
  return Object.fromEntries([...keys].filter((key) => from[key] !== to[key]).map((key) => [key, to[key]]));
}
