use bevy::prelude::*;
use crate::modules::enemies::components::{SpawnPortal, PortalVortex, PortalLight, WavePhase, WaveState};
use crate::shared::rand_01;

/// Кэшированные ассеты для частиц порталов
#[derive(Resource)]
pub struct PortalVfxAssets {
    /// Меш частицы дыма (сфера)
    pub smoke_mesh: Handle<Mesh>,
    /// Материал дыма портала 0 (тёмно-фиолетовый)
    pub smoke_material_0: Handle<StandardMaterial>,
    /// Материал дыма портала 1 (болотно-зелёный)
    pub smoke_material_1: Handle<StandardMaterial>,
    /// Меш искры (маленькая сфера)
    pub spark_mesh: Handle<Mesh>,
    /// Материал искры портала 0 (красный)
    pub spark_material_0: Handle<StandardMaterial>,
    /// Материал искры портала 1 (зелёный)
    pub spark_material_1: Handle<StandardMaterial>,
}

/// Инициализация ассетов VFX порталов
pub fn init_portal_vfx_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let smoke_mesh = meshes.add(Sphere::new(0.15));
    let spark_mesh = meshes.add(Sphere::new(0.05));

    // Портал 0 — "Разлом Огня": фиолетовый дым, красные искры
    let smoke_material_0 = materials.add(StandardMaterial {
        base_color: Color::srgba(0.15, 0.0, 0.2, 0.5),
        emissive: LinearRgba::new(0.4, 0.0, 0.6, 1.0),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let spark_material_0 = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.15, 0.05),
        emissive: LinearRgba::new(8.0, 0.5, 0.2, 1.0),
        unlit: true,
        ..default()
    });

    // Портал 1 — "Разлом Тьмы": болотный дым, зелёные искры
    let smoke_material_1 = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 0.12, 0.08, 0.5),
        emissive: LinearRgba::new(0.0, 0.4, 0.2, 1.0),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let spark_material_1 = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 1.0, 0.3),
        emissive: LinearRgba::new(0.5, 8.0, 1.5, 1.0),
        unlit: true,
        ..default()
    });

    commands.insert_resource(PortalVfxAssets {
        smoke_mesh,
        smoke_material_0,
        smoke_material_1,
        spark_mesh,
        spark_material_0,
        spark_material_1,
    });
}

// ─── Пульсация порталов ────────────────────────────────────────────

/// Пульсация масштаба и яркости порталов в зависимости от фазы волны
pub fn portal_pulse_system(
    time: Res<Time>,
    wave: Res<WaveState>,
    portals: Query<&Children, With<SpawnPortal>>,
    mut vortex_q: Query<&mut Transform, With<PortalVortex>>,
    mut light_q: Query<&mut PointLight, With<PortalLight>>,
) {
    let t = time.elapsed_secs();

    // Интенсивность пульсации по фазе
    let pulse_intensity = match wave.phase {
        WavePhase::Cooldown => {
            let remaining = wave.wave_cooldown.remaining_secs();
            if remaining < 1.0 {
                1.0 - remaining // Нарастание за последнюю секунду
            } else {
                0.0
            }
        }
        WavePhase::Spawning => 1.0,
        WavePhase::Fighting => 0.3 + 0.1 * (t * 2.0).sin(),
    };

    let base_scale = portal_scale_for_wave(wave.current_wave);

    for children in &portals {
        for child in children.iter() {
            if let Ok(mut transform) = vortex_q.get_mut(child) {
                let pulse = 1.0 + pulse_intensity * 0.15 * (t * 4.0).sin();
                transform.scale = Vec3::splat(base_scale * pulse);
            }

            if let Ok(mut light) = light_q.get_mut(child) {
                let base = 150_000.0;
                let boost = pulse_intensity * 100_000.0 * (t * 3.0).sin().abs();
                light.intensity = base + boost;
            }
        }
    }
}

/// Масштаб портала по номеру волны (эскалация визуала)
fn portal_scale_for_wave(wave: u32) -> f32 {
    match wave {
        0..=4 => 1.0,    // 3м диаметр
        5..=9 => 1.333,  // 4м диаметр
        _ => 1.667,      // 5м диаметр
    }
}

// ─── Частицы порталов ──────────────────────────────────────────────

/// Маркер частицы дыма из портала
#[derive(Component)]
pub struct PortalSmokeParticle {
    pub velocity: Vec3,
    pub timer: Timer,
}

