use bevy::prelude::*;
use bevy_hanabi::prelude::*;

/// Спавнит парящие угольки/пылинки по всей арене для атмосферы
pub fn setup_ambient_particles(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let mut module = Module::default();

    // Позиция: сфера 20м (покрывает арену)
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(20.0),
        dimension: ShapeDimension::Volume,
    };

    // Lifetime: 3–8 сек
    let lt_min = module.lit(3.0);
    let lt_max = module.lit(8.0);
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        module.uniform(lt_min, lt_max),
    );

    // Начальный размер: 0.01–0.04
    let sz_min = module.lit(Vec3::splat(0.005));
    let sz_max = module.lit(Vec3::splat(0.015));
    let init_size = SetAttributeModifier::new(
        Attribute::SIZE3,
        module.uniform(sz_min, sz_max),
    );

    // Скорость: медленная, направление вверх
    let spd_min = module.lit(0.05);
    let spd_max = module.lit(0.15);
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.uniform(spd_min, spd_max),
    };

    // Медленный подъём
    let update_accel = AccelModifier::constant(&mut module, Vec3::new(0., 0.15, 0.));
    let update_drag = LinearDragModifier::constant(&mut module, 0.5);

    // Круглая форма + биллборд
    let round = RoundModifier::ellipse(&mut module);

    // Градиент цвета: тёплый янтарь → затухание
    let mut color_grad = bevy_hanabi::Gradient::new();
    color_grad.add_key(0.0, Vec4::new(8.0, 4.0, 1.0, 0.0));    // fade-in: горячий янтарь HDR
    color_grad.add_key(0.1, Vec4::new(6.0, 3.0, 0.5, 0.6));    // пик яркости
    color_grad.add_key(0.5, Vec4::new(3.0, 1.0, 0.2, 0.4));    // тлеющий
    color_grad.add_key(1.0, Vec4::new(1.0, 0.3, 0.05, 0.0));   // затухание

    // Кривая размера: fade in → sustain → fade out
    let mut size_grad = bevy_hanabi::Gradient::new();
    size_grad.add_key(0.0, Vec3::ZERO);
    size_grad.add_key(0.1, Vec3::ONE);
    size_grad.add_key(0.9, Vec3::ONE);
    size_grad.add_key(1.0, Vec3::ZERO);

    // 20 частиц/сек × 8 сек ≈ 160 частиц макс
    let effect = EffectAsset::new(1024, SpawnerSettings::rate(20.0_f32.into()), module)
        .with_name("ambient_embers")
        .with_alpha_mode(bevy_hanabi::AlphaMode::Add)
        .init(init_pos)
        .init(init_lifetime)
        .init(init_size)
        .init(init_vel)
        .update(update_accel)
        .update(update_drag)
        .render(ColorOverLifetimeModifier::new(color_grad))
        .render(SizeOverLifetimeModifier {
            gradient: size_grad,
            screen_space_size: false,
        })
        .render(OrientModifier::new(OrientMode::ParallelCameraDepthPlane))
        .render(round);

    let handle = effects.add(effect);

    commands.spawn((
        ParticleEffect::new(handle),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}
