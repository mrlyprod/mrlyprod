import { reached, sourced, tap } from './feed.js';
import { COPY, DOWN, LENS, QUAD } from './shaders.js';

/* LOOKS */

const LEVELS = 7;
const PACE = 50;
const LIGHT = [-0.5, -0.866];

const STYLES = {
  clear: { lod: 0.6, mix: 0.3, tint: 0.06, sat: 1.25, spec: 0.55, chroma: 0.3 },
  tinted: { lod: 3.2, mix: 1, tint: 0.3, sat: 1.5, spec: 0.4, chroma: 0 },
  frosted: { lod: 4.6, mix: 1, tint: 0.5, sat: 1.25, spec: 0.3, chroma: 0 },
};

const KINDS = {
  bar: { bezel: 14, depth: 14, thick: 10, lod: 0, tint: 0.25, sat: 1, mix: 0 },
  window: { bezel: 18, depth: 18, thick: 12, lod: 1.4, tint: 0.45, sat: 0.75, mix: 0.85 },
  dock: { bezel: 16, depth: 16, thick: 12, lod: 0.2, tint: 0.2, sat: 1.1, mix: 0 },
  widget: { bezel: 16, depth: 14, thick: 10, lod: 0.8, tint: 0.1, sat: 1.1, mix: 0.6 },
  launchpad: { bezel: 22, depth: 22, thick: 14, lod: 2.2, tint: 0.5, sat: 0.8, mix: 1 },
  menu: { bezel: 12, depth: 12, thick: 8, lod: 1, tint: 0.45, sat: 0.9, mix: 0.8 },
  panel: { bezel: 16, depth: 16, thick: 12, lod: -1, tint: 0.5, sat: 0.85, mix: 0.9 },
  pill: { bezel: 14, depth: 14, thick: 10, lod: -1, tint: 0.3, sat: 1, mix: 0.4 },
};

const style = () => document.documentElement.dataset.glass || 'tinted';

/* COLOR */

let probe = null;

function rgb(css) {
  probe ??= document.createElement('canvas').getContext('2d', { willReadFrequently: true });
  probe.clearRect(0, 0, 1, 1);
  probe.fillStyle = '#000';
  probe.fillStyle = css;
  probe.fillRect(0, 0, 1, 1);
  const [r, g, b] = probe.getImageData(0, 0, 1, 1).data;
  return [r / 255, g / 255, b / 255];
}

function tones() {
  const css = getComputedStyle(document.documentElement);
  const ground = rgb(css.getPropertyValue('--bg').trim() || '#000');
  const accent = rgb(css.getPropertyValue('--accent').trim() || '#000');
  return { ground, tint: ground.map((one, i) => one * 0.88 + accent[i] * 0.12) };
}

/* GL */

function compile(gl, kind, text) {
  const shader = gl.createShader(kind);
  gl.shaderSource(shader, text);
  gl.compileShader(shader);
  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) throw new Error(`glass: ${gl.getShaderInfoLog(shader)}`);
  return shader;
}

function program(gl, frag) {
  const one = gl.createProgram();
  gl.attachShader(one, compile(gl, gl.VERTEX_SHADER, QUAD));
  gl.attachShader(one, compile(gl, gl.FRAGMENT_SHADER, frag));
  gl.bindAttribLocation(one, 0, 'a');
  gl.linkProgram(one);
  if (!gl.getProgramParameter(one, gl.LINK_STATUS)) throw new Error(`glass: ${gl.getProgramInfoLog(one)}`);
  const spots = new Map();
  const at = (name) => {
    if (!spots.has(name)) spots.set(name, gl.getUniformLocation(one, name));
    return spots.get(name);
  };
  return { one, at };
}

function texture(gl, w, h) {
  const tex = gl.createTexture();
  gl.bindTexture(gl.TEXTURE_2D, tex);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.MIRRORED_REPEAT);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.MIRRORED_REPEAT);
  if (w && h) gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA8, w, h, 0, gl.RGBA, gl.UNSIGNED_BYTE, null);
  return tex;
}

function target(gl, w, h) {
  const tex = texture(gl, w, h);
  const fbo = gl.createFramebuffer();
  gl.bindFramebuffer(gl.FRAMEBUFFER, fbo);
  gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, tex, 0);
  gl.bindFramebuffer(gl.FRAMEBUFFER, null);
  return { tex, fbo, w, h };
}

