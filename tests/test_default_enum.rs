#[cfg(test)]
mod test_default {
    use anime_grubber::gen_enum;

    #[test]
    fn test_simple_enum_default() {
        gen_enum!(SimpleEnum, [First, Second, Third]);

        let default_value = SimpleEnum::default();
        assert_eq!(<&str>::from(&default_value), "First");
    }

    #[test]
    fn test_nested_enum_default() {
        gen_enum!(Inner, [One, Two]);
        gen_enum!(Outer, [Value(Inner), Other(Inner)]);

        let default_value = Outer::default();
        assert_eq!(<&str>::from(&default_value), "Value");
        assert_eq!(default_value.nested_str(), "One");
        assert_eq!(default_value.deepest_str(), "One");
    }

    #[test]
    fn test_deep_nested_enum_default() {
        gen_enum!(Level5, [Five, Six]);
        gen_enum!(Level4, [Fourth(Level5)]);
        gen_enum!(Level3, [Third(Level4)]);
        gen_enum!(Level2, [Second(Level3)]);
        gen_enum!(Level1, [First(Level2)]);

        let default_value = Level1::default();

        // Проверяем, что все уровни используют первый вариант
        assert_eq!(<&str>::from(&default_value), "First");

        // Проверяем deepest_str()
        assert_eq!(default_value.deepest_str(), "Five");

        // Проверяем каждый уровень через деструктуризацию
        let Level1::First(level2) = default_value;
        assert_eq!(<&str>::from(&level2), "Second");

        let Level2::Second(level3) = level2;
        assert_eq!(<&str>::from(&level3), "Third");

        let Level3::Third(level4) = level3;
        assert_eq!(<&str>::from(&level4), "Fourth");

        let Level4::Fourth(level5) = level4;
        assert_eq!(<&str>::from(&level5), "Five");
    }

    #[test]
    fn test_mixed_enum_default() {
        // Enum с обычными вариантами
        gen_enum!(Simple, [A, B, C]);

        // Enum с вложенным Simple
        gen_enum!(Complex, [First(Simple), Second(Simple)]);

        let simple_default = Simple::default();
        assert_eq!(<&str>::from(&simple_default), "A");

        let complex_default = Complex::default();
        assert_eq!(<&str>::from(&complex_default), "First");
        assert_eq!(complex_default.nested_str(), "A");
        assert_eq!(complex_default.deepest_str(), "A");
    }

    #[test]
    fn test_default_consistency() {
        gen_enum!(TestEnum, [First, Second]);

        // Проверяем, что множественные вызовы default() возвращают одинаковый результат
        let default1 = TestEnum::default();
        let default2 = TestEnum::default();
        assert_eq!(<&str>::from(&default1), <&str>::from(&default2));
    }
}
