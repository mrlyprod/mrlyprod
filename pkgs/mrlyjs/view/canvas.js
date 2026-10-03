import { CLEAR, board } from "./frame.js";

const TAU = 2 * Math.PI;

// PAINT

const css = (c) => `rgba(${c[0]},${c[1]},${c[2]},${c[3] / 255})`;

function surface(pixels) {
  const [h, w] = pixels.shape;
  const src = pixels.colors;
  const bytes = src instanceof Uint8ClampedArray ? src : new Uint8ClampedArray(src.buffer, src.byteOffset, w * h * 4);
  const out = new OffscreenCanvas(w, h);
  out.getContext("2d").putImageData(new ImageData(bytes, w, h), 0, 0);
  return out;
}

// CANVAS

export function canvas(ctx, width, height, ground = CLEAR) {
  ctx.clearRect(0, 0, width, height);
  if (ground[3] > 0) {
    ctx.fillStyle = css(ground);
    ctx.fillRect(0, 0, width, height);
  }

  function fill(c) {
    ctx.fillStyle = css(c);
    ctx.fill("evenodd");
  }

  function stroke(c, thick) {
    ctx.strokeStyle = css(c);
    ctx.lineWidth = thick;
    ctx.lineCap = "round";
    ctx.lineJoin = "round";
    ctx.stroke();
  }

  function path(pts) {
    ctx.beginPath();
    ctx.moveTo(pts[0][0], pts[0][1]);
    for (let i = 1; i < pts.length; i++) ctx.lineTo(pts[i][0], pts[i][1]);
  }

  function disc(cx, cy, r, c) {
    if (!(r > 0)) return;
    ctx.beginPath();
    ctx.arc(cx, cy, r, 0, TAU);
    fill(c);
  }

  function ring(cx, cy, r, thick, c) {
    if (r <= 0) return disc(cx, cy, r + thick / 2, c);
    ctx.beginPath();
    ctx.arc(cx, cy, r, 0, TAU);
    stroke(c, thick);
  }

  function polygon(pts, c) {
    if (pts.length < 3) return;
    path(pts);
    ctx.closePath();
    fill(c);
  }

  return {
    ...board(width, height),
    rect(x, y, w, h, c) {
      ctx.fillStyle = css(c);
      ctx.fillRect(x, y, w, h);
    },
    round_rect(x, y, w, h, r, c) {
      ctx.beginPath();
      ctx.roundRect(x, y, w, h, Math.max(Math.min(r, w / 2, h / 2), 0));
      fill(c);
    },
    disc,
    ring,
    segment(a, b, thick, c) {
      if (a[0] === b[0] && a[1] === b[1]) return disc(a[0], a[1], thick / 2, c);
      path([a, b]);
      stroke(c, thick);
    },
    polyline(pts, thick, c) {
      if (pts.length < 2) return;
      path(pts);
      stroke(c, thick);
    },
    triangle(a, b, c, color) {
      polygon([a, b, c], color);
    },
    polygon,
    arc(center, r, angles, thick, c) {
      const [cx, cy] = center;
      const lo = Math.min(angles[0], angles[1]);
      const span = Math.abs(angles[1] - angles[0]);
      if (span >= TAU) return ring(cx, cy, r, thick, c);
      if (span === 0) return disc(cx + r * Math.cos(lo), cy + r * Math.sin(lo), thick / 2, c);
      ctx.beginPath();
      ctx.arc(cx, cy, r, lo, lo + span);
      stroke(c, thick);
    },
    image(x, y, w, h, pixels) {
      const smooth = ctx.imageSmoothingEnabled;
      ctx.imageSmoothingEnabled = false;
      ctx.drawImage(surface(pixels), x, y, w, h);
      ctx.imageSmoothingEnabled = smooth;
    },
  };
}
