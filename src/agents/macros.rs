#[macro_export]
/// Macros for generate [Waifu]
/// # Examples
/// if you need default
/// fn default() {
///     let standard = waifu_pics!();
///     assert_eq!(standard.categorie, Categories::SFW(waifu_pics::SFW::Waifu))
/// }
/// if you need to choose category with expr
/// fn with_categories() {
///     let expected_categorie = Categories::SFW(waifu_pics::SFW::Blush);
///     let custom = waifu_pics!(expected_categorie);
///     assert_eq!(custom.categorie, expected_categorie)
/// }
/// if you need to choose category with tokens
/// fn with_aspects() {
///     let expected_categorie = Categories::SFW(waifu_pics::SFW::Blush);
///     let custom = waifu_pics!(SFW, Blush);
///     assert_eq!(custom.categorie, expected_categorie)
/// }
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
