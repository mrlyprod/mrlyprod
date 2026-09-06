import FONT from './font.json' with { type: 'json' };

const FPS = 25;
const BLANK = ['000', '000', '000', '000', '000'];

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
    blocks.push({ rows, col, offset: 0 });
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

/* ANIMATION */

export function animate(text, pad = 1) {
  const fast = bridge('font_animate');
  if (fast) return JSON.parse(fast(text, pad));
  const { width, height, blocks } = layout(text);
  if (!width) return { rows: 0, cols: 0, fps: FPS, frames: [[]] };
  const cols = width + 2 * pad;
  const frames = [[]];
  const lit = [];
  for (const block of blocks) {
    for (let c = 0; c < block.rows[0].length; c++) {
      for (let r = block.rows.length - 1; r >= 0; r--) {
        if (block.rows[r][c] !== '1') continue;
        lit.push((pad + block.offset + r) * cols + pad + block.col + c);
        frames.push([...lit].sort((a, b) => a - b));
      }
    }
  }
  return { rows: height + 2 * pad, cols, fps: FPS, frames };
}

export function cycle(text, pad = 1, hold = 40) {
  const fast = bridge('font_cycle');
  if (fast) return JSON.parse(fast(text, pad, hold));
  const write = animate(text, pad);
  const rest = (frame) => Array.from({ length: hold }, () => frame);
  const full = write.frames[write.frames.length - 1];
  return { ...write, frames: [...write.frames, ...rest(full), ...[...write.frames].reverse(), ...rest([])] };
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
