import FONT from './font.json' with { type: 'json' };

export const FPS = 25;
export const HOLD = 25;
const BLANK = ['000', '000', '000', '000', '000'];
const STEPS = [[-1, 0], [0, 1], [1, 0], [0, -1]];

/* GLYPHS */

function glyph(char) {
  const rows = FONT[char];
  if (!rows) return BLANK;
  let from = Infinity;
  let to = 0;
  for (const row of rows) {
    for (let c = 0; c < row.length; c++) {
      if (row[c] !== '1') continue;
      from = Math.min(from, c);
      to = Math.max(to, c + 1);
    }
  }
  return from === Infinity ? BLANK : rows.map((row) => row.slice(from, to));
}

function layout(text) {
  const blocks = [];
  let col = 0;
  let height = 5;
  for (const char of text) {
    const rows = glyph(char);
    height = Math.max(height, rows.length);
    blocks.push({ char, rows, col, offset: 0 });
    col += rows[0].length + 1;
  }
  for (const block of blocks) block.offset = (height - block.rows.length) >> 1;
  return { width: blocks.length ? col - 1 : 0, height, blocks };
}

const bridge = (name) => globalThis.mrly?.[name];

/* RASTER */

const cells = (row) => (typeof row === 'string' ? [...row].map((c) => (c === '1' ? 1 : 0)) : row);

export function letters(text) {
  const fast = bridge('font_raster');
  if (fast) {
    const raster = JSON.parse(fast(text));
    return { rows: raster.rows, cols: raster.cols, grid: raster.grid.map(cells) };
  }
  const { width, height, blocks } = layout(text);
  const grid = Array.from({ length: width ? height : 0 }, () => Array(width).fill(0));
  for (const block of blocks) {
    block.rows.forEach((row, r) => {
      for (let c = 0; c < row.length; c++) if (row[c] === '1') grid[block.offset + r][block.col + c] = 1;
    });
  }
  return { rows: grid.length, cols: width, grid };
}

export function glyphSvg(text) {
  const { rows, cols, grid } = letters(text);
  const cells = [];
  grid.forEach((row, y) => row.forEach((on, x) => on && cells.push(`<rect x="${x}" y="${y}" width="1" height="1"/>`)));
  return `<svg class="glyphs" viewBox="0 0 ${cols} ${rows}" aria-hidden="true">${cells.join('')}</svg>`;
}

/* STROKES */

const PATHS = {
  M: [
    [[4, 0], [3, 0], [2, 0], [1, 0], [0, 0]],
    [[0, 1], [0, 2], [0, 3], [0, 4]],
    [[1, 4], [2, 4], [3, 4], [4, 4]],
    [[1, 2], [2, 2], [3, 2], [4, 2]],
  ],
  R: [
    [[4, 0], [3, 0], [2, 0], [1, 0], [0, 0]],
    [[0, 1], [0, 2], [0, 3], [0, 4]],
  ],
  L: [
    [[0, 0], [1, 0], [2, 0], [3, 0], [4, 0]],
    [[4, 1], [4, 2], [4, 3], [4, 4]],
  ],
  Y: [
    [[0, 0], [1, 0], [2, 0]],
    [[2, 1], [2, 2], [2, 3]],
    [[0, 4], [1, 4], [2, 4]],
    [[3, 4], [4, 4], [4, 3], [4, 2], [4, 1], [4, 0]],
  ],
  P: [
    [[4, 0], [3, 0], [2, 0], [1, 0], [0, 0]],
    [[0, 1], [0, 2], [0, 3], [0, 4]],
    [[1, 4]],
    [[2, 4], [2, 3], [2, 2], [2, 1]],
  ],
  O: [
    [[4, 0], [3, 0], [2, 0], [1, 0], [0, 0]],
    [[0, 1], [0, 2], [0, 3], [0, 4]],
    [[1, 4], [2, 4], [3, 4], [4, 4]],
    [[4, 3], [4, 2], [4, 1]],
  ],
  D: [
    [[2, 3], [2, 2], [2, 1], [2, 0]],
    [[3, 0], [4, 0]],
    [[4, 1], [4, 2], [4, 3], [4, 4]],
    [[3, 4], [2, 4], [1, 4], [0, 4]],
  ],
};

const key = (r, c) => r * 64 + c;

const unkey = (k) => [Math.floor(k / 64), k % 64];

function litOf(rows) {
  const left = new Set();
  rows.forEach((row, r) => {
    for (let c = 0; c < row.length; c++) if (row[c] === '1') left.add(key(r, c));
  });
  return left;
}

const step = (r, c, [dr, dc]) => (r + dr >= 0 && c + dc >= 0 ? key(r + dr, c + dc) : -1);

const degree = (r, c, left) => STEPS.filter((d) => left.has(step(r, c, d))).length;

function opening(left) {
  let best = null;
  let rank = null;
  for (const k of left) {
    const [r, c] = unkey(k);
    const now = [degree(r, c, left) !== 1 ? 1 : 0, -r, c];
    if (!rank || now[0] < rank[0] || (now[0] === rank[0] && (now[1] < rank[1] || (now[1] === rank[1] && now[2] < rank[2])))) {
      best = k;
      rank = now;
    }
  }
  return best;
}

function derive(rows) {
  const left = litOf(rows);
  const out = [];
  while (left.size) {
    const start = opening(left);
    left.delete(start);
    const stroke = [unkey(start)];
    let heading = null;
    for (;;) {
      const [r, c] = stroke[stroke.length - 1];
      let next = heading ? step(r, c, heading) : -1;
      if (!left.has(next)) next = STEPS.map((d) => step(r, c, d)).find((k) => left.has(k)) ?? -1;
      if (next < 0) break;
      const [nr, nc] = unkey(next);
      heading = [nr - r, nc - c];
      left.delete(next);
      stroke.push([nr, nc]);
    }
    out.push(stroke);
  }
  return out;
}

