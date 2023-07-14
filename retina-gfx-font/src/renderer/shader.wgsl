struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

struct VertexUniform {
    transform: mat4x4<f32>,
};

struct FragmentUniform {
    color: vec4<f32>,
}

@group(0)
@binding(2)
var<uniform> vertex_uniform: VertexUniform;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vertex_uniform.transform * vec4<f32>(model.position, 1.0);
    return out;
}

@group(0)
@binding(0)
var t_diffuse: texture_2d<f32>;

@group(0)
@binding(1)
var s_diffuse: sampler;

@group(0)
@binding(3)
var<uniform> fragment_uniform: FragmentUniform;

const MAX_OPACITY: f32 = 0.000;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var mask = textureSample(t_diffuse, s_diffuse, in.tex_coords);

    if (mask.r <= MAX_OPACITY && mask.g <= MAX_OPACITY && mask.b <= MAX_OPACITY) {
        discard;
    }

    var color = fragment_uniform.color * mask;

    return color;
}
