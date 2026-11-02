/* TYPES */

export type Sampled = { tex: WebGLTexture | null };

export type Sampler = WebGLTexture | Sampled;

export type Uniform = number | boolean | Iterable<number> | Sampler | null;

export type Uniforms = Record<string, Uniform>;

export type Program = {
  use: () => void;
  set: (values: Uniforms) => void;
  attrib: (name: string) => number;
  drop: () => void;
};

export type Link = { ready: () => boolean; done: () => Program; drop: () => void };

export type Target = {
  fb: WebGLFramebuffer;
  tex: WebGLTexture;
  aux: WebGLTexture | null;
  w: number;
  h: number;
  size: (w: number, h: number) => Target;
  drop: () => void;
};

export type TargetOptions = { hdr?: boolean; mips?: boolean; extra?: number };

export type Format = "rgba8" | "r8" | "rg32f" | "r32f" | "rgba16f" | "rgba32f";

export type Filter = "linear" | "nearest" | "mip";

export type Wrap = "clamp" | "repeat" | "mirror";

export type TextureOptions = {
  w: number;
  h: number;
  d?: number;
  format?: Format;
  data?: ArrayBufferView | null;
  filter?: Filter;
  wrap?: Wrap;
};

export type Blend = "alpha" | "screen" | "add" | null;

export type Quads = {
  data: Float32Array;
  stride: number;
  draw: (prog: Program, n: number, mode?: Blend) => void;
  drop: () => void;
};

export type View = { w: number; h: number; t: number; dpr?: number; fixed?: boolean };

export type Tier = "phone" | "desk" | "big";

export type Governor = { scale: () => number; tick: () => number };

export type Done = () => void;

export type Blank = {
  draw: () => void;
  trigger: (onDone?: Done, onJump?: Done) => void;
  exit: (onDone?: Done) => void;
  windDown: (onDone?: Done) => void;
  hold: () => void;
  phase: () => "cruise";
  stop: () => void;
};

type Setter = (v: Uniform) => void;

type Slot = { kind: number; loc: WebGLUniformLocation | null; slot: number; tex: WebGLTexture | null | undefined };

type Constant = {
  [K in keyof WebGL2RenderingContext]: WebGL2RenderingContext[K] extends number ? K : never;
}[keyof WebGL2RenderingContext];

/* CONSTANTS */

const TRIANGLE = `#version 300 es
out vec2 v;
void main() {
  vec2 p = vec2(float((gl_VertexID << 1) & 2), float(gl_VertexID & 2));
  v = p;
  gl_Position = vec4(p * 2.0 - 1.0, 0.0, 1.0);
}
`;

export const CUBE = `#version 300 es
uniform int uFace;
out vec3 d;
void main() {
  vec2 p = vec2(float((gl_VertexID << 1) & 2), float(gl_VertexID & 2));
  vec2 q = p * 2.0 - 1.0;
  if (uFace == 0) d = vec3(1.0, -q.y, -q.x);
  else if (uFace == 1) d = vec3(-1.0, -q.y, q.x);
  else if (uFace == 2) d = vec3(q.x, 1.0, q.y);
  else if (uFace == 3) d = vec3(q.x, -1.0, -q.y);
  else if (uFace == 4) d = vec3(q.x, -q.y, 1.0);
  else d = vec3(-q.x, -q.y, -1.0);
  gl_Position = vec4(q, 0.0, 1.0);
}
`;

const PHONE = 600;
const BIG = 4.2e6;
const SLOW = 22;
const FAST = 14;
const DOWN = 1000;
const UP = 3000;
const EMA = 0.1;
const SCRATCH = 15;

const park = (gl: WebGL2RenderingContext) => gl.activeTexture(gl.TEXTURE0 + SCRATCH);

/* CONTEXT */

export function context(canvas: HTMLCanvasElement): WebGL2RenderingContext | null {
  try {
    return (
      canvas.getContext("webgl2", {
        alpha: false,
        antialias: false,
        depth: false,
        stencil: false,
        preserveDrawingBuffer: true,
      }) ?? null
    );
  } catch {
    return null;
  }
}

const floats = new WeakMap<WebGL2RenderingContext, boolean>();

