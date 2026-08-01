const GLOW: f32 = 0.6;

struct Uniforms {
  resolution: vec2<f32>,
  time: f32,
  primary: vec3<f32>,
  accent: vec3<f32>,
  rotation: f32,
  grid: vec2<f32>,
};
@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var field: texture_2d<f32>;
@group(0) @binding(2) var field_sampler: sampler;

fn srgb(c: vec3<f32>) -> vec3<f32> {
  let lo = c * 12.92;
  let hi = 1.055 * pow(max(c, vec3<f32>(0.0)), vec3<f32>(1.0 / 2.4)) - 0.055;
  return clamp(select(hi, lo, c <= vec3<f32>(0.0031308)), vec3<f32>(0.0), vec3<f32>(1.0));
}

@fragment
fn fs_main(@builtin(position) frag: vec4<f32>) -> @location(0) vec4<f32> {
  let uv = frag.xy / u.resolution;
  let texel = 1.0 / max(u.grid, vec2<f32>(1.0));
  let base = textureSample(field, field_sampler, uv).rgb;
  var halo = textureSample(field, field_sampler, uv + vec2<f32>(texel.x, 0.0)).rgb;
  halo += textureSample(field, field_sampler, uv - vec2<f32>(texel.x, 0.0)).rgb;
  halo += textureSample(field, field_sampler, uv + vec2<f32>(0.0, texel.y)).rgb;
  halo += textureSample(field, field_sampler, uv - vec2<f32>(0.0, texel.y)).rgb;
  let energy = max(halo * 0.25 - u.primary, vec3<f32>(0.0));
  return vec4<f32>(srgb(base + energy * GLOW), 1.0);
}
