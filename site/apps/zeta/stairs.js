import { veil } from '../../lib/scene.js';
import { AXIS, BEAT, CURVE, DASH, EDGE, FILL, GUESS, PAD, STAIR, fold, layout, phase, staircase, table } from './engine.js';

/* SCENE */

export function stairs(canvas, view, value, opts) {
  const ctx = canvas.getContext('2d');
  const plan = fold(opts.num, value.x, value.zeros);
  const { ladder, grid, gammas, zeros } = plan;
  const length = (zeros + 1) * BEAT;
  let origin = 0;
  let shown = zeros;
  let accent = '#000';
  let told = -1;
  let gone = false;
  const paint = () => {
    accent = view.look().accent;
  };
  const trace = (box, xs, ys) => {
    ctx.beginPath();
    for (let i = 0; i < xs.length; i++) {
      const x = box.left + (xs[i] - 1) * box.kx;
      const y = box.base - ys[i] * box.ky;
      if (i) ctx.lineTo(x, y);
      else ctx.moveTo(x, y);
    }
  };
  const axes = (box) => {
    ctx.strokeStyle = veil(accent, AXIS);
    ctx.lineWidth = view.dpr;
    ctx.beginPath();
    ctx.moveTo(0, box.base);
    ctx.lineTo(view.w, box.base);
    ctx.moveTo(box.left, 0);
    ctx.lineTo(box.left, view.h);
    ctx.stroke();
  };
  const tell = (count) => {
    if (count === told || gone) return;
    told = count;
    opts.onPass?.(count, count ? gammas[count - 1] : null);
  };
  const draw = () => {
    const dpr = view.dpr;
    const box = layout(plan, view.w, view.h, PAD * dpr);
    const { at, veil: fade } = phase(view.t - origin, length, view.still);
    const k = Math.min(zeros, Math.floor(at * (zeros + 1)));
    shown = k;
    ctx.setTransform(1, 0, 0, 1, 0, 0);
    ctx.clearRect(0, 0, view.w, view.h);
    ctx.lineJoin = 'round';
    axes(box);
    ctx.save();
    ctx.beginPath();
    ctx.rect(box.left, box.roof, box.right - box.left, box.base - box.roof);
    ctx.clip();
    ctx.globalAlpha = fade;
    trace(box, ladder.xs, ladder.ys);
    ctx.lineTo(box.right, box.base);
    ctx.lineTo(box.left, box.base);
    ctx.closePath();
    ctx.fillStyle = veil(accent, FILL);
    ctx.fill();
    trace(box, ladder.xs, ladder.ys);
    ctx.strokeStyle = veil(accent, EDGE);
    ctx.lineWidth = STAIR * dpr;
    ctx.stroke();
    ctx.setLineDash(DASH.map((d) => d * dpr));
    trace(box, grid, plan.curve(0));
    ctx.strokeStyle = veil(accent, GUESS);
    ctx.lineWidth = dpr;
    ctx.stroke();
    ctx.setLineDash([]);
    trace(box, grid, plan.curve(k));
    ctx.strokeStyle = accent;
    ctx.lineWidth = CURVE * dpr;
    ctx.stroke();
    ctx.restore();
    tell(k);
  };
  const again = () => {
    origin = view.t;
    if (view.still) view.wake?.();
    else draw();
  };
  const stop = () => {
    gone = true;
  };
  const svg = () => staircase(plan, { w: view.w / view.dpr, h: view.h / view.dpr, pad: PAD }, shown, accent);
  const csv = () => table({ zeros: gammas, first: plan.first });
  paint();
  return { draw, theme: paint, again, stop, svg, csv, facts: plan.facts };
}
