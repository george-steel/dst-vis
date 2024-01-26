struct BlockData {
    color: vec4f,
}

struct StitchData {
    location: vec2f,
}

struct Camera2D{
    zoom_xy: vec2f,
    center: vec2f,
}

@group(0) @binding(0) var<uniform> camera: Camera2D;
@group(0) @binding(1) var<storage,read> blocks: array<BlockData>;
@group(0) @binding(2) var<storage,read> stitches: array<StitchData>;

struct VSOut {
    @builtin(position) pos: vec4f,
    @location(0) color: vec4f,
}

@vertex
fn vs_main(@builtin(vertex_index) idx: u32, @builtin(instance_index) inst: u32) -> VSOut {
    let xy = stitches[idx].location;
    var out: VSOut;
    let clip_xy = (xy - camera.center) / camera.zoom_xy;
    out.pos = vec4<f32>(clip_xy, 0.0, 1.0);
    out.color = blocks[inst].color;
    return out;
}

@fragment
fn fs_main(v: VSOut) -> @location(0) vec4<f32> {
    return v.color;
}
