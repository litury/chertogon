use bevy::prelude::*;
use bevy::asset::AssetMetaCheck;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::winit::WinitSettings;
use avian3d::prelude::*;

#[cfg(feature = "remote_debug")]
use bevy::remote::{RemotePlugin, http::RemoteHttpPlugin};

pub fn configure_app(app: &mut App) {
    app
        // Основные плагины Bevy
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Yandex Game".into(),
                    resolution: (1280, 720).into(),
                    ..default()
                }),
                ..default()
            })
            // Отключить проверку .meta файлов (WASM: HTTP 404 → ошибки парсинга)
            .set(bevy::asset::AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
        )

        // Физика Avian3D с оптимизированными параметрами
        .add_plugins(
            PhysicsPlugins::new(Update)  // ✅ Используем Update для совместимости с Kinematic bodies
                .with_length_unit(1.0)    // 1 unit = 1 метр
        )
        .insert_resource(Gravity(Vec3::ZERO))  // ✅ ОТКЛЮЧАЕМ гравитацию - top-down игра!
        .insert_resource(ClearColor(Color::srgb(0.05, 0.04, 0.08)))  // Тьма за ареной (совпадает с туманом)
        .insert_resource(SubstepCount(3))     // ✅ 3 подшага для точных коллизий на высоких скоростях
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_state::<crate::shared::GameState>();

    #[cfg(target_arch = "wasm32")]
    {
        app.insert_resource(WinitSettings::default());
        info!("WASM browser mode");
    }

    #[cfg(all(not(target_arch = "wasm32"), any(target_os = "ios", target_os = "android")))]
    {
        app.insert_resource(WinitSettings::mobile_defaults());
        info!("📱 Mobile battery optimization enabled");
    }

    #[cfg(all(not(target_arch = "wasm32"), not(any(target_os = "ios", target_os = "android"))))]
    {
        app.insert_resource(WinitSettings::game());
        info!("🖥️ Desktop game mode enabled (uncapped FPS)");
    }

    // BRP для live-диагностики (cargo run --features remote_debug)
    #[cfg(feature = "remote_debug")]
    {
        use crate::modules::player::components::*;
        use crate::modules::enemies::components::*;
        use crate::modules::combat::components::*;
        use crate::modules::combat::parts::hit_particles::HitParticle;
        use crate::modules::combat::parts::slash_vfx::SlashVfx;
        use crate::modules::camera::components::*;

        app.add_plugins((RemotePlugin::default(), RemoteHttpPlugin::default()));

        app.register_type::<Player>()
           .register_type::<PlayerAnimState>()
           .register_type::<AnimationState>()
           .register_type::<PlayerModel>()
           .register_type::<AnimationSetupComplete>()
           .register_type::<Enemy>()
           .register_type::<Health>()
           .register_type::<EnemyType>()
           .register_type::<EnemyModel>()
           .register_type::<EnemyAnimationSetupComplete>()
           .register_type::<EnemyAnimState>()
           .register_type::<EnemyAnim>()
           .register_type::<EnemyDying>()
           .register_type::<ChasePlayer>()
           .register_type::<EnemyCorpse>()
           .register_type::<Weapon>()
           .register_type::<AttackCooldown>()
           .register_type::<AttackAnimTimer>()
           .register_type::<PlayerHealth>()
           .register_type::<EnemyAttackCooldown>()
           .register_type::<HitParticle>()
           .register_type::<SlashVfx>()
           .register_type::<CameraTarget>()
           .register_type::<CameraZoom>();

        info!("BRP enabled on port 15702");
    }

    // Inspector для отладки (только в debug режиме)
    // ВРЕМЕННО ОТКЛЮЧЕН - перехватывает WASD ввод!
    // #[cfg(debug_assertions)]
    // {
    //     app.add_plugins(EguiPlugin::default());
    //     app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
    // }
}