export function hdr(gl: WebGL2RenderingContext): boolean {
  if (floats.has(gl)) return floats.get(gl)!;
  let ok = false;
  if (gl.getExtension("EXT_color_buffer_float") || gl.getExtension("EXT_color_buffer_half_float")) {
    const tex = gl.createTexture();
    const fb = gl.createFramebuffer();
    park(gl);
    gl.bindTexture(gl.TEXTURE_2D, tex);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA16F, 4, 4, 0, gl.RGBA, gl.HALF_FLOAT, null);
    gl.bindFramebuffer(gl.FRAMEBUFFER, fb);
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, tex, 0);
    ok = gl.checkFramebufferStatus(gl.FRAMEBUFFER) === gl.FRAMEBUFFER_COMPLETE;
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.deleteFramebuffer(fb);
    gl.deleteTexture(tex);
  }
  floats.set(gl, ok);
  return ok;
}

/* PROGRAM */

const compile = (gl: WebGL2RenderingContext, kind: number, source: string) => {
  const shader = gl.createShader(kind)!;
  gl.shaderSource(shader, source);
  gl.compileShader(shader);
  return shader;
};

const failed = (gl: WebGL2RenderingContext, shader: WebGLShader) =>
  gl.getShaderParameter(shader, gl.COMPILE_STATUS) ? null : gl.getShaderInfoLog(shader) || "the shader did not compile";

const parallel = new WeakMap<WebGL2RenderingContext, KHR_parallel_shader_compile | null>();

const setter = (
  gl: WebGL2RenderingContext,
  type: number,
  loc: WebGLUniformLocation | null,
  size: number,
): Setter | null => {
  const many = size > 1;
  switch (type) {
    case gl.FLOAT:
      return (v) => (many ? gl.uniform1fv(loc, v as Iterable<number>) : gl.uniform1f(loc, v as number));
    case gl.FLOAT_VEC2:
      return (v) => gl.uniform2fv(loc, v as Iterable<number>);
    case gl.FLOAT_VEC3:
      return (v) => gl.uniform3fv(loc, v as Iterable<number>);
    case gl.FLOAT_VEC4:
      return (v) => gl.uniform4fv(loc, v as Iterable<number>);
    case gl.INT:
      return (v) => (many ? gl.uniform1iv(loc, v as Iterable<number>) : gl.uniform1i(loc, v as number));
    case gl.BOOL:
      return (v) => gl.uniform1i(loc, v ? 1 : 0);
    case gl.UNSIGNED_INT:
      return (v) => gl.uniform1ui(loc, v as number);
    case gl.INT_VEC2:
      return (v) => gl.uniform2iv(loc, v as Iterable<number>);
    case gl.INT_VEC3:
      return (v) => gl.uniform3iv(loc, v as Iterable<number>);
    case gl.INT_VEC4:
      return (v) => gl.uniform4iv(loc, v as Iterable<number>);
    case gl.FLOAT_MAT2:
      return (v) => gl.uniformMatrix2fv(loc, false, v as Iterable<number>);
    case gl.FLOAT_MAT3:
      return (v) => gl.uniformMatrix3fv(loc, false, v as Iterable<number>);
    case gl.FLOAT_MAT4:
      return (v) => gl.uniformMatrix4fv(loc, false, v as Iterable<number>);
    default:
      return null;
  }
};

export function link(gl: WebGL2RenderingContext, frag: string, vert = TRIANGLE): Link {
  const vs = compile(gl, gl.VERTEX_SHADER, vert);
  const fs = compile(gl, gl.FRAGMENT_SHADER, frag);
  const prog = gl.createProgram();
  gl.attachShader(prog, vs);
  gl.attachShader(prog, fs);
  gl.linkProgram(prog);
  let made: Program | null = null;
  let gone = false;
  const drop = () => {
    if (gone) return;
    gone = true;
    gl.deleteProgram(prog);
    gl.deleteShader(vs);
    gl.deleteShader(fs);
  };
  const ready = () => {
    if (made || gone) return true;
    if (!parallel.has(gl)) parallel.set(gl, gl.getExtension("KHR_parallel_shader_compile"));
    const fast = parallel.get(gl);
    return !fast || Boolean(gl.getProgramParameter(prog, fast.COMPLETION_STATUS_KHR));
  };
  const done = () => {
    if (made) return made;
    if (gone) throw new Error("the program was dropped");
    const log =
      failed(gl, vs) ??
      failed(gl, fs) ??
      (gl.getProgramParameter(prog, gl.LINK_STATUS) ? null : gl.getProgramInfoLog(prog) || "the program did not link");
    if (log) {
      drop();
      throw new Error(log);
    }
    made = wrap(gl, prog, drop);
    return made;
  };
  return { ready, done, drop };
}

