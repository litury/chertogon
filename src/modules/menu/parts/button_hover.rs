use bevy::prelude::*;

/// Единая hover-система для всех кнопок меню
pub fn button_hover_system(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut BoxShadow),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut bg, mut shadow) in &mut query {
        match *interaction {
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgba(0.95, 0.7, 0.2, 0.18));
                shadow.0 = vec![ShadowStyle {
                    color: Color::srgba(0.95, 0.7, 0.2, 0.35),
                    x_offset: Val::Px(0.0),
                    y_offset: Val::Px(0.0),
                    spread_radius: Val::Px(4.0),
                    blur_radius: Val::Px(22.0),
                }];
            }
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgba(0.95, 0.7, 0.2, 0.28));
                shadow.0 = vec![ShadowStyle {
                    color: Color::srgba(1.0, 0.5, 0.1, 0.45),
                    x_offset: Val::Px(0.0),
                    y_offset: Val::Px(0.0),
                    spread_radius: Val::Px(6.0),
                    blur_radius: Val::Px(28.0),
                }];
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgba(0.95, 0.7, 0.2, 0.06));
                shadow.0 = vec![ShadowStyle {
                    color: Color::srgba(0.95, 0.7, 0.2, 0.15),
                    x_offset: Val::Px(0.0),
                    y_offset: Val::Px(0.0),
                    spread_radius: Val::Px(2.0),
                    blur_radius: Val::Px(15.0),
                }];
            }
        }
    }
}
