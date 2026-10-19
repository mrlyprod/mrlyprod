import { TILE } from './consts.js';

const GAP = 2;
const VAR = 16;
const JITTER = 0.4;
const BEND = 2;
const PACE = 0.5;

/* PROBE */

function decode(probe, w, h) {
  const n = w * h;
  const inside = new Uint8Array(n);
  const count = new Float32Array(n);
  const per = new Int32Array(n);
  for (let i = 0; i < n; i++) {
    const b = probe[4 * i + 2];
    inside[i] = b >= 128 ? 1 : 0;
    count[i] = ((probe[4 * i] << 8) | probe[4 * i + 1]) / 8;
    per[i] = ((b & 127) << 8) | probe[4 * i + 3];
  }
  const edge = new Uint8Array(n);
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const i = y * w + x;
      for (const [nx, ny] of [[x + 1, y], [x, y + 1]]) {
        if (nx >= w || ny >= h) continue;
        const j = ny * w + nx;
        if (inside[i] !== inside[j] || (!inside[i] && Math.abs(count[i] - count[j]) > 2)) {
          edge[i] = 1;
          edge[j] = 1;
        }
      }
    }
  }
  return { inside, count, per, edge };
}

function islands(inside, per, w, h) {
  const seen = new Uint8Array(w * h);
  const found = [];
  for (let s = 0; s < w * h; s++) {
    if (!inside[s] || seen[s]) continue;
    const stack = [s];
    seen[s] = 1;
    let touch = false;
    let sx = 0;
    let sy = 0;
    let n = 0;
    const votes = new Map();
    while (stack.length) {
      const i = stack.pop();
      const x = i % w;
      const y = (i - x) / w;
      if (x === 0 || y === 0 || x === w - 1 || y === h - 1) touch = true;
      sx += x;
      sy += y;
      n += 1;
      votes.set(per[i], (votes.get(per[i]) ?? 0) + 1);
      for (const [nx, ny] of [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]]) {
        if (nx < 0 || ny < 0 || nx >= w || ny >= h) continue;
        const j = ny * w + nx;
        if (inside[j] && !seen[j]) {
          seen[j] = 1;
          stack.push(j);
        }
      }
    }
    if (touch) continue;
    let period = 0;
    let most = 0;
    for (const [p, k] of votes) {
      if (k > most || (k === most && p < period)) {
        most = k;
        period = p;
      }
    }
    if (period > 0 && most * 2 >= n && n <= TILE * TILE) found.push({ x: sx / n + 0.5, y: sy / n + 0.5, n, period });
  }
  return found;
}

export function score(probe, w, h) {
  const { inside, count, per, edge } = decode(probe, w, h);
  const pools = islands(inside, per, w, h);
  const list = [];
  for (let ty = 0; ty * TILE < h; ty++) {
    for (let tx = 0; tx * TILE < w; tx++) {
      let n = 0;
      let held = 0;
      let rim = 0;
      let sum = 0;
      let sq = 0;
      let best = -1;
      let top = -1;
      for (let y = ty * TILE; y < Math.min(h, (ty + 1) * TILE); y++) {
        for (let x = tx * TILE; x < Math.min(w, (tx + 1) * TILE); x++) {
          const i = y * w + x;
          n += 1;
          rim += edge[i];
          if (inside[i]) {
            held += 1;
            continue;
          }
          sum += count[i];
          sq += count[i] * count[i];
          if (edge[i] && (best < 0 || count[i] > count[best])) best = i;
          if (top < 0 || count[i] > count[top]) top = i;
        }
      }
      const out = n - held;
      const mean = out ? sum / out : 0;
      const variance = out > 1 ? Math.max(0, sq / out - mean * mean) : 0;
      if (held > 0.9 * n || variance < 0.5) continue;
      const pool = pools.find((one) => Math.floor(one.x - 0.5) >= tx * TILE && Math.floor(one.x - 0.5) < (tx + 1) * TILE && Math.floor(one.y - 0.5) >= ty * TILE && Math.floor(one.y - 0.5) < (ty + 1) * TILE);
      const b = rim / n;
      const v = variance / (variance + VAR);
      const share = out / n;
      const s = b + v + (share >= 0.2 && share <= 0.8 ? 0.5 : 0) + 0.5 * b * v + (pool ? 1 : 0);
      const at = best >= 0 ? best : top;
      list.push({
        x: pool ? pool.x : (at % w) + 0.5,
        y: pool ? pool.y : Math.floor(at / w) + 0.5,
        ex: (at % w) + 0.5,
        ey: Math.floor(at / w) + 0.5,
        s,
        kind: pool ? 'island' : b * v > 0.25 ? 'spiral' : 'edge',
        period: pool ? pool.period : per[at],
        n: pool ? pool.n : 0,
        tx,
        ty,
      });
    }
  }
  return list.sort((a, b) => b.s - a.s);
}

export function spread(list, most, gap = GAP) {
  const kept = [];
  for (const one of list) {
    if (kept.length >= most) break;
    if (kept.some((k) => Math.max(Math.abs(k.tx - one.tx), Math.abs(k.ty - one.ty)) <= gap)) continue;
    kept.push(one);
  }
  return kept;
}

export function edge(probe, w, h) {
  const { inside, count, per, edge: rim } = decode(probe, w, h);
  let best = -1;
  let fallback = -1;
  for (let i = 0; i < w * h; i++) {
    if (inside[i] || !rim[i]) continue;
    const x = i % w;
    const y = (i - x) / w;
    const touches = [[x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]].some(([nx, ny]) => nx >= 0 && ny >= 0 && nx < w && ny < h && inside[ny * w + nx]);
    if (touches && (best < 0 || count[i] > count[best])) best = i;
    if (fallback < 0 || count[i] > count[fallback]) fallback = i;
  }
  const at = best >= 0 ? best : fallback;
  return at < 0 ? null : { x: (at % w) + 0.5, y: Math.floor(at / w) + 0.5, period: per[at] };
}

const turn = (a, b) => {
  const la = Math.hypot(a[0], a[1]);
  const lb = Math.hypot(b[0], b[1]);
  return la > 0 && lb > 0 ? (1 - (a[0] * b[0] + a[1] * b[1]) / (la * lb)) / 2 : 0;
};

const clamp = (to, reach) => {
  const far = Math.hypot(to[0], to[1]);
  return far <= reach ? to : [(to[0] * reach) / far, (to[1] * reach) / far];
};

export function prefer(probe, w, h, list, offset, heading, reach, rand = () => 0.5) {
  const { inside, edge: rim } = decode(probe, w, h);
  const worth = new Map(list.map((one) => [`${one.tx} ${one.ty}`, one.s * (1 - JITTER / 2 + JITTER * rand())]));
  const weigh = (to, s) => s - BEND * turn(heading, to) + (PACE * Math.hypot(to[0], to[1])) / reach;
  let best = null;
  for (let i = 0; i < w * h; i++) {
    if (inside[i] || !rim[i]) continue;
    const x = i % w;
    const y = (i - x) / w;
    const s = worth.get(`${Math.floor(x / TILE)} ${Math.floor(y / TILE)}`);
    const to = s === undefined ? null : offset(x + 0.5, y + 0.5);
    if (!to || Math.hypot(to[0], to[1]) > reach) continue;
    const r = weigh(to, s);
    if (!best || r > best.r) best = { to, r };
  }
  if (best) return best.to;
  for (const one of list) {
    const to = clamp(offset(one.ex, one.ey), reach);
    const r = weigh(to, worth.get(`${one.tx} ${one.ty}`));
    if (!best || r > best.r) best = { to, r };
  }
  return best?.to ?? null;
}
