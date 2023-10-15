// Line Shader

struct VertexData {
    @location(0) position: vec2<f32>,
};

struct FragmentData {
    @builtin(position) position: vec4<f32>,
    @location(8) color: vec4<f32>,
};

const width: f32 = 800.0;
const height: f32 = 600.0;

@vertex
fn vs_main(vertex: VertexData) -> FragmentData {
    var out: FragmentData;
    out.position = vec4f(vertex.position.xy, 0.0, 1.0);
    out.position.x = out.position.x/width * 2.0 - 1.0;
    out.position.y = 1.0 - out.position.y/height * 2.0;
    out.color = vec4f(0.0, 0.0, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(frag: FragmentData) -> @location(0) vec4<f32> {
    return frag.color;
}
