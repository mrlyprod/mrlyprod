struct Rect {
    origin: vec2<f32>,
    span: vec2<f32>,
};

@group(0) @binding(0) var sheet: texture_2d<f32>;
@group(0) @binding(1) var flat: sampler;
@group(0) @binding(2) var<uniform> rect: Rect;

struct Out {
    @builtin(position) at: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> Out {
    var corners = array<vec2<f32>, 4>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
    );
    let uv = corners[i];
    let unit = rect.origin + uv * rect.span;
    var out: Out;
    out.at = vec4<f32>(unit.x * 2.0 - 1.0, 1.0 - unit.y * 2.0, 0.0, 1.0);
    out.uv = uv;
    return out;
}

@fragment
fn fs_main(in: Out) -> @location(0) vec4<f32> {
    return textureSample(sheet, flat, in.uv);
}
