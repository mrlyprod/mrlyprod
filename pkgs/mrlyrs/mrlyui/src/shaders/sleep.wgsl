struct Uniforms {
  resolution: vec2<f32>,
  time: f32,
  primary: vec3<f32>,
  accent: vec3<f32>,
  rotation: f32,
  grid: vec2<f32>,
  spot: vec2<f32>,
  span: f32,
};
@group(0) @binding(0) var<uniform> u: Uniforms;

@fragment
fn fs_main(@builtin(position) frag: vec4<f32>) -> @location(0) vec4<f32> {
  let scale = min(u.resolution.x / u.grid.x, u.resolution.y / u.grid.y);
  let origin = 0.5 * (u.resolution - u.grid * scale);
  let p = (frag.xy - origin) / scale;
  let d = p - u.spot;
  let inside = all(p >= vec2<f32>(0.0, 0.0)) && all(p < u.grid);
  let hit = all(d >= vec2<f32>(0.0, 0.0)) && all(d < vec2<f32>(u.span, u.span));
  var color = u.primary;
  if (inside && hit) {
    color = u.accent;
  }
  return vec4<f32>(color, 1.0);
}