export function program(gl: WebGL2RenderingContext, frag: string, vert = TRIANGLE): Program {
  return link(gl, frag, vert).done();
}

function wrap(gl: WebGL2RenderingContext, prog: WebGLProgram, drop: () => void): Program {
  const table = new Map<string, Setter>();
  const attribs = new Map<string, number>();
  const samplers = new Map<number, number>([
    [gl.SAMPLER_2D, gl.TEXTURE_2D],
    [gl.SAMPLER_3D, gl.TEXTURE_3D],
    [gl.SAMPLER_CUBE, gl.TEXTURE_CUBE_MAP],
  ]);
  const slots: Slot[] = [];
  const count: number = gl.getProgramParameter(prog, gl.ACTIVE_UNIFORMS);
  for (let i = 0; i < count; i++) {
    const info = gl.getActiveUniform(prog, i);
    if (!info) continue;
    const name = info.name.replace(/\[0\]$/, "");
    const loc = gl.getUniformLocation(prog, info.name);
    const kind = samplers.get(info.type);
    if (kind) {
      const one: Slot = { kind, loc, slot: slots.length, tex: undefined };
      slots.push(one);
      table.set(name, (v) => {
        one.tex = (v as Partial<Sampled> | null)?.tex ?? v ?? null;
      });
      continue;
    }
    const set = setter(gl, info.type, loc, info.size);
    if (set) table.set(name, set);
  }
  const bind = () => {
    for (const one of slots) {
      if (one.tex === undefined) continue;
      gl.activeTexture(gl.TEXTURE0 + one.slot);
      gl.bindTexture(one.kind, one.tex);
      gl.uniform1i(one.loc, one.slot);
    }
  };
  const use = () => {
    gl.useProgram(prog);
    bind();
  };
  const set = (values: Uniforms) => {
    gl.useProgram(prog);
    for (const key in values) table.get(key)?.(values[key]);
    bind();
  };
  const attrib = (name: string) => {
    if (!attribs.has(name)) attribs.set(name, gl.getAttribLocation(prog, name));
    return attribs.get(name)!;
  };
  return { use, set, attrib, drop };
}

const empty = new WeakMap<WebGL2RenderingContext, WebGLVertexArrayObject>();

export function fill(gl: WebGL2RenderingContext) {
  if (!empty.has(gl)) empty.set(gl, gl.createVertexArray());
  gl.bindVertexArray(empty.get(gl)!);
  gl.drawArrays(gl.TRIANGLES, 0, 3);
}

/* TEXTURES */

export function target(
  gl: WebGL2RenderingContext,
  w: number,
  h: number,
  { hdr: want = false, mips = false, extra = 0 }: TargetOptions = {},
): Target {
  const high = want && hdr(gl);
  const tex = gl.createTexture();
  const aux = extra ? gl.createTexture() : null;
  const fb = gl.createFramebuffer();
  park(gl);
  gl.bindTexture(gl.TEXTURE_2D, tex);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, mips ? gl.LINEAR_MIPMAP_LINEAR : gl.LINEAR);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  if (aux) {
    gl.bindTexture(gl.TEXTURE_2D, aux);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  }
  const size = (nw: number, nh: number): Target => {
    const sw = Math.max(1, Math.round(nw));
    const sh = Math.max(1, Math.round(nh));
    if (sw === self.w && sh === self.h) return self;
    self.w = sw;
    self.h = sh;
    park(gl);
    gl.bindTexture(gl.TEXTURE_2D, tex);
    gl.texImage2D(
      gl.TEXTURE_2D,
      0,
      high ? gl.RGBA16F : gl.RGBA8,
      sw,
      sh,
      0,
      gl.RGBA,
      high ? gl.HALF_FLOAT : gl.UNSIGNED_BYTE,
      null,
    );
    if (mips) gl.generateMipmap(gl.TEXTURE_2D);
    if (aux) {
      gl.bindTexture(gl.TEXTURE_2D, aux);
      gl.texImage2D(
        gl.TEXTURE_2D,
        0,
        high ? gl.RGBA16F : gl.RGBA8,
        sw,
        sh,
        0,
        gl.RGBA,
        high ? gl.HALF_FLOAT : gl.UNSIGNED_BYTE,
        null,
      );
    }
    gl.bindFramebuffer(gl.FRAMEBUFFER, fb);
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, tex, 0);
    if (aux) {
      gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT1, gl.TEXTURE_2D, aux, 0);
      gl.drawBuffers([gl.COLOR_ATTACHMENT0, gl.COLOR_ATTACHMENT1]);
    }
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    return self;
  };
  const drop = () => {
    gl.deleteFramebuffer(fb);
    gl.deleteTexture(tex);
    if (aux) gl.deleteTexture(aux);
  };
  const self: Target = { fb, tex, aux, w: 0, h: 0, size, drop };
  return self.size(w, h);
}

