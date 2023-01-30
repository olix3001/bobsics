struct Globals {
    u_resolution: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> globals: Globals;

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
    @location(2) top_left: vec2<f32>,
    @location(3) bottom_right: vec2<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    var top_left: vec2<f32> = vec2<f32>(0.0, 0.0);
    var bottom_right: vec2<f32> = vec2<f32>(0.0, 0.0);

    // Calculate the top left and bottom right coordinates of the rectangle
    if (input.top_left.x <= input.bottom_right.x) {
        top_left.x = input.top_left.x;
        bottom_right.x = input.bottom_right.x;
    } else {
        top_left.x = input.bottom_right.x;
        bottom_right.x = input.top_left.x;
    }
    if (input.top_left.y > input.bottom_right.y) {
        top_left.y = input.top_left.y;
        bottom_right.y = input.bottom_right.y;
    } else {
        top_left.y = input.bottom_right.y;
        bottom_right.y = input.top_left.y;
    }

    // Draw the rectangle (CCW winding order)
    switch input.vertex_index {
        case 0u, 4u: { out.position = vec4<f32>(top_left, 0.0, 1.0); }
        case 1u: { out.position = vec4<f32>(top_left.x, bottom_right.y, 0.0, 1.0); }
        case 2u, 5u: { out.position = vec4<f32>(bottom_right, 0.0, 1.0); }
        case 3u: { out.position = vec4<f32>(bottom_right.x, top_left.y, 0.0, 1.0); }
        default: { out.position = vec4<f32>(0.0, 0.0, 0.0, 0.0); }
    }

    out.color = input.color;
    out.border_radius = input.border_radius;

    out.top_left = top_left;
    out.bottom_right = bottom_right;

    return out;
}

fn ndc_to_screen(ndc: vec2<f32>) -> vec2<f32> {
    // ndc to screen

    // flip y
    let ndc = vec2<f32>(ndc.x, -ndc.y);

    // translate
    let ndc = ndc + vec2<f32>(1.0, 1.0);

    // scale to screen
    let screen = ndc * globals.u_resolution / 2.0;

    return screen;
}

fn box_dist(p: vec2<f32>, size: vec2<f32>, r: f32) -> f32 {
    let size = size - vec2<f32>(r, r);
    let d = abs(p) - size;
    return length(max(d, vec2<f32>(0.0))) + min(max(d.x, d.y), 0.0) - r;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Translate 
    let top_left = ndc_to_screen(input.top_left);
    let bottom_right = ndc_to_screen(input.bottom_right);

    let size = bottom_right - top_left;
    let center = top_left + size / 2.0;

    // Move p relative to the center of the rectangle
    let p = input.position.xy - center;

    // calculate distance to the rectangle
    let dist = box_dist(p, size/2.0, input.border_radius);

    // Calculate the alpha
    let alpha = 1.0 - smoothstep(-0.75, -0.1, dist);

    // Return the color with the alpha
    return vec4<f32>(input.color.rgb, alpha);
}