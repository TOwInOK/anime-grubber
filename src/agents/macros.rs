#[macro_export]
macro_rules! waifu_pics {
    ($($categorie:expr)?) => {{
        use $crate::agents::waifu_pics::Waifu;
        let __waifu = Waifu::default();
        $(let __waifu = Waifu::new($categorie);)?
        __waifu
    }};
    ($main:ident, $aspect:ident) => {{
        use $crate::agents::waifu_pics::Waifu;
        let __waifu = Waifu::new(Categories::$main(waifu_pics::$main::$aspect));
        __waifu
    }};
}
