#[cfg(feature = "macro")]
#[cfg(test)]
mod waifu_pics {
    use anime_grubber::waifu_pics;
    use anime_grubber::waifu_pics::Categories;
    #[test]
    fn macros_standart() {
        let standard = waifu_pics!();
        assert_eq!(standard.categorie, Categories::SFW(waifu_pics::SFW::Waifu))
    }
    #[test]
    fn macros_custom() {
        let expected_categorie = Categories::SFW(waifu_pics::SFW::Blush);
        let custom = waifu_pics!(expected_categorie);
        assert_eq!(custom.categorie, expected_categorie)
    }
    #[test]
    fn macros_aspect() {
        let expected_categorie = Categories::SFW(waifu_pics::SFW::Blush);
        let custom = waifu_pics!(SFW, Blush);
        assert_eq!(custom.categorie, expected_categorie)
    }
}