const sequence = (char, rows) => PATHS[char] ?? derive(rows);

/* ANIMATION */

const board = (laid, pad) => (laid.blocks.length ? [laid.height + 2 * pad, laid.width + 2 * pad] : [0, 0]);

export function animate(text, pad = 1) {
  const fast = bridge('font_animate');
  if (fast) return JSON.parse(fast(text, pad));
  const laid = layout(text);
  const [rows, cols] = board(laid, pad);
  const frames = [[]];
  const current = [];
  for (const block of laid.blocks) {
    for (const [r, c] of sequence(block.char, block.rows).flat()) {
      current.push((pad + block.offset + r) * cols + (pad + block.col + c));
      frames.push([...current].sort((a, b) => a - b));
    }
  }
  return { rows, cols, fps: FPS, frames };
}

function stamp(blocks, spots, rows, cols) {
  const active = new Set();
  blocks.forEach((block, i) => {
    const [cx, cy] = spots[i];
    block.rows.forEach((row, r) => {
      for (let c = 0; c < row.length; c++) {
        if (row[c] !== '1') continue;
        const y = cy + r;
        const x = cx + c;
        if (y >= 0 && y < rows && x >= 0 && x < cols) active.add(y * cols + x);
      }
    });
  });
  return [...active].sort((a, b) => a - b);
}

function beat(frame, total, phases) {
  const len = Math.floor(total / phases);
  for (let p = 1; p < phases; p++) if (frame < len * p) return [p, (frame - len * (p - 1)) / len];
  const done = len * (phases - 1);
  return [phases, (frame - done) / (total - done)];
}

const lerp = (start, end, p) => start + Math.trunc((end - start) * p);

function place(idx, n, phases, phase, progress, starts, targets) {
  const slide = (a, b) => [lerp(a[0], b[0], progress), lerp(a[1], b[1], progress)];
  if (phase < phases) {
    if (idx < phase) return slide(starts[phase - 1], starts[phase]);
    if (idx >= n - phase) return slide(starts[n - phase], starts[n - phase - 1]);
    return starts[idx];
  }
  const anchor = idx < phases ? starts[phases - 1] : idx >= n - phases ? starts[n - phases] : starts[idx];
  return slide(anchor, targets[idx]);
}

const same = (a, b) => a.length === b.length && a.every((v, i) => v === b[i]);

export function merge(text, pad = 1) {
  const laid = layout(text);
  const [rows, cols] = board(laid, pad);
  const n = laid.blocks.length;
  const phases = n >> 1;
  const starts = laid.blocks.map((b) => [pad + b.col, pad + b.offset]);
  const targets = laid.blocks.map((b) => [Math.floor((cols - b.rows[0].length) / 2), pad + b.offset]);
  if (!phases) return [stamp(laid.blocks, starts, rows, cols)];
  const total = cols >> 1;
  const frames = [];
  for (let i = 0; i <= total; i++) {
    const [phase, progress] = beat(i, total, phases);
    const spots = laid.blocks.map((_, idx) => place(idx, n, phases, phase, progress, starts, targets));
    const frame = stamp(laid.blocks, spots, rows, cols);
    if (!frames.length || !same(frames[frames.length - 1], frame)) frames.push(frame);
  }
  return frames;
}

export function cycle(text, pad = 1, hold = HOLD) {
  const fast = bridge('font_cycle');
  if (fast) return JSON.parse(fast(text, pad, hold));
  const write = animate(text, pad);
  const folded = merge(text, pad);
  const rest = (frame) => Array.from({ length: hold }, () => frame);
  const frames = [
    ...write.frames,
    ...rest(write.frames[write.frames.length - 1]),
    ...folded,
    ...rest(folded[folded.length - 1]),
    ...[...folded].reverse(),
    ...rest(folded[0]),
    ...[...write.frames].reverse(),
    ...rest(write.frames[0]),
  ];
  return { rows: write.rows, cols: write.cols, fps: write.fps, frames };
}

/* PLAYBACK */

export function mark(canvas, anim, color) {
  canvas.width = anim.cols;
  canvas.height = anim.rows;
  const ctx = canvas.getContext('2d');
  const draw = (frame) => {
    ctx.clearRect(0, 0, anim.cols, anim.rows);
    ctx.fillStyle = color ?? getComputedStyle(canvas).color;
    for (const i of frame) ctx.fillRect(i % anim.cols, Math.floor(i / anim.cols), 1, 1);
  };
  const full = anim.frames.reduce((a, b) => (b.length > a.length ? b : a), []);
  const still = matchMedia('(prefers-reduced-motion: reduce)');
  const shade = matchMedia('(prefers-color-scheme: dark)');
  if (still.matches || anim.frames.length < 2) {
    draw(full);
    const again = () => draw(full);
    const eye = new MutationObserver(again);
    eye.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
    shade.addEventListener('change', again);
    return () => {
      eye.disconnect();
      shade.removeEventListener('change', again);
    };
  }
  let at = 0;
  let timer = 0;
  const tick = () => {
    draw(anim.frames[at]);
    at = (at + 1) % anim.frames.length;
  };
  const play = () => {
    if (!timer) timer = setInterval(tick, 1000 / anim.fps);
  };
  const pause = () => {
    clearInterval(timer);
    timer = 0;
  };
  const eye = new IntersectionObserver(([entry]) => (entry.isIntersecting ? play() : pause()));
  eye.observe(canvas);
  return () => {
    pause();
    eye.disconnect();
  };
}
