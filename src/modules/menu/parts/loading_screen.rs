use bevy::prelude::*;
use bevy::asset::LoadState;
use crate::shared::GameState;
use crate::toolkit::asset_paths;
use crate::modules::menu::parts::fade_transition::FadeState;

/// Маркер для UI элементов экрана загрузки
#[derive(Component)]
pub struct LoadingScreenUI;

/// Маркер для заполненной части прогресс-бара
#[derive(Component)]
pub struct LoadingBarFill;

/// Маркер для текста процентов
#[derive(Component)]
pub struct LoadingPercentText;

/// Ресурс с handles всех предзагружаемых ассетов
#[derive(Resource)]
pub struct AssetPreloader {
    handles: Vec<UntypedHandle>,
    total: usize,
    /// Загрузка завершена — ждём fade
    done: bool,
}

/// Запускает предзагрузку всех игровых ассетов и показывает UI
pub fn setup_loading_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("📦 Loading screen: начинаем предзагрузку ассетов...");

    let mut handles: Vec<UntypedHandle> = Vec::new();

    // GLB модели (загружаем Scene0 — это триггерит загрузку всего GLB включая анимации)
    let glb_paths: &[&str] = &[
        asset_paths::BOGATYR_MODEL,
        asset_paths::SWORD_MODEL,
        asset_paths::UPYR_MODEL,
        asset_paths::LESHIY_MODEL,
        asset_paths::VOLKOLAK_MODEL,
        asset_paths::CLIFF_WALL_A,
        asset_paths::CLIFF_WALL_B,
        asset_paths::CLIFF_WALL_C,
        asset_paths::RUNE_STONE,
        asset_paths::ROCK_LARGE,
        asset_paths::DEAD_TREE,
        asset_paths::BONE_PILE,
    ];
    for path in glb_paths {
        let h: Handle<Scene> = asset_server.load(*path);
        handles.push(h.untyped());
    }

    // Текстуры
    let texture_paths: &[&str] = &[
        asset_paths::FLOOR_DIFF,
        asset_paths::FLOOR_NORMAL,
        asset_paths::GAMEOVER_BG,
        asset_paths::PORTRAIT_BOGATYR,
        asset_paths::PORTRAIT_UPYR,
        asset_paths::PORTRAIT_LESHIY,
        asset_paths::PORTRAIT_VOLKOLAK,
    ];
    for path in texture_paths {
        let h: Handle<Image> = asset_server.load(*path);
        handles.push(h.untyped());
    }

    let total = handles.len();
    info!("📦 Предзагрузка {} ассетов", total);

    commands.insert_resource(AssetPreloader {
        handles,
        total,
        done: false,
    });

    // UI загрузки
    let font = asset_server.load(asset_paths::FONT_UI);
    let font_bold = asset_server.load(asset_paths::FONT_UI_BOLD);

    commands.spawn((
        LoadingScreenUI,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(24.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.05, 0.04, 0.08)),
    )).with_children(|parent| {
        // Текст "ЗАГРУЗКА"
        parent.spawn((
            LoadingScreenUI,
            Text::new("ЗАГРУЗКА"),
            TextFont {
                font: font_bold.clone(),
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::srgba(0.95, 0.8, 0.3, 0.9)),
        ));

        // Контейнер прогресс-бара (фон)
        parent.spawn((
            LoadingScreenUI,
            Node {
                width: Val::Px(300.0),
                height: Val::Px(8.0),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.1)),
        )).with_children(|bar_bg| {
            // Заполненная часть (растёт по ширине)
            bar_bg.spawn((
                LoadingScreenUI,
                LoadingBarFill,
                Node {
                    width: Val::Percent(0.0),
                    height: Val::Percent(100.0),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.95, 0.75, 0.2)),
            ));
        });

        // Текст процентов
        parent.spawn((
            LoadingScreenUI,
            LoadingPercentText,
            Text::new("0%"),
            TextFont {
                font,
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
        ));
    });
}

/// Обновляет прогресс-бар и переходит в Playing когда всё загружено
pub fn update_loading_progress(
    asset_server: Res<AssetServer>,
    mut preloader: ResMut<AssetPreloader>,
    mut bar_query: Query<&mut Node, With<LoadingBarFill>>,
    mut text_query: Query<&mut Text, With<LoadingPercentText>>,
    mut fade: ResMut<FadeState>,
) {
    if preloader.done {
        return;
    }

    // Считаем ассеты как "готовые" если загружены ИЛИ упали с ошибкой
    // (Failed ассеты не должны блокировать загрузку)
    let mut done_count = 0;
    let mut failed_count = 0;
    for h in &preloader.handles {
        match asset_server.load_state(h.id()) {
            LoadState::Loaded => done_count += 1,
            LoadState::Failed(_) => { done_count += 1; failed_count += 1; },
            _ => {},
        }
    }

    let progress = if preloader.total > 0 {
        done_count as f32 / preloader.total as f32
    } else {
        1.0
    };

    // Обновляем прогресс-бар
    if let Ok(mut node) = bar_query.single_mut() {
        node.width = Val::Percent(progress * 100.0);
    }

    // Обновляем текст
    if let Ok(mut text) = text_query.single_mut() {
        **text = format!("{}%", (progress * 100.0) as u32);
    }

    // Все ассеты обработаны (загружены или failed) → переход в Playing
    if done_count >= preloader.total {
        if failed_count > 0 {
            warn!("⚠️ {} ассетов не загрузились, продолжаем без них", failed_count);
        }
        info!("✅ Загрузка завершена ({}/{} OK) — переход в Playing", done_count - failed_count, preloader.total);
        preloader.done = true;
        fade.start_fade(GameState::Playing, false);
    }
}

/// Очищает UI загрузки
pub fn cleanup_loading_screen(
    mut commands: Commands,
    query: Query<Entity, (With<LoadingScreenUI>, Without<ChildOf>)>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<AssetPreloader>();
}
