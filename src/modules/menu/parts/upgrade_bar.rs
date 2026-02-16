use bevy::prelude::*;
use crate::modules::menu::components::*;
use crate::modules::progression::components::{UpgradeInventory, UpgradeCategory};
use crate::modules::progression::parts::upgrades;
use crate::toolkit::asset_paths;

/// Перестраивает иконки апгрейдов при изменении UpgradeInventory
pub fn update_upgrade_bar(
    inventory: Res<UpgradeInventory>,
    container_query: Query<Entity, With<UpgradeBarContainer>>,
    existing_icons: Query<Entity, With<UpgradeIcon>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if !inventory.is_changed() {
        return;
    }

    let Ok(container) = container_query.single() else { return };

    // Despawn старых иконок
    for entity in &existing_icons {
        commands.entity(entity).despawn();
    }

    let font = asset_server.load(asset_paths::FONT_UI_BOLD);

    // Создаём иконки для каждого апгрейда
    for &(upgrade_id, level) in &inventory.upgrades {
        let Some(def) = upgrades::get_upgrade_def(&upgrade_id) else { continue };

        let category_color = match def.category {
            UpgradeCategory::Attack => Color::srgb(0.9, 0.3, 0.2),
            UpgradeCategory::Defense => Color::srgb(0.3, 0.6, 0.9),
            UpgradeCategory::Path => Color::srgb(0.3, 0.9, 0.4),
        };

        let icon = commands.spawn((
            HudUI,
            UpgradeIcon,
            Node {
                width: Val::Px(28.0),
                height: Val::Px(28.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(category_color.with_alpha(0.7)),
            BorderColor::all(category_color),
        )).with_children(|icon_parent| {
            icon_parent.spawn((
                HudUI,
                Text::new(format!("{}", level)),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextShadow {
                    offset: Vec2::new(1.0, 1.0),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.9),
                },
            ));
        }).id();

        commands.entity(container).add_child(icon);
    }
}
