struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) top_left: vec2<f32>,
    @location(1) bottom_right: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) border_radius: f32,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) border_radius: f32,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    switch input.vertex_index {
        case 0u: { out.position = vec4<f32>(input.top_left, 0.0, 1.0); }
        case 1u: { out.position = vec4<f32>(input.bottom_right.x, input.top_left.y, 0.0, 1.0); }
        case 2u: { out.position = vec4<f32>(input.top_left.x, input.bottom_right.y, 0.0, 1.0); }
        case 3u: { out.position = vec4<f32>(input.bottom_right, 0.0, 1.0); }
        default: { out.position = vec4<f32>(0.0, 0.0, 0.0, 0.0); }
    }

    out.color = input.color;
    out.border_radius = input.border_radius;

    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}