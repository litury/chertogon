# Как внести вклад / Contributing

Спасибо за интерес к проекту! Вот как можно помочь.

## Настройка окружения

1. Установить [Rust](https://rustup.rs/) (1.82+)
2. Клонировать репозиторий и запустить:

```bash
git clone https://github.com/USERNAME/chertogon.git
cd chertogon
cargo run
```

## Стиль кода

Перед коммитом:

```bash
cargo fmt       # форматирование
cargo clippy    # линтер
cargo build     # проверка сборки
```

## Стиль коммитов

Формат: `verb(scope): brief description`

Подробнее — см. [doc/git.md](doc/git.md).

## Pull Request

1. Форкните репозиторий
2. Создайте ветку: `git checkout -b feat/my-feature`
3. Сделайте изменения, убедитесь что `cargo fmt` и `cargo clippy` проходят
4. Отправьте PR с описанием что и зачем изменено
