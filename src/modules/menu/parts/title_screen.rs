use bevy::prelude::*;
use crate::shared::GameState;
use crate::modules::menu::components::*;
use crate::modules::menu::parts::fade_transition::FadeState;
use crate::toolkit::asset_paths;

/// Создаёт Title Screen с фоновым изображением, виньеткой и анимациями
pub fn setup_title_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font_title = asset_server.load(asset_paths::FONT_TITLE);
    let font_ui = asset_server.load(asset_paths::FONT_UI);
    let font_ui_bold = asset_server.load(asset_paths::FONT_UI_BOLD);
    let bg_image: Handle<Image> = asset_server.load(asset_paths::TITLE_BG);

    // Root — fullscreen контейнер для контента
    commands.spawn((
        TitleScreenUI,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(16.0),
            ..default()
        },
    )).with_children(|parent| {
        // Фоновое изображение (absolute, fullscreen, за контентом)
        parent.spawn((
            TitleScreenUI,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ImageNode::new(bg_image),
        ));

        // Виньетка поверх фона (затемнение центра для читаемости текста)
        parent.spawn((
            TitleScreenUI,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundGradient::from(RadialGradient {
                stops: vec![
                    ColorStop::new(Color::srgba(0.0, 0.0, 0.0, 0.5), Val::Percent(0.0)),
                    ColorStop::new(Color::srgba(0.03, 0.02, 0.02, 0.3), Val::Percent(45.0)),
                    ColorStop::new(Color::srgba(0.03, 0.02, 0.02, 0.7), Val::Percent(100.0)),
                ],
                ..default()
            }),
        ));
        // Контейнер заголовка с BoxShadow glow (огненное свечение вокруг)
        parent.spawn((
            TitleScreenUI,
            Node {
                padding: UiRect::axes(Val::Px(30.0), Val::Px(8.0)),
                ..default()
            },
            BoxShadow(vec![
                ShadowStyle {
                    color: Color::srgba(1.0, 0.5, 0.1, 0.25),
                    x_offset: Val::Px(0.0),
                    y_offset: Val::Px(0.0),
                    spread_radius: Val::Px(10.0),
                    blur_radius: Val::Px(50.0),
                },
                ShadowStyle {
                    color: Color::srgba(0.85, 0.12, 0.08, 0.12),
                    x_offset: Val::Px(0.0),
                    y_offset: Val::Px(0.0),
                    spread_radius: Val::Px(5.0),
                    blur_radius: Val::Px(80.0),
                },
            ]),
        )).with_children(|title_box| {
            // "ЧЕРТОГОН" — кровавый красный + золотой TextShadow (огненная подсветка)
            title_box.spawn((
                TitleScreenUI,
                Text::new("ЧЕРТОГОН"),
                TextFont {
                    font: font_title,
                    font_size: 96.0,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.12, 0.08)),
                TextShadow {
                    offset: Vec2::new(4.0, 4.0),
                    color: Color::srgba(1.0, 0.5, 0.1, 0.8),
                },
            ));
        });

        // Лор — пергаментный цвет с чёрной тенью для читаемости
        parent.spawn((
            TitleScreenUI,
            Text::new("Мечом и верой — очисти землю от нечисти.\nArena roguelike в славянском сеттинге."),
            TextFont {
                font: font_ui.clone(),
                font_size: 22.0,
                ..default()
            },
            TextColor(Color::srgb(0.65, 0.6, 0.5)),
            TextShadow {
                offset: Vec2::new(2.0, 2.0),
                color: Color::srgba(0.0, 0.0, 0.0, 0.9),
            },
            TextLayout::new_with_justify(Justify::Center),
        ));

        // Кнопка "НАЧАТЬ" — золотой BorderGradient + BoxShadow glow
        parent.spawn((
            TitleScreenUI,
            Node {
                margin: UiRect::top(Val::Px(28.0)),
                padding: UiRect::axes(Val::Px(44.0), Val::Px(16.0)),
                border: UiRect::all(Val::Px(1.5)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.95, 0.7, 0.2, 0.06)),
            BorderGradient::from(LinearGradient {
                angle: 90_f32.to_radians(),
                stops: vec![
                    Color::srgba(0.95, 0.7, 0.2, 0.8).into(),
                    Color::srgba(1.0, 0.5, 0.1, 0.3).into(),
                ],
                ..default()
            }),
            BoxShadow(vec![ShadowStyle {
                color: Color::srgba(0.95, 0.7, 0.2, 0.15),
                x_offset: Val::Px(0.0),
                y_offset: Val::Px(0.0),
                spread_radius: Val::Px(2.0),
                blur_radius: Val::Px(15.0),
            }]),
            Button,
        )).with_children(|btn| {
            btn.spawn((
                TitleScreenUI,
                Text::new("НАЧАТЬ"),
                TextFont {
                    font: font_ui_bold,
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.75, 0.3)),
                TextShadow {
                    offset: Vec2::new(1.0, 1.0),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.5),
                },
            ));
        });

        // Пульсирующий текст-подсказка
        parent.spawn((
            TitleScreenUI,
            PulsingText,
            Text::new("или нажми любую клавишу"),
            TextFont {
                font: font_ui,
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgba(0.65, 0.6, 0.5, 0.6)),
            Node {
                margin: UiRect::top(Val::Px(6.0)),
                ..default()
            },
        ));
    });

    info!("Title Screen displayed");
}

/// Пульсация прозрачности текста-подсказки
pub fn pulsing_text_system(
    time: Res<Time>,
    mut query: Query<&mut TextColor, With<PulsingText>>,
) {
    let alpha = (time.elapsed_secs() * 2.0).sin() * 0.35 + 0.45;
    for mut color in &mut query {
        color.0 = Color::srgba(0.65, 0.6, 0.5, alpha);
    }
}

/// Любой клик или клавиша → начать игру (через fade)
pub fn title_screen_interaction(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    button_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut fade: ResMut<FadeState>,
) {
    if fade.is_active() {
        return;
    }

    let mut start = false;

    for interaction in &button_query {
        if *interaction == Interaction::Pressed {
            start = true;
        }
    }

    if keyboard.any_just_pressed([
        KeyCode::Space, KeyCode::Enter, KeyCode::KeyW, KeyCode::KeyA,
        KeyCode::KeyS, KeyCode::KeyD, KeyCode::ArrowUp, KeyCode::ArrowDown,
    ]) {
        start = true;
    }

    if mouse.just_pressed(MouseButton::Left) {
        start = true;
    }

    if touches.any_just_pressed() {
        start = true;
    }

    if start {
        fade.start_fade(GameState::Playing, false);
    }
}

/// Убирает LoadingOverlay после загрузки шрифтов и появления TitleScreen UI
pub fn remove_loading_overlay(
    mut commands: Commands,
    overlay_query: Query<Entity, With<LoadingOverlay>>,
    title_query: Query<&TitleScreenUI>,
    asset_server: Res<AssetServer>,
) {
    if title_query.is_empty() || overlay_query.is_empty() {
        return;
    }
    let fonts_loaded = [
        asset_paths::FONT_TITLE,
        asset_paths::FONT_UI,
        asset_paths::FONT_UI_BOLD,
    ].iter().all(|path| {
        let handle: Handle<Font> = asset_server.load(*path);
        asset_server.is_loaded_with_dependencies(&handle)
    });
    if !fonts_loaded {
        return;
    }
    for entity in &overlay_query {
        commands.entity(entity).despawn();
    }
}

/// Удаляет Title Screen UI
pub fn cleanup_title_screen(
    mut commands: Commands,
    query: Query<Entity, With<TitleScreenUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
