const VERT = `#version 300 es
in vec2 a_position;
void main() { gl_Position = vec4(a_position, 0.0, 1.0); }
`;

const FRAG = `#version 300 es
precision highp float;
uniform vec2 u_resolution;
uniform vec4 u_viewport;
uniform vec2 u_c;
uniform bool u_julia;
uniform int u_maxIter;
uniform vec3 u_primary;
uniform vec3 u_accent;
uniform float u_rotation;
uniform float u_time;
out vec4 fragColor;
void main() {
  vec2 uv = gl_FragCoord.xy / u_resolution;
  float px = mix(u_viewport.x, u_viewport.y, uv.x);
  float py = mix(u_viewport.w, u_viewport.z, uv.y);
  float mr = (u_viewport.x + u_viewport.y) * 0.5;
  float mi = (u_viewport.z + u_viewport.w) * 0.5;
  float dr = px - mr, di = py - mi;
  float ca = cos(u_rotation), sa = sin(u_rotation);
  px = dr * ca - di * sa + mr;
  py = dr * sa + di * ca + mi;
  float zr = u_julia ? px : 0.0;
  float zi = u_julia ? py : 0.0;
  float cr = u_julia ? u_c.x : px;
  float ci = u_julia ? u_c.y : py;
  int iter = 0;
  for (int i = 0; i < 1000; i++) {
    if (i >= u_maxIter) break;
    if (zr * zr + zi * zi > 128.0) break;
    float tmp = zr * zr - zi * zi + cr;
    zi = 2.0 * zr * zi + ci;
    zr = tmp;
    iter++;
  }
  if (iter >= u_maxIter) {
    fragColor = vec4(u_primary, 1.0);
  } else {
    float sl = float(iter) - log2(log2(zr * zr + zi * zi)) + 4.0;
    float t = 0.5 + 0.5 * cos(3.0 + sl * 0.15 + u_time);
    fragColor = vec4(mix(u_primary, u_accent, t), 1.0);
  }
}
`;

export const UNIFORMS = ['resolution', 'viewport', 'c', 'julia', 'maxIter', 'primary', 'accent', 'rotation', 'time'];

export function build(gl) {
  const make = (kind, source) => {
    const shader = gl.createShader(kind);
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    if (gl.getShaderParameter(shader, gl.COMPILE_STATUS)) return shader;
    gl.deleteShader(shader);
    return null;
  };
  const vs = make(gl.VERTEX_SHADER, VERT);
  const fs = make(gl.FRAGMENT_SHADER, FRAG);
  if (!vs || !fs) {
    gl.deleteShader(vs);
    gl.deleteShader(fs);
    return null;
  }
  const prog = gl.createProgram();
  gl.attachShader(prog, vs);
  gl.attachShader(prog, fs);
  gl.linkProgram(prog);
  if (!gl.getProgramParameter(prog, gl.LINK_STATUS)) {
    gl.deleteProgram(prog);
    gl.deleteShader(vs);
    gl.deleteShader(fs);
    return null;
  }
  gl.useProgram(prog);
  const vao = gl.createVertexArray();
  gl.bindVertexArray(vao);
  const buffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, -1, 1, 1, -1, 1, 1]), gl.STATIC_DRAW);
  const slot = gl.getAttribLocation(prog, 'a_position');
  gl.enableVertexAttribArray(slot);
  gl.vertexAttribPointer(slot, 2, gl.FLOAT, false, 0, 0);
  const loc = {};
  for (const name of UNIFORMS) loc[name] = gl.getUniformLocation(prog, `u_${name}`);
  return { prog, vs, fs, vao, buffer, loc };
}

export function drop(gl, kit) {
  gl.deleteBuffer(kit.buffer);
  gl.deleteVertexArray(kit.vao);
  gl.deleteProgram(kit.prog);
  gl.deleteShader(kit.fs);
  gl.deleteShader(kit.vs);
}
