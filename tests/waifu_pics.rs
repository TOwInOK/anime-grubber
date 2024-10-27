#[cfg(test)]
mod test {
    use anime_grubber::{
        agent::Agent,
        agents::waifu_pics::{Categories, Waifu, SFW},
        gen_enum, url,
    };
    use std::sync::LazyLock;
    use tracing::info;

    static TS: LazyLock<()> = LazyLock::new(|| {
        use tracing::Level;
        use tracing_subscriber::FmtSubscriber;

        tracing::subscriber::set_global_default(
            FmtSubscriber::builder()
                .with_max_level(Level::DEBUG)
                .pretty()
                .without_time()
                .finish(),
        )
        .expect("Fail to set global default subscriber");
    });

    #[tokio::test]
    async fn check_solo() -> anyhow::Result<()> {
        LazyLock::force(&TS);
        let instanse = Waifu::new(Categories::SFW(SFW::Dance));
        let image = instanse.get().await?;
        info!("{image}");
        Ok(())
    }
    #[tokio::test]
    async fn check_many() -> anyhow::Result<()> {
        LazyLock::force(&TS);
        let instanse = Waifu::new(Categories::SFW(SFW::Dance));
        let images = instanse.get_many().await?;
        assert_eq!(images.len(), 30);
        info!("{:#?}", images);
        Ok(())
    }

    #[test]
    fn url() -> anyhow::Result<()> {
        const SOLO_URL: &str = "https://api.waifu.pics";
        let url = url!(SOLO_URL, "Test", "tseT");
        let expected = "https://api.waifu.pics/test/tset";
        assert_eq!(&url, expected);
        Ok(())
    }
    #[test]
    fn deep_nested_enum() {
        gen_enum!(Level5, [Five, Six]);
        gen_enum!(Level4, [Fourth(Level5)]);
        gen_enum!(Level3, [Third(Level4)]);
        gen_enum!(Level2, [Second(Level3)]);
        gen_enum!(Level1, [First(Level2)]);

        let deep = Level1::First(Level2::Second(Level3::Third(Level4::Fourth(Level5::Five))));

        // Проверяем прямое преобразование каждого уровня
        assert_eq!(<&str>::from(&deep), "First");

        // Проверяем получение самого глубокого значения
        assert_eq!(deep.deepest_str(), "Five");

        // Проверяем каждый уровень через деструктуризацию
        let Level1::First(level2) = deep;
        assert_eq!(<&str>::from(&level2), "Second");

        let Level2::Second(level3) = level2;
        assert_eq!(<&str>::from(&level3), "Third");

        let Level3::Third(level4) = level3;
        assert_eq!(<&str>::from(&level4), "Fourth");

        let Level4::Fourth(level5) = level4;
        assert_eq!(<&str>::from(&level5), "Five");
    }
}
