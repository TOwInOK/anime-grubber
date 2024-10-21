#[macro_export]
macro_rules! url {
    ($base_url:expr, $categoier:expr, $aspect:expr) => {{
        use tracing::debug;
        let __url = format!(
            "{}/{}/{}",
            $base_url,
            $categoier.to_lowercase(),
            $aspect.to_lowercase()
        );
        debug!("url {:#?}", &__url);
        __url
    }};
}
