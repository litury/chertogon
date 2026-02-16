#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::globals

struct PortalVortexSettings {
    color: vec4<f32>,
    speed: f32,
    twist: f32,
    _pad1: f32,
    _pad2: f32,
}

@group(3) @binding(0)
var<uniform> settings: PortalVortexSettings;

// ---- Simplex noise (из ground_fog.wgsl, проверен на WebGL2) ----

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

// ---- Portal vortex ----

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let t = globals.time * settings.speed;

    // UV → полярные координаты (центр 0.5, 0.5)
    let uv = in.uv - vec2(0.5);
    let dist = length(uv);
    let angle = atan2(uv.y, uv.x);

    // Спиральное искажение: дальние точки вращаются больше
    let spiral_angle = angle + dist * settings.twist + t * 2.0;

    // Два слоя noise для глубины
    let n1 = snoise(vec2(spiral_angle * 1.5, dist * 4.0 - t * 1.5)) * 0.5 + 0.5;
    let n2 = snoise(vec2(spiral_angle * 3.0 + 17.0, dist * 8.0 - t * 2.5)) * 0.5 + 0.5;
    let noise_val = n1 * 0.7 + n2 * 0.3;

    // Затягивание к центру: ярче в центре, но минимум 0.4 на краях
    let center_glow = smoothstep(0.5, 0.05, dist) * 0.6 + 0.4;

    // Мягкий круглый край — обрезка за пределами радиуса
    let edge_fade = 1.0 - smoothstep(0.48, 0.5, dist);

    // Финальная яркость
    let intensity = noise_val * center_glow * edge_fade;

    // Пульсация яркости
    let pulse = 0.85 + 0.15 * sin(t * 3.0);

    let final_color = settings.color.rgb * intensity * pulse * 4.0;

    return vec4(final_color, intensity * edge_fade * pulse);
}