function drop(gl, one) {
  gl.deleteTexture(one.tex);
  gl.deleteFramebuffer(one.fbo);
}

/* ENGINE */

function engine(canvas) {
  const gl = canvas.getContext('webgl2', { alpha: true, premultipliedAlpha: true, antialias: false, depth: false, stencil: false, preserveDrawingBuffer: false });
  if (!gl) return null;
  const quad = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, quad);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([0, 0, 1, 0, 0, 1, 1, 1]), gl.STATIC_DRAW);
  const vao = gl.createVertexArray();
  gl.bindVertexArray(vao);
  gl.enableVertexAttribArray(0);
  gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0);
  const copy = program(gl, COPY);
  const down = program(gl, DOWN);
  const lens = program(gl, LENS);
  const uploads = new Map();
  let scene = null;
  let levels = [];
  const box = (prog, x0, y0, x1, y1) => gl.uniform4f(prog.at('uBox'), x0, y0, x1, y1);
  const draw = () => gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
  const bind = (unit, tex) => {
    gl.activeTexture(gl.TEXTURE0 + unit);
    gl.bindTexture(gl.TEXTURE_2D, tex);
  };
  const size = (w, h) => {
    if (scene?.w === w && scene?.h === h) return;
    if (scene) drop(gl, scene);
    for (const one of levels) drop(gl, one);
    scene = target(gl, w, h);
    levels = [];
    let lw = w;
    let lh = h;
    for (let k = 0; k < LEVELS; k++) {
      lw = Math.max(1, Math.ceil(lw / 2));
      lh = Math.max(1, Math.ceil(lh / 2));
      levels.push(target(gl, lw, lh));
    }
  };
  const upload = (canvas) => {
    let tex = uploads.get(canvas);
    if (!tex) {
      tex = texture(gl, 0, 0);
      uploads.set(canvas, tex);
    }
    gl.bindTexture(gl.TEXTURE_2D, tex);
    gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, true);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, canvas);
    gl.pixelStorei(gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL, false);
  };
  const compose = (view, scale, list, ground) => {
    size(Math.max(1, Math.round(view.w * scale)), Math.max(1, Math.round(view.h * scale)));
    gl.bindFramebuffer(gl.FRAMEBUFFER, scene.fbo);
    gl.viewport(0, 0, scene.w, scene.h);
    gl.disable(gl.BLEND);
    gl.clearColor(ground[0], ground[1], ground[2], 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);
    gl.useProgram(copy.one);
    gl.uniform1i(copy.at('uTex'), 0);
    for (const { canvas, rect, reach } of list) {
      const tex = uploads.get(canvas);
      if (!tex || !rect.width || !rect.height) continue;
      const out = reach ?? rect;
      if (reach) {
        gl.enable(gl.SCISSOR_TEST);
        gl.scissor(Math.floor(out.left * scale), Math.floor(out.top * scale), Math.ceil(out.width * scale), Math.ceil(out.height * scale));
        gl.clear(gl.COLOR_BUFFER_BIT);
        gl.disable(gl.SCISSOR_TEST);
      }
      bind(0, tex);
      gl.uniform4f(copy.at('uUv'), (out.left - rect.left) / rect.width, (out.top - rect.top) / rect.height, out.width / rect.width, out.height / rect.height);
      box(copy, (out.left / view.w) * 2 - 1, (out.top / view.h) * 2 - 1, (out.right / view.w) * 2 - 1, (out.bottom / view.h) * 2 - 1);
      draw();
    }
    gl.disable(gl.BLEND);
    gl.useProgram(down.one);
    gl.uniform1i(down.at('uTex'), 0);
    box(down, -1, -1, 1, 1);
    let from = scene;
    for (const one of levels) {
      gl.bindFramebuffer(gl.FRAMEBUFFER, one.fbo);
      gl.viewport(0, 0, one.w, one.h);
      bind(0, from.tex);
      gl.uniform2f(down.at('uStep'), 1 / from.w, 1 / from.h);
      draw();
      from = one;
    }
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
  };
  const paint = (piece, view, look, out) => {
    const { rect, dpr, scale, radii, kind } = piece;
    const w = Math.max(1, Math.round(rect.width * dpr));
    const h = Math.max(1, Math.round(rect.height * dpr));
    if (canvas.width < w || canvas.height < h) {
      canvas.width = Math.max(canvas.width, w);
      canvas.height = Math.max(canvas.height, h);
    }
    const lod = Math.min(LEVELS, Math.max(0, look.lod + kind.lod + Math.log2(scale)));
    const lo = Math.max(1, Math.floor(lod));
    const hi = Math.min(LEVELS, lo + 1);
    gl.bindFramebuffer(gl.FRAMEBUFFER, null);
    gl.viewport(0, 0, w, h);
    gl.disable(gl.BLEND);
    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.useProgram(lens.one);
    box(lens, -1, -1, 1, 1);
    bind(0, scene.tex);
    bind(1, levels[lo - 1].tex);
    bind(2, levels[hi - 1].tex);
    const u = (name) => lens.at(name);
    gl.uniform1i(u('uScene'), 0);
    gl.uniform1i(u('uLo'), 1);
    gl.uniform1i(u('uHi'), 2);
    gl.uniform2f(u('uView'), view.w, view.h);
    gl.uniform4f(u('uRect'), rect.left, rect.top, rect.width, rect.height);
    gl.uniform1f(u('uDpr'), dpr);
    gl.uniform4f(u('uRadii'), ...radii);
    gl.uniform1f(u('uBezel'), kind.bezel);
    gl.uniform1f(u('uDepth'), kind.depth);
    gl.uniform1f(u('uThick'), kind.thick);
    gl.uniform1f(u('uRefract'), 1);
    const mix = Math.max(look.mix, kind.mix);
    gl.uniform1f(u('uMix'), lod < 1 ? mix * lod : mix);
    gl.uniform1f(u('uLerp'), lod < 1 ? 0 : Math.min(1, lod - lo));
    gl.uniform1f(u('uSat'), look.sat * kind.sat);
    gl.uniform1f(u('uSpec'), look.spec);
    gl.uniform1f(u('uChroma'), look.chroma);
    gl.uniform1f(u('uRim'), 1.5);
    gl.uniform4f(u('uTint'), ...look.color, Math.min(0.92, look.tint + kind.tint));
    gl.uniform2f(u('uLight'), ...LIGHT);
    draw();
    if (out.width !== w) out.width = w;
    if (out.height !== h) out.height = h;
    const ctx = out.getContext('2d');
    ctx.clearRect(0, 0, w, h);
    ctx.drawImage(canvas, 0, canvas.height - h, w, h, 0, 0, w, h);
  };
  const forget = (gone) => {
    const tex = uploads.get(gone);
    if (!tex) return;
    gl.deleteTexture(tex);
    uploads.delete(gone);
  };
  return { gl, canvas, upload, compose, paint, forget };
}

