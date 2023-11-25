struct VertexIn {
    @location(0) pos: vec3<f32>,
}

struct InstanceIn {
    @location(1) color: vec4<f32>,
    @location(2) model_0: vec4<f32>,
    @location(3) model_1: vec4<f32>,
    @location(4) model_2: vec4<f32>,
    @location(5) model_3: vec4<f32>,
}

struct VertexOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
}

struct Camera {
    vp: mat4x4<f32>,
}
@group(0) @binding(0)
var<uniform> camera: Camera;

@vertex
fn vs_main(
    in: VertexIn,
    instance: InstanceIn,
) -> VertexOut {

    let model = mat4x4<f32> (
        instance.model_0,
        instance.model_1,
        instance.model_2,
        instance.model_3,
    );

    var out: VertexOut;
    out.color = instance.color;
    out.pos = camera.vp * model * vec4<f32>(in.pos, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return in.color;
}