# ЧЕРТОГОН / Chertogon

[![CI](https://github.com/litury/chertogon/actions/workflows/ci.yml/badge.svg)](https://github.com/litury/chertogon/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Bevy](https://img.shields.io/badge/Bevy-0.18-232326?logo=bevy&logoColor=white)](https://bevyengine.org)

3D top-down roguelike arena shooter на [Bevy 0.18](https://bevyengine.org/).
Gothic Slavic dark fantasy, вдохновлённый Vampire Survivors, но в полном 3D.

> Портал в пекло треснул. Упыри ломятся в мир живых.
> Ты — богатырь с рунным мечом. Режь нечисть, прокачивайся, заливай арену кровью.

## Возможности

- 3D графика с top-down видом камеры
- Управление персонажем (WASD + Shift для бега)
- Анимации персонажа (idle / walk / run / attack)
- Враги (упыри) с AI: патрулирование, преследование, атака
- Автоматическая атака ближайшего врага
- Боевая система: урон, смерть, VFX (slash, частицы, тряска камеры)
- Физика через Avian3D (коллизии, слои)
- Gothic арена 50x50м со стенами и факелами
- WASD + тач-управление

## Быстрый старт

### Требования

- Rust 1.82+ ([установить](https://rustup.rs/))
- macOS, Linux или Windows

### Установка и запуск

```bash
# Клонировать репозиторий
git clone https://github.com/litury/chertogon.git
cd chertogon

# Запуск (первая компиляция займёт несколько минут)
cargo run
```

## Управление

| Клавиша | Действие |
|---------|----------|
| **W/A/S/D** | Движение персонажа |
| **Shift** | Бег (удерживать вместе с WASD) |
| **Esc** | Закрыть игру |

## Структура проекта

```
chertogon/
├── Cargo.toml
├── src/
│   ├── main.rs                          # Точка входа
│   ├── lib.rs                           # Экспорт модулей
│   ├── config/
│   │   └── game_config.rs               # Настройка App, плагинов
│   ├── modules/
│   │   ├── player/                      # Игрок: спавн, движение, анимации
│   │   ├── camera/                      # Камера: setup, следование за игроком
│   │   ├── input/                       # Ввод: клавиатура, тач
│   │   ├── world/                       # Мир: арена, стены, факелы
│   │   ├── enemies/                     # Враги: спавн, AI, анимации
│   │   └── combat/                      # Бой: автоатака, урон, VFX
│   ├── shared/
│   │   └── constants.rs                 # Общие константы
│   └── toolkit/
│       └── asset_paths.rs               # Пути к ассетам
└── assets/
    ├── models/                          # 3D модели (GLB)
    └── textures/                        # PBR текстуры
```

## Архитектура

Каждый модуль — это Bevy Plugin. Модули независимы и могут быть:
- Изменены без затрагивания других модулей
- Отключены (закомментировать в `main.rs`)
- Расширены новыми системами и компонентами

Внутренняя реализация каждого модуля находится в папке `parts/` и скрыта от внешнего кода.

## Документация

- [Game Design Document](GAME_DESIGN.md) — полное описание механик, врагов, апгрейдов и роадмапа

## Разработка

```bash
# Debug-сборка с hot reload
cargo run

# Release-сборка
cargo build --release
./target/release/chertogon

# Логи
RUST_LOG=debug cargo run

# Форматирование и линтер
cargo fmt
cargo clippy
```

## Используемые технологии

- **[Bevy 0.18](https://bevyengine.org/)** — игровой движок
- **[Avian3D](https://github.com/Jondolf/avian)** — физический движок
- **[bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui)** — инспектор для отладки

## Лицензия

Код проекта распространяется под двойной лицензией:
- [MIT License](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

По вашему выбору.