/* PIECES */

const radii = (el) => {
  const css = getComputedStyle(el);
  const px = (one) => Number.parseFloat(one) || 0;
  return [px(css.borderBottomRightRadius), px(css.borderTopRightRadius), px(css.borderBottomLeftRadius), px(css.borderTopLeftRadius)];
};

function lensOf(el) {
  let out = el.querySelector(':scope > canvas.lens');
  if (!out) {
    out = document.createElement('canvas');
    out.className = 'lens';
    out.setAttribute('aria-hidden', 'true');
    el.prepend(out);
  }
  return out;
}

/* START */

let running = null;

export function start() {
  if (running) return running;
  const html = document.documentElement;
  let core = null;
  let frame = 0;
  let fresh = new Set(sourced());
  let tone = null;
  let composed = '';
  const pieces = new Set();
  const holder = document.createElement('canvas');
  const veiled = () => 'saver' in html.dataset || (html.dataset.welcome === 'open' && scrollY < innerHeight * 0.8);
  const sizes = new ResizeObserver(() => wake());
  const unlit = () => {
    for (const el of pieces) {
      el.removeAttribute('data-lit');
      el.querySelector(':scope > canvas.lens')?.remove();
      sizes.unobserve(el);
    }
    pieces.clear();
  };
  const scan = () => {
    const now = new Set(document.querySelectorAll('[data-lens]'));
    for (const el of pieces) {
      if (now.has(el)) continue;
      el.removeAttribute('data-lit');
      el.querySelector(':scope > canvas.lens')?.remove();
      sizes.unobserve(el);
      pieces.delete(el);
    }
    for (const el of now) {
      if (pieces.has(el)) continue;
      pieces.add(el);
      sizes.observe(el);
    }
  };
  const render = () => {
    frame = 0;
    if (performance.now() < until) wake();
    const look = STYLES[style()];
    if (!look || !core || document.hidden || veiled()) return;
    scan();
    const view = { w: innerWidth, h: innerHeight };
    const dpr = Math.min(globalThis.devicePixelRatio || 1, 2);
    tone ??= tones();
    const { ground, tint } = tone;
    const changed = fresh.size > 0;
    if (changed) fedAt = performance.now();
    for (const one of fresh) {
      if (sourced().includes(one)) core.upload(one);
      else core.forget(one);
    }
    fresh.clear();
    const list = sourced().map((canvas) => ({ canvas, rect: canvas.getBoundingClientRect(), reach: reached(canvas) }));
    const key = [view.w, view.h, dpr, ground, ...list.map(({ rect, reach }) => [rect.left, rect.top, rect.width, rect.height, reach?.left, reach?.top, reach?.width, reach?.height])].join();
    if (changed || key !== composed) core.compose(view, dpr, list, ground);
    composed = key;
    for (const el of pieces) {
      const rect = el.getBoundingClientRect();
      const kind = KINDS[el.dataset.lens] ?? KINDS.bar;
      if (!rect.width || !rect.height) {
        el.removeAttribute('data-lit');
        continue;
      }
      core.paint({ rect, dpr, scale: dpr, radii: radii(el), kind }, view, { ...look, color: tint }, lensOf(el));
      el.setAttribute('data-lit', '');
    }
  };
  let until = 0;
  let fedAt = 0;
  let later = 0;
  const wake = () => {
    if (!frame) frame = requestAnimationFrame(render);
  };
  const follow = (e) => {
    until = Math.max(until, performance.now() + (Number(e?.detail) || 0));
    wake();
  };
  const boot = () => {
    if (core || !STYLES[style()]) return;
    try {
      core = engine(holder);
    } catch (error) {
      console.error(error);
      core = null;
    }
    if (!core) return;
    composed = '';
    fresh = new Set(sourced());
    wake();
  };
  const restyle = () => {
    tone = null;
    if (!STYLES[style()]) {
      cancelAnimationFrame(frame);
      frame = 0;
      unlit();
      return;
    }
    if (!core) return boot();
    wake();
  };
  const lost = (e) => {
    e.preventDefault();
    core = null;
    unlit();
  };
  holder.addEventListener('webglcontextlost', lost);
  holder.addEventListener('webglcontextrestored', boot);
  const untap = tap((canvas) => {
    fresh.add(canvas);
    const wait = fedAt + PACE - performance.now();
    if (wait <= 0) return wake();
    later ||= setTimeout(() => {
      later = 0;
      wake();
    }, wait);
  });
  const watch = new MutationObserver(wake);
  watch.observe(document.body, { subtree: true, attributes: true, attributeFilter: ['data-lens'] });
  const kids = new MutationObserver(wake);
  kids.observe(document.body, { childList: true });
  const mood = new MutationObserver(restyle);
  mood.observe(html, { attributes: true, attributeFilter: ['data-glass', 'data-theme', 'data-tint', 'data-window', 'data-welcome', 'data-saver'] });
  const shade = matchMedia('(prefers-color-scheme: dark)');
  const kick = () => {
    tone = null;
    wake();
  };
  shade.addEventListener('change', kick);
  addEventListener('resize', kick);
  addEventListener('wire', kick);
  addEventListener('shift', follow);
  addEventListener('scroll', wake, { passive: true });
  document.addEventListener('visibilitychange', kick);
  boot();
  running = () => {
    untap();
    watch.disconnect();
    kids.disconnect();
    mood.disconnect();
    sizes.disconnect();
    shade.removeEventListener('change', kick);
    removeEventListener('resize', kick);
    removeEventListener('wire', kick);
    removeEventListener('shift', follow);
    removeEventListener('scroll', wake);
    holder.removeEventListener('webglcontextlost', lost);
    holder.removeEventListener('webglcontextrestored', boot);
    document.removeEventListener('visibilitychange', kick);
    cancelAnimationFrame(frame);
    clearTimeout(later);
    unlit();
    core?.gl.getExtension('WEBGL_lose_context')?.loseContext();
    core = null;
    running = null;
  };
  return running;
}
