struct VertexIn {
    @location(0) pos: vec3<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(
    in: VertexIn,
) -> VertexOut {

    var out: VertexOut;
    out.color = in.color;
    out.pos = vec4<f32>(in.pos, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return in.color;
}