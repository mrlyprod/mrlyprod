struct Uniforms {
  resolution: vec2<f32>,
  time: f32,
  primary: vec3<f32>,
  accent: vec3<f32>,
  rotation: f32,
  viewport: vec4<f32>,
  c: vec2<f32>,
  max_iter: f32,
  band: f32,
  fade: f32,
};
@group(0) @binding(0) var<uniform> u: Uniforms;

fn paint(level: f32) -> vec3<f32> {
  let p = fract((level + u.time) / max(u.band, 1.0));
  let t = 1.0 - abs(2.0 * p - 1.0);
  return mix(u.primary, u.accent, t);
}

@fragment
fn fs_main(@builtin(position) frag: vec4<f32>) -> @location(0) vec4<f32> {
  let uv = frag.xy / u.resolution;
  var zr = mix(u.viewport.x, u.viewport.y, uv.x);
  var zi = mix(u.viewport.w, u.viewport.z, uv.y);
  let mr = (u.viewport.x + u.viewport.y) * 0.5;
  let mi = (u.viewport.z + u.viewport.w) * 0.5;
  let dr = zr - mr;
  let di = zi - mi;
  let ca = cos(u.rotation);
  let sa = sin(u.rotation);
  zr = dr * ca - di * sa + mr;
  zi = dr * sa + di * ca + mi;
  var iter = 0.0;
  for (var i = 0; i < 1000; i = i + 1) {
    if (f32(i) >= u.max_iter || zr * zr + zi * zi > 128.0) { break; }
    let tmp = zr * zr - zi * zi + u.c.x;
    zi = 2.0 * zr * zi + u.c.y;
    zr = tmp;
    iter = iter + 1.0;
  }
  var color = u.primary;
  if (iter < u.max_iter) {
    let level = iter - log2(log2(max(zr * zr + zi * zi, 2.0))) + 4.0;
    color = paint(level);
  }
  return vec4<f32>(mix(u.primary, color, clamp(u.fade, 0.0, 1.0)), 1.0);
}
