import { ink, fit, role } from './mrly.js';

export function board(canvas, height, { pad = 14, left = pad, right = pad, top = 26, bottom = 22 } = {}) {
  const [ctx, w, h] = fit(canvas, height);
  ctx.clearRect(0, 0, w, h);
  const floor = h - bottom, wide = w - left - right, tall = floor - top;
  const mono = getComputedStyle(document.body).getPropertyValue('--mono');
  ctx.font = `11px ${mono}`;
  return { ctx, w, h, left, right, roof: top, floor, wide, tall, mono, x: (f) => left + wide * f, y: (f) => floor - tall * f };
}

export function bars(b, values, { peak = Math.max(...values, 1e-12), color = ink.blue, inset = 1 } = {}) {
  const n = values.length, step = b.wide / n;
  const paint = typeof color === 'function' ? color : () => color;
  values.forEach((v, i) => {
    const tall = b.tall * v / peak;
    b.ctx.fillStyle = paint(i, v);
    b.ctx.fillRect(b.x(i / n) + inset, b.floor - tall, Math.max(1, step - 2 * inset), tall);
  });
}

export function line(b, points, color, { width = 1.5, dots = 0, dash = [], fill = 0 } = {}) {
  const { ctx } = b;
  const path = new Path2D();
  points.forEach(([fx, fy], i) => (i ? path.lineTo(b.x(fx), b.y(fy)) : path.moveTo(b.x(fx), b.y(fy))));
  ctx.strokeStyle = color;
  ctx.fillStyle = color;
  if (fill) {
    const area = new Path2D(path);
    area.lineTo(b.x(points.at(-1)[0]), b.y(0));
    area.lineTo(b.x(points[0][0]), b.y(0));
    ctx.globalAlpha = fill;
    ctx.fill(area);
    ctx.globalAlpha = 1;
  }
  ctx.lineWidth = width;
  ctx.setLineDash(dash);
  ctx.stroke(path);
  ctx.setLineDash([]);
  for (const [fx, fy] of dots ? points : []) {
    ctx.beginPath();
    ctx.arc(b.x(fx), b.y(fy), dots, 0, Math.PI * 2);
    ctx.fill();
  }
}

export function axis(b, labels = [], { wall = false } = {}) {
  const { ctx } = b;
  ctx.strokeStyle = ink.line;
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.moveTo(b.x(0), wall ? b.roof : b.floor);
  ctx.lineTo(b.x(0), b.floor);
  ctx.lineTo(b.x(1), b.floor);
  ctx.stroke();
  ctx.fillStyle = ink.dim;
  for (const [f, text] of labels) {
    ctx.textAlign = f <= 0 ? 'left' : f >= 1 ? 'right' : 'center';
    ctx.fillText(text, b.x(f), b.h - 6);
  }
  ctx.textAlign = 'left';
}

export function tag(b, text, color = ink.dim, align = 'left', x = align === 'right' ? b.x(1) : b.x(0), y = 14) {
  b.ctx.fillStyle = color;
  b.ctx.textAlign = align;
  b.ctx.fillText(text, x, y);
  b.ctx.textAlign = 'left';
  return x + b.ctx.measureText(text).width;
}

export function keep(fn) {
  let last = null;
  addEventListener('resize', () => {
    if (last) fn(...last);
  });
  return (...args) => {
    last = args;
    fn(...args);
  };
}

export function seek(canvas, onFrac, pad = 14) {
  const at = (event) => {
    const box = canvas.getBoundingClientRect();
    onFrac((event.clientX - box.left - pad) / (box.width - 2 * pad));
  };
  canvas.onpointerdown = (event) => {
    canvas.setPointerCapture(event.pointerId);
    at(event);
  };
  canvas.onpointermove = (event) => {
    if (event.buttons) at(event);
  };
}

export function web(canvas, height, nodes, branches, roles, radius) {
  const [ctx, w, h] = fit(canvas, height);
  ctx.clearRect(0, 0, w, h);
  const n = nodes.length / 2;
  if (!n) return;
  let [x0, y0, x1, y1] = [Infinity, Infinity, -Infinity, -Infinity];
  for (let i = 0; i < n; i++) {
    const x = nodes[2 * i], y = nodes[2 * i + 1];
    x0 = Math.min(x0, x);
    x1 = Math.max(x1, x);
    y0 = Math.min(y0, y);
    y1 = Math.max(y1, y);
  }
  const pad = radius + 8;
  const scale = Math.min((w - 2 * pad) / (x1 - x0 || 1), (h - 2 * pad) / (y1 - y0 || 1));
  const ox = (w - (x1 - x0) * scale) / 2 - x0 * scale, oy = (h - (y1 - y0) * scale) / 2 - y0 * scale;
  const px = (i) => ox + nodes[2 * i] * scale, py = (i) => oy + nodes[2 * i + 1] * scale;
  const lines = role.map(() => new Path2D());
  for (let k = 0; k < branches.length; k += 2) {
    const [a, b] = [branches[k], branches[k + 1]];
    const path = lines[roles ? Math.min(roles[a], roles[b]) : 0];
    path.moveTo(px(a), py(a));
    path.lineTo(px(b), py(b));
  }
  ctx.lineWidth = 1;
  lines.forEach((path, r) => {
    ctx.strokeStyle = roles ? role[r] : ink.line;
    ctx.stroke(path);
  });
  const dots = role.map(() => new Path2D());
  for (let i = 0; i < n; i++) {
    const path = dots[roles ? roles[i] : 2];
    path.moveTo(px(i) + radius, py(i));
    path.arc(px(i), py(i), radius, 0, Math.PI * 2);
  }
  dots.forEach((path, r) => {
    ctx.fillStyle = role[r];
    ctx.fill(path);
  });
}
