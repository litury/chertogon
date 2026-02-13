use bevy::prelude::*;
use crate::toolkit::asset_paths;

/// Ресурс для хранения Handle шрифтов и отслеживания загрузки
#[derive(Resource)]
pub struct FontHandles {
    pub title: Handle<Font>,
    pub ui: Handle<Font>,
    pub ui_bold: Handle<Font>,
    pub logged: bool,
}

/// Загружаем шрифты при старте и сохраняем Handle для проверки
pub fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title = asset_server.load(asset_paths::FONT_TITLE);
    let ui = asset_server.load(asset_paths::FONT_UI);
    let ui_bold = asset_server.load(asset_paths::FONT_UI_BOLD);

    info!("🔤 Loading fonts: {}, {}, {}",
        asset_paths::FONT_TITLE, asset_paths::FONT_UI, asset_paths::FONT_UI_BOLD);

    commands.insert_resource(FontHandles { title, ui, ui_bold, logged: false });
}

/// Проверяем статус загрузки шрифтов каждый кадр до полной загрузки
pub fn check_font_loading(
    asset_server: Res<AssetServer>,
    mut fonts: ResMut<FontHandles>,
) {
    if fonts.logged {
        return;
    }

    let title_state = asset_server.get_load_state(&fonts.title);
    let ui_state = asset_server.get_load_state(&fonts.ui);
    let ui_bold_state = asset_server.get_load_state(&fonts.ui_bold);

    let all_done = title_state.is_some() && ui_state.is_some() && ui_bold_state.is_some();

    // Логируем результат один раз когда все шрифты обработаны
    if all_done {
        info!("🔤 Font '{}': {:?}", asset_paths::FONT_TITLE, title_state);
        info!("🔤 Font '{}': {:?}", asset_paths::FONT_UI, ui_state);
        info!("🔤 Font '{}': {:?}", asset_paths::FONT_UI_BOLD, ui_bold_state);
        fonts.logged = true;
    }
}
