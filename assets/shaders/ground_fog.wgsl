#import bevy_pbr::{
    forward_io::Vertex,
    mesh_functions::{get_world_from_local, mesh_position_local_to_world},
    view_transformations::position_world_to_clip,
}

struct GroundFogSettings {
    fog_color: vec4<f32>,
    time: f32,
    speed: f32,
    max_height: f32,
    density: f32,
    layer_index: f32,
    _pad1: f32,
    _pad2: f32,
    _pad3: f32,
}

@group(3) @binding(0)
var<uniform> settings: GroundFogSettings;

// ---- Simplex noise (vertex shader — ~4096 вызовов, не миллионы пикселей) ----

fn mod289v2(x: vec2<f32>) -> vec2<f32> { return x - floor(x * (1.0 / 289.0)) * 289.0; }
fn mod289v3(x: vec3<f32>) -> vec3<f32> { return x - floor(x * (1.0 / 289.0)) * 289.0; }
fn permute3(x: vec3<f32>) -> vec3<f32> { return mod289v3(((x * 34.0) + 1.0) * x); }

fn snoise(v: vec2<f32>) -> f32 {
    let C = vec4(0.211324865405187, 0.366025403784439, -0.577350269189626, 0.024390243902439);
    var i = floor(v + dot(v, C.yy));
    let x0 = v - i + dot(i, C.xx);
    var i1: vec2<f32>;
    if x0.x > x0.y { i1 = vec2(1.0, 0.0); } else { i1 = vec2(0.0, 1.0); }
    let x12 = x0.xyxy + C.xxzz - vec4(i1.x, i1.y, 0.0, 0.0);
    i = mod289v2(i);
    let p = permute3(permute3(i.y + vec3(0.0, i1.y, 1.0)) + i.x + vec3(0.0, i1.x, 1.0));
    var m = max(vec3(0.5) - vec3(dot(x0, x0), dot(x12.xy, x12.xy), dot(x12.zw, x12.zw)), vec3(0.0));
    m = m * m;
    m = m * m;
    let x = 2.0 * fract(p * C.www) - 1.0;
    let h = abs(x) - 0.5;
    let ox = floor(x + 0.5);
    let a0 = x - ox;
    m = m * (1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h));
    return 130.0 * dot(m, vec3(a0.x * x0.x + h.x * x0.y, a0.y * x12.x + h.y * x12.y, a0.z * x12.z + h.z * x12.w));
}

fn fbm(p: vec2<f32>) -> f32 {
    var val = 0.0;
    var amp = 0.6;
    var pos = p;
    val += amp * snoise(pos); pos = pos * 2.03 + vec2(1.7, 9.2); amp *= 0.45;
    val += amp * snoise(pos); pos = pos * 2.01 + vec2(5.1, 3.8); amp *= 0.45;
    val += amp * snoise(pos);
    return val * 0.5 + 0.5;
}

// ---- Vertex / Fragment I/O ----

struct FogVaryings {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_xz: vec2<f32>,
    @location(1) displacement: f32,
}

@vertex
fn vertex(vertex: Vertex) -> FogVaryings {
    let world_from_local = get_world_from_local(vertex.instance_index);
    var world_pos = mesh_position_local_to_world(world_from_local, vec4(vertex.position, 1.0));

    let t = settings.time * settings.speed;
    let layer_off = settings.layer_index * 43.7;
    let n = fbm(world_pos.xz * 0.09 + t * vec2(0.18, 0.12) + layer_off);
    let disp = max(n - 0.25, 0.0) * settings.max_height;

    world_pos.y += disp;

    var out: FogVaryings;
    out.clip_position = position_world_to_clip(world_pos.xyz);
    out.world_xz = world_pos.xz;
    out.displacement = disp;
    return out;
}

@fragment
fn fragment(in: FogVaryings) -> @location(0) vec4<f32> {
    // Выше вершина — прозрачнее (верхушки "холмов" тают)
    let height_alpha = 1.0 - smoothstep(0.0, settings.max_height * 0.85, in.displacement);

    // Затухание к краям арены
    let dist = length(in.world_xz);
    let edge_fade = 1.0 - smoothstep(18.0, 24.0, dist);

    let alpha = height_alpha * settings.density * edge_fade;

    return vec4(settings.fog_color.rgb, alpha);
}
