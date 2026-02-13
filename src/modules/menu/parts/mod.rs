//! Реализация UI экранов:
//! - title_screen: заставка с фоном, пульсирующий текст, переход по любой клавише
//! - game_over_screen: статистика (волна, время, убийства), кнопки "Заново" / "В Меню"
//! - hud: минимальный HUD — волна (лево), таймер + убийства (право)
//! - fps_counter: счётчик FPS в углу (обновление 4 раза/сек)
//! - button_hover: универсальный hover-эффект для кнопок
//! - fade_transition: плавное затемнение между экранами (Real time, работает при паузе)

pub mod title_screen;
pub mod game_over_screen;
pub mod hud;
pub mod fps_counter;
pub mod button_hover;
pub mod fade_transition;
pub mod font_diagnostics;
