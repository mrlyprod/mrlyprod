struct Uniforms {
  res: vec4<f32>,
  board: vec4<f32>,
  base: vec4<f32>,
  pose: vec4<f32>,
  lens: vec4<f32>,
  light: vec4<f32>,
};
@group(0) @binding(0) var<uniform> u: Uniforms;

const NEAR: f32 = 0.25;
const FOCAL: f32 = 1.2;

fn spun(v: vec3<f32>) -> vec3<f32> {
  let c = cos(u.pose.x);
  let s = sin(u.pose.x);
  return vec3<f32>(c * v.x + s * v.z, v.y, -s * v.x + c * v.z);
}

fn view() -> mat3x3<f32> {
  let cy = cos(u.pose.y);
  let sy = sin(u.pose.y);
  let cp = cos(u.pose.z);
  let sp = sin(u.pose.z);
  return mat3x3<f32>(
    vec3<f32>(cy, -sp * sy, cp * sy),
    vec3<f32>(0.0, cp, sp),
    vec3<f32>(-sy, -sp * cy, cp * cy),
  );
}

fn project(e: vec3<f32>) -> vec4<f32> {
  let dist = u.pose.w;
  let far = dist + 4.0;
  let ortho = u.lens.z > 0.5;
  var a = 2.0 * FOCAL;
  var q = far / (far - NEAR);
  var w = dist - e.z;
  if (ortho) {
    a = 2.0 * FOCAL / dist;
    q = 1.0 / (far - NEAR);
    w = 1.0;
  }
  let x = a * (e.x + u.lens.x) * u.res.y / max(u.res.x, 1.0);
  let y = a * (e.y + u.lens.y);
  let z = q * (dist - NEAR - e.z);
  return vec4<f32>(x, y, z, w);
}

fn facing(n: vec3<f32>, e: vec3<f32>) -> f32 {
  if (u.lens.z > 0.5) {
    return n.z;
  }
  return -(n.x * e.x + n.y * e.y + n.z * (e.z - u.pose.w));
}

fn beam() -> vec3<f32> {
  let cl = cos(u.light.y);
  return vec3<f32>(sin(u.light.x) * cl, sin(u.light.y), cos(u.light.x) * cl);
}

struct Tri {
  @builtin(position) pos: vec4<f32>,
  @location(0) @interpolate(flat) color: vec3<f32>,
  @location(1) @interpolate(flat) face: f32,
};

@vertex
fn vs_main(@location(0) pos: vec3<f32>, @location(1) normal: vec3<f32>) -> Tri {
  let world = spun(pos);
  let n = spun(normal);
  let lit = (dot(n, beam()) + 1.0) * 0.5;
  let bands = u.base.w;
  let band = clamp(floor(lit * bands), 0.0, bands - 1.0);
  let t = (64.0 + floor(191.0 * band / (bands - 1.0))) / 255.0;
  let e = view() * world;
  var out: Tri;
  out.pos = project(e);
  out.color = u.base.rgb * t;
  out.face = facing(view() * n, e);
  return out;
}

@fragment
fn fs_main(in: Tri) -> @location(0) vec4<f32> {
  return vec4<f32>(in.color, 1.0);
}

@fragment
fn fs_back(in: Tri) -> @location(0) vec4<f32> {
  if (in.face > 0.0) {
    discard;
  }
  return vec4<f32>(in.color * u.lens.w, u.lens.w);
}

@fragment
fn fs_front(in: Tri) -> @location(0) vec4<f32> {
  if (in.face <= 0.0) {
    discard;
  }
  return vec4<f32>(in.color * u.lens.w, u.lens.w);
}

struct Line {
  @builtin(position) pos: vec4<f32>,
  @location(0) color: vec4<f32>,
};

@vertex
fn vs_line(@location(0) pos: vec3<f32>, @location(1) flag: f32, @location(2) color: vec4<f32>) -> Line {
  var world = pos;
  if (flag > 0.5) {
    world = spun(pos);
  }
  var out: Line;
  out.pos = project(view() * world);
  out.pos.z = out.pos.z - 0.005 * out.pos.w;
  out.color = color;
  return out;
}

@fragment
fn fs_line(in: Line) -> @location(0) vec4<f32> {
  return vec4<f32>(in.color.rgb * in.color.a, in.color.a);
}
