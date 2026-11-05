export type Level = "off" | "low" | "high";

export const POOL = 4096;

export const BUDGET: Record<Exclude<Level, "off">, { pool: number; trail: number; scale: number }> = {
  low: { pool: 1024, trail: 1, scale: 0.4 },
  high: { pool: POOL, trail: 4, scale: 1 },
};

export const FLOATS = 12;

const DOT = 3;

const GRAVITY = 1500;

const DRAG = 1.6;

const VERTEX = `#version 300 es
precision highp float;
layout(location = 0) in vec2 corner;
layout(location = 1) in vec4 a;
layout(location = 2) in vec4 b;
layout(location = 3) in vec4 c;
uniform float now;
uniform vec2 view;
uniform float trail;
uniform float light;
out vec2 local;
out vec4 ink;
out float core;
void main() {
  float ghost = mod(float(gl_InstanceID), trail);
  float age = now - b.x - ghost * 0.022;
  if (age < 0.0 || age > b.y) {
    gl_Position = vec4(2.0, 2.0, 0.0, 1.0);
    return;
  }
  float f = age / b.y;
  float glide = (1.0 - exp(-${DRAG.toFixed(1)} * age)) / ${DRAG.toFixed(1)};
  vec2 p = a.xy + a.zw * glide + vec2(0.0, 0.5 * ${GRAVITY.toFixed(1)} * c.w * age * age);
  p = floor(p / ${DOT.toFixed(1)}) * ${DOT.toFixed(1)} + ${(DOT / 2).toFixed(1)};
  float size = max(${DOT.toFixed(1)}, floor(b.z * (1.0 - 0.6 * f) / ${DOT.toFixed(1)}) * ${DOT.toFixed(1)});
  if (ghost > 0.0) size = ${DOT.toFixed(1)};
  if (light > 0.5) size += ${DOT.toFixed(1)};
  float reach = size * 0.5 + ${(DOT * 3).toFixed(1)};
  vec2 q = p + corner * reach;
  gl_Position = vec4(q.x / view.x * 2.0 - 1.0, 1.0 - q.y / view.y * 2.0, 0.0, 1.0);
  local = corner * reach;
  core = size * 0.5;
  float fade = (floor((1.0 - f) * 4.0) + 1.0) / 5.0;
  fade *= 1.0 - ghost / trail;
  vec3 hot = light > 0.5 ? c.rgb * 0.45 : vec3(1.0);
  ink = vec4(mix(hot, c.rgb, clamp(f * 2.5 + b.w, 0.0, 1.0)), fade);
}`;

const FRAGMENT = `#version 300 es
precision mediump float;
in vec2 local;
in vec4 ink;
in float core;
uniform highp float light;
out vec4 color;
void main() {
  vec2 d = abs(local) - vec2(core);
  float edge = max(d.x, d.y);
  float a = 1.0;
  if (edge > 0.0) {
    if (light > 0.5) discard;
    a = 0.42 * pow(0.45, floor(edge / ${DOT.toFixed(1)}));
  }
  float alpha = a * ink.a;
  if (light > 0.5) color = vec4(ink.rgb * alpha, alpha);
  else color = vec4(ink.rgb * alpha, alpha * 0.35);
}`;

let seed = 0x2545f491;

export const roll = () => {
  seed ^= seed << 13;
  seed ^= seed >>> 17;
  seed ^= seed << 5;
  return (seed >>> 0) / 4294967296;
};

export type Spray = {
  x: number;
  y: number;
  n: number;
  angle: number;
  spread: number;
  speed: [number, number];
  life: [number, number];
  size: [number, number];
  fall?: number;
  push?: [number, number];
  hot?: number;
};

export class Ring {
  data = new Float32Array(POOL * FLOATS);
  head = 0;
  used = 0;
  from = -1;
  count = 0;
  until = 0;
  pool = POOL;
  inks: number[][] = [[1, 0.85, 0.1]];

  write(spray: Spray, now: number) {
    const { x, y, n, angle, spread, speed, life, size, fall = 1, push = [0, 0], hot = 0 } = spray;
    for (let k = 0; k < n; k++) {
      const i = this.head;
      const o = i * FLOATS;
      const turn = angle + (roll() - 0.5) * spread;
      const v = speed[0] + roll() * (speed[1] - speed[0]);
      const floater = roll() < 0.12;
      const span = life[0] + roll() * (life[1] - life[0]);
      const ink = this.inks[Math.floor(roll() * this.inks.length)]!;
      const d = this.data;
      d[o] = x + (roll() - 0.5) * 4;
      d[o + 1] = y + (roll() - 0.5) * 4;
      d[o + 2] = Math.cos(turn) * v * (floater ? 0.25 : 1) + push[0];
      d[o + 3] = Math.sin(turn) * v * (floater ? 0.25 : 1) + push[1];
      d[o + 4] = now;
      d[o + 5] = floater ? span * 2.4 : span;
      d[o + 6] = size[0] + roll() * (size[1] - size[0]);
      d[o + 7] = hot;
      d[o + 8] = ink[0]!;
      d[o + 9] = ink[1]!;
      d[o + 10] = ink[2]!;
      d[o + 11] = floater ? -0.05 : fall;
      this.until = Math.max(this.until, now + d[o + 5]!);
      if (this.from < 0) this.from = i;
      this.count = Math.min(this.pool, this.count + 1);
      this.head = (i + 1) % this.pool;
      this.used = Math.max(this.used, this.head === 0 ? this.pool : this.head);
    }
  }

