use bevy::prelude::*;
use bevy::winit::WinitSettings;
use avian3d::prelude::*;

pub fn configure_app(app: &mut App) {
    app
        // Основные плагины Bevy
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Yandex Game".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))

        // Физика Avian3D с оптимизированными параметрами
        .add_plugins(
            PhysicsPlugins::new(Update)  // ✅ Используем Update для совместимости с Kinematic bodies
                .with_length_unit(1.0)    // 1 unit = 1 метр
        )
        .insert_resource(Gravity(Vec3::ZERO))  // ✅ ОТКЛЮЧАЕМ гравитацию - top-down игра!
        .insert_resource(SubstepCount(3));     // ✅ 3 подшага для точных коллизий на высоких скоростях

    // ✅ Battery optimization для мобильных устройств
    // На iOS/Android: экономит батарею
    // На desktop: обычный режим для максимальной отзывчивости
    #[cfg(any(target_os = "ios", target_os = "android"))]
    {
        app.insert_resource(WinitSettings::mobile_defaults());
        info!("📱 Mobile battery optimization enabled");
    }

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        app.insert_resource(WinitSettings::game());
        info!("🖥️ Desktop game mode enabled (uncapped FPS)");
    }

    // Inspector для отладки (только в debug режиме)
    // ВРЕМЕННО ОТКЛЮЧЕН - перехватывает WASD ввод!
    // #[cfg(debug_assertions)]
    // {
    //     app.add_plugins(EguiPlugin::default());
    //     app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
    // }
}
