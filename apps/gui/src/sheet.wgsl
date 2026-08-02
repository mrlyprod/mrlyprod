struct Rect {
    origin: vec2<f32>,
    span: vec2<f32>,
    top: vec4<f32>,
    foot: vec4<f32>,
};

@group(0) @binding(0) var sheet: texture_2d<f32>;
@group(0) @binding(1) var flat: sampler;
@group(0) @binding(2) var<uniform> rect: Rect;

struct Out {
    @builtin(position) at: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) paper: f32,
};

@vertex
fn vs_main(@builtin(vertex_index) i: u32, @builtin(instance_index) n: u32) -> Out {
    var corners = array<vec2<f32>, 4>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
    );
    let uv = corners[i];
    var unit = uv;
    if (n == 1u) {
        unit = rect.origin + uv * rect.span;
    }
    var out: Out;
    out.at = vec4<f32>(unit.x * 2.0 - 1.0, 1.0 - unit.y * 2.0, 0.0, 1.0);
    out.uv = uv;
    out.paper = f32(n);
    return out;
}

fn lift(c: vec3<f32>) -> vec3<f32> {
    let low = c / 12.92;
    let high = pow((c + vec3<f32>(0.055)) / 1.055, vec3<f32>(2.4));
    return select(high, low, c <= vec3<f32>(0.04045));
}

@fragment
fn fs_main(in: Out) -> @location(0) vec4<f32> {
    if (in.paper < 0.5) {
        let wash = mix(rect.top, rect.foot, in.uv.y);
        return vec4<f32>(lift(wash.rgb), 1.0);
    }
    return textureSample(sheet, flat, in.uv);
}
