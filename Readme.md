# anime-grubber
[![Crates.io](https://img.shields.io/crates/v/anime-grubber.svg)](https://crates.io/crates/anime-grubber)
[![Documentation]( https://img.shields.io/badge/docs-doc.rs-red)](https://docs.rs/anime-grubber)
[![License](https://img.shields.io/crates/l/anime-grubber.svg)](LICENSE)
удобная библиотека для извлечения изображений милых (или не совсем) персонажей с веб-сайтов.

## Возможности

- Извлечение изображений и GIF с аниме-сайтов и других ресурсов.
- Поддержка работы с различными категориями медиа, включая SFW (Safe for Work) и NSFW (Not Safe for Work).
- Лёгкая интеграция с асинхронными приложениями на базе **Tokio**.
- Встроенные агенты для работы с популярными API (например, Waifu.pics).
- Логирование событий для отладки и отслеживания процесса загрузки медиа.

## Установка

Пропишите в консоль:
```sh
cargo add anime-grubber
```

или

Добавьте **anime-grubber** в ваш `Cargo.toml`:

```toml
[dependencies]
anime-grubber = "0"
```

## Использование

```rust
use anime_grubber::agents::vailfu::{Faifu, Categories, SFW};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let instance = Faifu::new(Categories::SFW(SFW::Dance));
    let image = instance.get().await?;
    println!("Fetched image URL: {}", image);
    Ok(())
}
```

## Примеры использования

### Извлечение одного изображения

Этот пример показывает, как можно получить одно изображение из категории **SFW::Dance**:

```rust
use anime_grubber::agents::vailfu::{Faifu, Categories, SFW};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let instance = Faifu::new(Categories::SFW(SFW::Dance));
    let image = instance.get().await?;
    info!("Fetched image URL: {}", image);
    Ok(())
}
```

### Извлечение нескольких изображений

Получение сразу нескольких изображений из той же категории:

```rust
use anime_grubber::agents::vailfu::{Faifu, Categories, SFW};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let instance = Faifu::new(Categories::SFW(SFW::Dance));
    let images = instance.get_many().await?;
    assert_eq!(images.len(), 30);
    info!("{:#?}", images);
    Ok(())
}
```

## Логирование

Для удобства отладки используется библиотека **tracing**. Пример настройки глобального логгера можно увидеть в тестах.

```rust
use tracing_subscriber::FmtSubscriber;

tracing::subscriber::set_global_default(
    FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG) //  or you can use INFO .with_max_level(tracing::Level::INFO)
        .pretty()
        .without_time()
        .finish(),
).expect("Failed to set global default subscriber");
```

## Тестирование

Библиотека содержит примеры тестов, которые можно найти в модуле [Tests](./tests/):


## Лицензия

Этот проект распространяется под лицензией **MIT**. Подробности см. в файле [LICENSE](./LICENSE).