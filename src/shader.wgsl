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

const thread_radius: f32 = 0.15;
const end_falloff: f32 = 0.2;

const vertpos: array<vec2f, 6> = array(
    vec2f(0, 1),
    vec2f(0, -1),
    vec2f(1, -1),
    vec2f(0, 1),
    vec2f(1, -1),
    vec2f(1, 1),
);

struct VSOut {
    @builtin(position) pos: vec4f,
    @location(0) color: vec4f,
    @location(1) uuv: vec3f,
}

@vertex
fn vs_main(@builtin(vertex_index) vert_idx: u32, @builtin(instance_index) inst: u32) -> VSOut {
    let stitch = vert_idx / 6;
    let vert =  vertpos[vert_idx % 6];
    let s1 = stitches[stitch].location;
    let s2 = stitches[stitch+1].location;

    let u = s2 - s1;
    let v = thread_radius * normalize(vec2f(-u.y, u.x));

    let xy = s1 + vert.x * u + vert.y * v;
    let l = length(u);
    let clip_xy = (xy - camera.center) / camera.zoom_xy;

    var out: VSOut;
    out.pos = vec4f(clip_xy, 0.0, 1.0);
    out.color = blocks[inst].color;
    out.uuv = vec3f(l * vert.x, l * (1 - vert.x), vert.y);
    return out;
}

@fragment
fn fs_main(v: VSOut) -> @location(0) vec4f {
    let end_dist = max(0.0, 1.0 - min(v.uuv.x, v.uuv.y) / end_falloff);
    let center_dist = v.uuv.z;
    let d: f32 = end_dist * end_dist + center_dist * center_dist;
    let shade = 1.1 - (0.8 * d);
    if d > 1 {
        discard;
    }
    return vec4f(v.color.xyz * shade, 1.0);
}
