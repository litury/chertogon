use bevy::prelude::*;
use chertogon::config::game_config;
use chertogon::modules::{WorldPlugin, InputPlugin, PlayerPlugin, CameraPlugin, EnemiesPlugin, CombatPlugin, MenuPlugin, SelectionPlugin, ProgressionPlugin};

fn main() {
    let mut app = App::new();

    // Конфигурация из модуля config
    game_config::configure_app(&mut app);

    // Игровые модули (плагины)
    app.add_plugins((
        WorldPlugin,
        InputPlugin,
        PlayerPlugin,
        CameraPlugin,
        EnemiesPlugin,
        CombatPlugin,
        MenuPlugin,
        SelectionPlugin,
        ProgressionPlugin,
    ));

    app.run();
}