const FORMATS: Record<Format, [Constant, Constant, Constant]> = {
  rgba8: ["RGBA8", "RGBA", "UNSIGNED_BYTE"],
  r8: ["R8", "RED", "UNSIGNED_BYTE"],
  rg32f: ["RG32F", "RG", "FLOAT"],
  r32f: ["R32F", "RED", "FLOAT"],
  rgba16f: ["RGBA16F", "RGBA", "FLOAT"],
  rgba32f: ["RGBA32F", "RGBA", "FLOAT"],
};

const WRAPS: Record<Wrap, Constant> = { clamp: "CLAMP_TO_EDGE", repeat: "REPEAT", mirror: "MIRRORED_REPEAT" };

export function texture(
  gl: WebGL2RenderingContext,
  { w, h, d = 0, format = "rgba8", data = null, filter = "linear", wrap = "clamp" }: TextureOptions,
): WebGLTexture {
  const [inner, form, type] = (FORMATS[format] ?? FORMATS.rgba8).map((key) => gl[key]);
  const kind = d ? gl.TEXTURE_3D : gl.TEXTURE_2D;
  const tex = gl.createTexture();
  park(gl);
  gl.bindTexture(kind, tex);
  gl.pixelStorei(gl.UNPACK_ALIGNMENT, 1);
  if (d) gl.texImage3D(kind, 0, inner, w, h, d, 0, form, type, data);
  else gl.texImage2D(kind, 0, inner, w, h, 0, form, type, data);
  const how = /32f$/.test(format) ? "nearest" : filter;
  const mips = how === "mip";
  if (mips) gl.generateMipmap(kind);
  gl.texParameteri(
    kind,
    gl.TEXTURE_MIN_FILTER,
    mips ? gl.LINEAR_MIPMAP_LINEAR : how === "nearest" ? gl.NEAREST : gl.LINEAR,
  );
  gl.texParameteri(kind, gl.TEXTURE_MAG_FILTER, how === "nearest" ? gl.NEAREST : gl.LINEAR);
  const edge = gl[WRAPS[wrap] ?? WRAPS.clamp];
  for (const key of d
    ? (["TEXTURE_WRAP_S", "TEXTURE_WRAP_T", "TEXTURE_WRAP_R"] satisfies Constant[])
    : (["TEXTURE_WRAP_S", "TEXTURE_WRAP_T"] satisfies Constant[]))
    gl.texParameteri(kind, gl[key], edge);
  return tex;
}