  dirty(): [number, number][] {
    if (this.from < 0) return [];
    const from = this.from;
    const count = this.count;
    this.from = -1;
    this.count = 0;
    if (count >= this.pool) return [[0, this.pool]];
    if (from + count <= this.pool) return [[from, count]];
    return [
      [from, this.pool - from],
      [0, from + count - this.pool],
    ];
  }

  resize(pool: number) {
    this.pool = pool;
    this.head = 0;
    this.used = 0;
    this.from = -1;
    this.count = 0;
    this.until = 0;
  }
}

export type Sparks = {
  ring: Ring;
  spray: (spray: Spray) => void;
  level: (level: Level) => void;
  theme: (inks: number[][], light: boolean) => void;
  fit: () => void;
  pause: (on: boolean) => void;
  stop: () => void;
};

function compile(gl: WebGL2RenderingContext, type: number, source: string) {
  const shader = gl.createShader(type)!;
  gl.shaderSource(shader, source);
  gl.compileShader(shader);
  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) throw new Error(gl.getShaderInfoLog(shader) ?? "shader");
  return shader;
}

export function sparks(canvas: HTMLCanvasElement): Sparks | null {
  const gl = canvas.getContext("webgl2", {
    alpha: true,
    premultipliedAlpha: true,
    antialias: false,
    depth: false,
    stencil: false,
    powerPreference: "low-power",
  });
  if (!gl) return null;
  const program = gl.createProgram()!;
  gl.attachShader(program, compile(gl, gl.VERTEX_SHADER, VERTEX));
  gl.attachShader(program, compile(gl, gl.FRAGMENT_SHADER, FRAGMENT));
  gl.linkProgram(program);
  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) throw new Error(gl.getProgramInfoLog(program) ?? "link");
  const where = {
    now: gl.getUniformLocation(program, "now"),
    view: gl.getUniformLocation(program, "view"),
    trail: gl.getUniformLocation(program, "trail"),
    light: gl.getUniformLocation(program, "light"),
  };
  const ring = new Ring();
  const vao = gl.createVertexArray();
  gl.bindVertexArray(vao);
  const quad = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, quad);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]), gl.STATIC_DRAW);
  gl.enableVertexAttribArray(0);
  gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
  const pool = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, pool);
  gl.bufferData(gl.ARRAY_BUFFER, ring.data.byteLength, gl.DYNAMIC_DRAW);
  const divide = (trail: number) => {
    for (const at of [1, 2, 3]) {
      gl.enableVertexAttribArray(at);
      gl.vertexAttribPointer(at, 4, gl.FLOAT, false, FLOATS * 4, (at - 1) * 16);
      gl.vertexAttribDivisor(at, trail);
    }
  };
  let budget = BUDGET.high;
  divide(budget.trail);
  gl.useProgram(program);
  gl.disable(gl.DEPTH_TEST);
  gl.enable(gl.BLEND);
  gl.clearColor(0, 0, 0, 0);
  let light = false;
  let off = false;
  let paused = typeof document !== "undefined" && document.hidden;
  let frame = 0;
  let lost = false;
  let width = 1;
  let height = 1;
  const born = performance.now();
  const clock = (ms = performance.now()) => (ms - born) / 1000;
  const blend = () =>
    light ? gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA) : gl.blendFuncSeparate(gl.ONE, gl.ONE, gl.ONE, gl.ONE);
  blend();
  const fit = () => {
    const dpr = Math.min(2, devicePixelRatio || 1);
    width = Math.max(1, innerWidth);
    height = Math.max(1, innerHeight);
    canvas.width = Math.round(width * dpr);
    canvas.height = Math.round(height * dpr);
    gl.viewport(0, 0, canvas.width, canvas.height);
  };
  fit();
  const flush = () => {
    gl.bindBuffer(gl.ARRAY_BUFFER, pool);
    for (const [from, count] of ring.dirty())
      gl.bufferSubData(gl.ARRAY_BUFFER, from * FLOATS * 4, ring.data, from * FLOATS, count * FLOATS);
  };
  const draw = (ms: number) => {
    frame = 0;
    if (paused || lost) return;
    const now = clock(ms);
    flush();
    gl.clear(gl.COLOR_BUFFER_BIT);
    if (now > ring.until || off) return;
    gl.uniform1f(where.now, now);
    gl.uniform2f(where.view, width, height);
    gl.uniform1f(where.trail, budget.trail);
    gl.uniform1f(where.light, light ? 1 : 0);
    gl.drawArraysInstanced(gl.TRIANGLE_STRIP, 0, 4, ring.used * budget.trail);
    frame = requestAnimationFrame(draw);
  };
  const kick = () => {
    if (!frame && !paused && !lost) frame = requestAnimationFrame(draw);
  };
  canvas.addEventListener("webglcontextlost", (e) => {
    e.preventDefault();
    lost = true;
  });
  return {
    ring,
    spray(spray) {
      if (off || lost) return;
      ring.write({ ...spray, n: Math.max(1, Math.round(spray.n * budget.scale)) }, clock());
      kick();
    },
    level(level) {
      off = level === "off";
      const next = level === "low" ? BUDGET.low : BUDGET.high;
      if (next !== budget) {
        budget = next;
        ring.resize(budget.pool);
        divide(budget.trail);
      }
      kick();
    },
    theme(inks, isLight) {
      ring.inks = inks.length ? inks : ring.inks;
      light = isLight;
      blend();
    },
    fit() {
      fit();
      kick();
    },
    pause(on) {
      paused = on;
      if (on && frame) {
        cancelAnimationFrame(frame);
        frame = 0;
      }
      kick();
    },
    stop() {
      if (frame) cancelAnimationFrame(frame);
      frame = 0;
      lost = true;
      gl.getExtension("WEBGL_lose_context")?.loseContext();
    },
  };
}