/// Маркер частицы искры из портала
#[derive(Component)]
pub struct PortalSparkParticle {
    pub velocity: Vec3,
    pub timer: Timer,
}

/// Таймер эмиссии частиц (чтобы не спавнить каждый кадр)
#[derive(Resource)]
pub struct PortalEmitTimer {
    pub timer: Timer,
}

impl Default for PortalEmitTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.15, TimerMode::Repeating),
        }
    }
}

/// Спавнит частицы дыма и искр из порталов
pub fn portal_particle_emitter_system(
    time: Res<Time>,
    wave: Res<WaveState>,
    mut emit_timer: ResMut<PortalEmitTimer>,
    portals: Query<(&SpawnPortal, &Transform)>,
    vfx_assets: Option<Res<PortalVfxAssets>>,
    mut commands: Commands,
) {
    let Some(assets) = vfx_assets else { return };

    // Эмитим частицы только при спавне или за 1с до волны
    let should_emit = match wave.phase {
        WavePhase::Spawning => true,
        WavePhase::Cooldown => wave.wave_cooldown.remaining_secs() < 1.0,
        WavePhase::Fighting => false,
    };
    if !should_emit { return; }

    emit_timer.timer.tick(time.delta());
    if !emit_timer.timer.just_finished() { return; }

    for (portal, transform) in &portals {
        let pos = transform.translation;
        let idx = portal.index;

        // Выбираем материалы по индексу портала
        let (smoke_mat, spark_mat) = if idx == 0 {
            (&assets.smoke_material_0, &assets.spark_material_0)
        } else {
            (&assets.smoke_material_1, &assets.spark_material_1)
        };

        // 2 частицы дыма
        for _ in 0..2 {
            let dir = Vec3::new(
                (rand_01() - 0.5) * 2.0,           // Разброс по X
                (rand_01() - 0.3) * 1.5,
                1.0 + rand_01() * 2.0,              // Вглубь арены (+Z)
            );
            commands.spawn((
                Mesh3d(assets.smoke_mesh.clone()),
                MeshMaterial3d(smoke_mat.clone()),
                Transform::from_translation(pos + Vec3::new(0.0, 0.0, 0.5)),
                PortalSmokeParticle {
                    velocity: dir.normalize() * (1.0 + rand_01()),
                    timer: Timer::from_seconds(0.8 + rand_01() * 0.4, TimerMode::Once),
                },
            ));
        }

        // 1 искра
        let spark_dir = Vec3::new(
            (rand_01() - 0.5) * 3.0,
            rand_01() * 2.0,
            1.5 + rand_01() * 3.0,              // Вглубь арены (+Z)
        );
        commands.spawn((
            Mesh3d(assets.spark_mesh.clone()),
            MeshMaterial3d(spark_mat.clone()),
            Transform::from_translation(pos + Vec3::new(0.0, 0.0, 0.3)),
            PortalSparkParticle {
                velocity: spark_dir.normalize() * (3.0 + rand_01() * 2.0),
                timer: Timer::from_seconds(0.3 + rand_01() * 0.2, TimerMode::Once),
            },
        ));
    }
}

/// Движение и despawn частиц дыма
pub fn portal_smoke_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut PortalSmokeParticle, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (entity, mut particle, mut transform) in &mut query {
        particle.timer.tick(time.delta());

        // Медленный дрейф вверх + замедление
        particle.velocity *= 1.0 - dt * 2.0;
        particle.velocity.y += dt * 0.5;
        transform.translation += particle.velocity * dt;

        // Растёт и затухает
        let progress = particle.timer.fraction();
        let scale = (1.0 - progress) * (0.5 + progress * 1.5);
        transform.scale = Vec3::splat(scale.max(0.01));

        if particle.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Движение и despawn частиц искр
pub fn portal_spark_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut PortalSparkParticle, &mut Transform)>,
) {
    let dt = time.delta_secs();
    let gravity = Vec3::new(0.0, -8.0, 0.0);

    for (entity, mut particle, mut transform) in &mut query {
        particle.timer.tick(time.delta());

        particle.velocity += gravity * dt;
        transform.translation += particle.velocity * dt;

        // Уменьшаемся
        let progress = particle.timer.fraction();
        transform.scale = Vec3::splat((1.0 - progress).max(0.01));

        if particle.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