export function bake(
  gl: WebGL2RenderingContext,
  size: number,
  frag: string | Program,
  values: Uniforms = {},
  mips = true,
): WebGLTexture {
  const tex = gl.createTexture();
  const fb = gl.createFramebuffer();
  const own = typeof frag === "string";
  const prog = own ? program(gl, frag, CUBE) : frag;
  const port: Int32Array = gl.getParameter(gl.VIEWPORT);
  park(gl);
  gl.bindTexture(gl.TEXTURE_CUBE_MAP, tex);
  for (let face = 0; face < 6; face++)
    gl.texImage2D(gl.TEXTURE_CUBE_MAP_POSITIVE_X + face, 0, gl.RGBA8, size, size, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
  gl.texParameteri(gl.TEXTURE_CUBE_MAP, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);
  gl.texParameteri(gl.TEXTURE_CUBE_MAP, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
  gl.texParameteri(gl.TEXTURE_CUBE_MAP, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_CUBE_MAP, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
  gl.bindFramebuffer(gl.FRAMEBUFFER, fb);
  gl.viewport(0, 0, size, size);
  gl.disable(gl.BLEND);
  for (let face = 0; face < 6; face++) {
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_CUBE_MAP_POSITIVE_X + face, tex, 0);
    prog.set({ ...values, uFace: face });
    fill(gl);
  }
  gl.bindFramebuffer(gl.FRAMEBUFFER, null);
  gl.deleteFramebuffer(fb);
  if (own) prog.drop();
  gl.viewport(port[0], port[1], port[2], port[3]);
  if (mips) mipmap(gl, tex);
  return tex;
}

export function mipmap(gl: WebGL2RenderingContext, tex: WebGLTexture | null) {
  park(gl);
  gl.bindTexture(gl.TEXTURE_CUBE_MAP, tex);
  gl.generateMipmap(gl.TEXTURE_CUBE_MAP);
}

/* QUADS */

export function blend(gl: WebGL2RenderingContext, mode: Blend) {
  if (!mode) {
    gl.disable(gl.BLEND);
    return;
  }
  gl.enable(gl.BLEND);
  if (mode === "alpha") gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
  else if (mode === "screen") gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_COLOR);
  else gl.blendFunc(gl.ONE, gl.ONE);
}

export function quads(gl: WebGL2RenderingContext, layout: Record<string, number>, max: number): Quads {
  const names = Object.keys(layout);
  const stride = names.reduce((n, key) => n + layout[key], 0);
  const room = Math.max(1, max);
  const data = new Float32Array(room * stride);
  const buffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
  gl.bufferData(gl.ARRAY_BUFFER, data.byteLength, gl.DYNAMIC_DRAW);
  const vaos = new Map<Program, WebGLVertexArrayObject>();
  const vao = (prog: Program) => {
    if (vaos.has(prog)) return vaos.get(prog)!;
    const made = gl.createVertexArray();
    gl.bindVertexArray(made);
    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
    let offset = 0;
    for (const name of names) {
      const at = prog.attrib(name);
      if (at >= 0) {
        gl.enableVertexAttribArray(at);
        gl.vertexAttribPointer(at, layout[name], gl.FLOAT, false, stride * 4, offset * 4);
        gl.vertexAttribDivisor(at, 1);
      }
      offset += layout[name];
    }
    vaos.set(prog, made);
    return made;
  };
  const draw = (prog: Program, n: number, mode: Blend = "add") => {
    const count = Math.min(n, room);
    if (count <= 0) return;
    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, data, 0, count * stride);
    prog.use();
    gl.bindVertexArray(vao(prog));
    blend(gl, mode);
    gl.drawArraysInstanced(gl.TRIANGLE_STRIP, 0, 4, count);
    gl.bindVertexArray(null);
  };
  const drop = () => {
    for (const made of vaos.values()) gl.deleteVertexArray(made);
    vaos.clear();
    gl.deleteBuffer(buffer);
  };
  return { data, stride, draw, drop };
}

/* BUDGET */

export function tier(view: Pick<View, "w" | "h" | "dpr" | "fixed">): Tier {
  if (!view.fixed && Math.min(view.w, view.h) / (view.dpr || 1) < PHONE) return "phone";
  return view.w * view.h > BIG ? "big" : "desk";
}

export function governor(view: Pick<View, "t" | "fixed">, notches: readonly number[]): Governor {
  let at = 0;
  let ema = 0;
  let last: number | null = null;
  let over = 0;
  let under = 0;
  const scale = () => notches[view.fixed ? 0 : at];
  const tick = () => {
    if (view.fixed || last === null) {
      last = view.t;
      return scale();
    }
    const step = view.t - last;
    last = view.t;
    if (step <= 0 || step > 100) return scale();
    ema = ema ? ema + (step - ema) * EMA : step;
    over = ema > SLOW ? over + step : 0;
    under = ema < FAST ? under + step : 0;
    if (over >= DOWN && at < notches.length - 1) {
      at++;
      over = 0;
    }
    if (under >= UP && at > 0) {
      at--;
      under = 0;
    }
    return scale();
  };
  return { scale, tick };
}

/* BLANK */

export function blank(canvas: HTMLCanvasElement): Blank {
  const gl = canvas?.getContext?.("webgl2") ?? null;
  const pen = gl ? null : (canvas?.getContext?.("2d") ?? null);
  const fire = (...calls: (Done | undefined)[]) => {
    for (const call of calls) call?.();
  };
  const draw = () => {
    if (gl) {
      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
      gl.clearColor(0, 0, 0, 1);
      gl.clear(gl.COLOR_BUFFER_BIT);
    } else if (pen) {
      pen.fillStyle = "#000";
      pen.fillRect(0, 0, canvas.width, canvas.height);
    }
  };
  return {
    draw,
    trigger: (onDone, onJump) => fire(onJump, onDone),
    exit: (onDone) => fire(onDone),
    windDown: (onDone) => fire(onDone),
    hold: () => {},
    phase: () => "cruise",
    stop: () => gl?.getExtension("WEBGL_lose_context")?.loseContext(),
  };
}
