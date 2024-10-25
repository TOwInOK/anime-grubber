#[cfg(test)]
mod test {
    use anime_grubber::{
        agent::Agent,
        agents::waifu_pics::{Categories, Waifu, SFW},
        url,
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
}
