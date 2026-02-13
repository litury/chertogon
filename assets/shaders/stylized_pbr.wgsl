#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions,
    forward_io::VertexOutput,
}

// Rim light uniform из RimLightExtension (group 3 = MATERIAL_BIND_GROUP_INDEX, binding 100)
struct RimLightSettings {
    color: vec4<f32>,   // rgb = цвет, a = сила
    power: f32,         // Экспонента Френеля (выше = тоньше ободок)
}

@group(3) @binding(100)
var<uniform> rim: RimLightSettings;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    // Стандартный PBR
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    var color = pbr_functions::apply_pbr_lighting(pbr_input);

    // Fresnel rim light
    let V = normalize(pbr_input.V);
    let N = normalize(pbr_input.N);
    let fresnel = pow(1.0 - max(dot(V, N), 0.0), rim.power);
    color = vec4(color.rgb + rim.color.rgb * fresnel * rim.color.a, color.a);

    return color;
}
