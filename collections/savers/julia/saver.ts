import type { Look, Play } from "../play.ts";
import { inks, play, rng } from "../play.ts";

const MOST = 6e6;

const SHORT = 3.1;

const LONG = 4.5;

const LAP = 300;

const TIDE = LAP * 2.618;

const EVEN = 0.2;

const BEND = 14;

const RIM = 0.95;

const SPAN = 0.005;

const SWAY = 0.01;

const FALL = { dark: 2, light: 3 };

const VERT = `#version 300 es
void main() {
  vec2 p = vec2(float((gl_VertexID << 1) & 2), float(gl_VertexID & 2));
  gl_Position = vec4(p * 2.0 - 1.0, 0.0, 1.0);
}`;

const FRAG = `#version 300 es
precision highp float;
uniform vec2 uC;
uniform vec2 uMid;
uniform vec2 uStep;
uniform float uFall;
uniform vec3 uPaper;
uniform vec3 uTint;
out vec4 o;
const int STEPS = 64;
const float FAR = 1.0e4;
const float STEEP = 1.0e30;
const float WIDE = 0.012;
const float FINE = 1.5;
const float INNER = 0.4;
void main() {
  vec2 q = gl_FragCoord.xy - uMid;
  vec2 z = vec2(q.x * uStep.x - q.y * uStep.y, q.x * uStep.y + q.y * uStep.x);
  vec2 dz = vec2(1.0, 0.0);
  float r = 0.0;
  float g = 1.0;
  float top = 1.0;
  for (int i = 0; i < STEPS; i++) {
    dz = 2.0 * vec2(z.x * dz.x - z.y * dz.y, z.x * dz.y + z.y * dz.x);
    z = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + uC;
    r = dot(z, z);
    g = dot(dz, dz);
    top = max(top, g);
    if (r > FAR || g > STEEP) break;
  }
  float d = r > FAR ? 0.5 * sqrt(r / g) * log(r) : INNER * inversesqrt(top);
  float k = pow(1.0 + d / max(WIDE, FINE * length(uStep)), -uFall);
  float n = fract(52.9829189 * fract(dot(gl_FragCoord.xy, vec2(0.06711056, 0.00583715))));
  o = vec4(mix(uPaper, uTint, k) + (n - 0.5) / 255.0, 1.0);
}`;

function pace(turn: number) {
  const half = Math.PI * turn;
  return 2 * (1 - EVEN) * half + 2 * EVEN * Math.atan2(BEND * Math.sin(half), Math.cos(half));
}

function keep(event: Event) {
  event.preventDefault();
}

export default function julia(canvas: HTMLCanvasElement, look: Look): Play {
  const gl = canvas.getContext("webgl2", {
    alpha: false,
    antialias: false,
    depth: false,
    stencil: false,
    preserveDrawingBuffer: true,
  });
  const { rgb, css } = inks(look);
  if (!gl || gl.isContextLost()) {
    const ctx = canvas.getContext("2d");
    return play(canvas, () => {
      if (!ctx) return;
      ctx.fillStyle = css(0);
      ctx.fillRect(0, 0, canvas.width, canvas.height);
    });
  }
  const rand = rng(look.seed);
  const phase = rand();
  const hand = rand() < 0.5 ? -1 : 1;
  const base = RIM + SPAN * (2 * rand() - 1);
  const drift = rand();
  const build = () => {
    gl.deleteProgram(gl.getParameter(gl.CURRENT_PROGRAM));
    const prog = gl.createProgram()!;
    for (const [type, src] of [
      [gl.VERTEX_SHADER, VERT],
      [gl.FRAGMENT_SHADER, FRAG],
    ] as const) {
      const part = gl.createShader(type)!;
      gl.shaderSource(part, src);
      gl.compileShader(part);
      gl.attachShader(prog, part);
      gl.deleteShader(part);
    }
    gl.linkProgram(prog);
    gl.useProgram(prog);
    const at = (name: string) => gl.getUniformLocation(prog, name);
    gl.uniform3fv(at("uPaper"), rgb(0));
    gl.uniform3fv(at("uTint"), rgb(1));
    gl.uniform1f(at("uFall"), FALL[look.theme]);
    return { prog, c: at("uC"), mid: at("uMid"), step: at("uStep") };
  };
  let kit = build();
  canvas.addEventListener("webglcontextlost", keep);
  return play(
    canvas,
    (t) => {
      if (gl.isContextLost()) return;
      if (!gl.isProgram(kit.prog)) kit = build();
      const { width: w, height: h } = canvas;
      const a = pace((t / LAP + phase) % 1);
      const rim = base + SWAY * Math.sin(2 * Math.PI * ((t / TIDE + drift) % 1));
      const [x, y] = [rim * Math.cos(a), rim * Math.sin(a)];
      const s = Math.max(SHORT / Math.min(w, h), LONG / Math.max(w, h));
      gl.viewport(0, 0, w, h);
      gl.uniform2f(kit.c, x / 2 - (x * x - y * y) / 4, hand * (y / 2 - (x * y) / 2));
      gl.uniform2f(kit.mid, w / 2, h / 2);
      gl.uniform2f(kit.step, h > w ? 0 : s, h > w ? s : 0);
      gl.drawArrays(gl.TRIANGLES, 0, 3);
    },
    MOST,
  );
}
