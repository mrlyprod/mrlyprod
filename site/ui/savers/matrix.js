import { veil } from './frame.js';

const CELL = 20;
const TICK = 33;
const CHARS = '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ';

export function fell(y, h, roll) {
  return y > h && roll > 0.975;
}

export function matrix(canvas, view) {
  const ctx = canvas.getContext('2d');
  const rand = view.rand;
  const drops = [];
  let cols = 0;
  const wide = () => view.w / view.dpr;
  const tall = () => view.h / view.dpr;
  const face = () => {
    ctx.setTransform(view.dpr, 0, 0, view.dpr, 0, 0);
    ctx.font = `${CELL}px MrlyFont, monospace`;
    ctx.textBaseline = 'alphabetic';
  };
  const wash = () => {
    ctx.fillStyle = view.look().paper;
    ctx.fillRect(0, 0, wide(), tall());
  };
  const size = () => {
    face();
    const next = Math.ceil(wide() / CELL);
    for (let i = cols; i < next; i++) drops[i] = cols ? rand() * (tall() / CELL) : 1;
    cols = next;
    wash();
  };
  const draw = () => {
    const skin = view.look();
    const h = tall();
    ctx.fillStyle = veil(skin.paper, 0.05);
    ctx.fillRect(0, 0, wide(), h);
    ctx.fillStyle = skin.accent;
    for (let i = 0; i < cols; i++) {
      const y = drops[i] * CELL;
      ctx.fillText(CHARS.charAt(Math.floor(rand() * CHARS.length)), i * CELL, y);
      if (fell(y, h, rand())) drops[i] = 0;
      drops[i]++;
    }
  };
  size();
  document.fonts?.ready.then(face);
  return { every: TICK, draw, size, theme: wash };
}
