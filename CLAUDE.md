# Chertogon — AI Context

3D top-down roguelike arena shooter на Bevy 0.18. Gothic Slavic dark fantasy, вдохновлённый Vampire Survivors, но в полном 3D.

## Технологии

- **Rust** (1.82+), **Bevy 0.18**, **Avian3D 0.6** (физика), **bevy_firework** (частицы)
- Top-down камера, нулевая гравитация, коллизии через слои (`GameLayer`)
- Состояния: `TitleScreen → Playing → GameOver` (см. `src/shared/game_state.rs`)

## Архитектура модулей

Каждый модуль в `src/modules/` — изолированный Bevy Plugin со структурой:

```
module/
├── mod.rs          # Публичный API (pub use)
├── plugin.rs       # Регистрация систем в App
├── components.rs   # ECS компоненты (данные)
└── parts/          # Внутренняя реализация (скрыта от внешнего кода)
    ├── mod.rs      # Внутренние экспорты
    └── *.rs        # Системы (по одной функции на файл)
```

**Правила:**
- Импорт компонентов: `use crate::modules::player::Player` — через `mod.rs`
- Никогда не импортировать из `parts/` напрямую из другого модуля
- Новые системы = новый файл в `parts/`, регистрация в `plugin.rs`
- Максимум 7-10 файлов в папке; если больше — выделить подмодуль

## Ключевые файлы

- `src/main.rs` — точка входа (27 строк, подключение плагинов)
- `src/config/game_config.rs` — настройка App, физики, окна
- `src/shared/constants.rs` — скорости, параметры камеры, `GameLayer`
- `src/shared/game_state.rs` — `GameState` enum (роутер состояний)
- `src/toolkit/asset_paths.rs` — все пути к ассетам (модели, текстуры, шрифты)
- `GAME_DESIGN.md` — полное описание механик, врагов, апгрейдов, роадмапа

## 7 модулей

| Модуль | Назначение | Зависит от |
|--------|-----------|------------|
| `world` | Арена 50x50м, стены, факелы | shared |
| `input` | WASD + touch ввод | shared |
| `player` | Спавн, движение, анимации, оружие | input, shared |
| `camera` | Top-down камера, зум, следование | player, shared |
| `enemies` | Волновой спавнер, AI, смерть | player, shared |
| `combat` | Автоатака, урон, VFX, game over | player, enemies, shared |
| `menu` | Title screen, HUD, game over UI | combat, shared |

## Команды

```bash
cargo run                          # Запуск (debug, hot reload)
cargo run --features remote_debug  # С BRP диагностикой (порт 15702)
cargo build --release              # Release сборка
cargo fmt                          # Форматирование
cargo clippy                       # Линтер
```

## Стиль кода

- Файлы: `snake_case.rs`
- Компоненты/ресурсы: `PascalCase`, derive `Component`/`Resource` + `Reflect`
- Системы: `описательное_имя_system` (напр. `player_auto_attack_system`)
- Doc-comments (`///`) на русском
- Модули: единственное число (`player`, `camera`, `world`)
- Ассеты: merged GLB (все анимации в одном файле), PBR текстуры (`*_diff`, `*_nor`, `*_rough`)

## Конвенция коммитов

`verb(scope): краткое описание` — подробнее в `doc/git.md`
