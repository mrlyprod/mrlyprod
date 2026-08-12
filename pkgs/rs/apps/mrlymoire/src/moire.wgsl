struct Uniforms {
  resolution: vec2<f32>,
  time: f32,
  primary: vec3<f32>,
  accent: vec3<f32>,
  rotation: f32,
  table_a: vec4<f32>,
  table_b: vec4<f32>,
  numbers: vec2<f32>,
  lattice: f32,
};
@group(0) @binding(0) var<uniform> u: Uniforms;

fn parity(v: f32) -> f32 {
  let n = floor(v);
  return n - 2.0 * floor(n * 0.5);
}

fn hit(table: vec4<f32>, n: f32, a: f32, b: f32) -> f32 {
  let ra = parity(n * a);
  let rb = parity(n * b);
  return mix(mix(table.x, table.y, rb), mix(table.z, table.w, rb), ra);
}

@fragment
fn fs_main(@builtin(position) frag: vec4<f32>) -> @location(0) vec4<f32> {
  let side = min(u.resolution.x, u.resolution.y);
  let uv = (frag.xy - 0.5 * (u.resolution - vec2<f32>(side, side))) / side;
  var a = uv.x;
  var b = uv.y;
  if (u.lattice > 0.5) {
    let s3 = sqrt(3.0);
    a = uv.x - uv.y / s3;
    b = 2.0 * uv.y / s3;
  }
  let value = hit(u.table_a, u.numbers.x, a, b) + hit(u.table_b, u.numbers.y, a, b);
  return vec4<f32>(mix(u.primary, u.accent, 0.5 * value), 1.0);
}
