/// A macro for generating enums with a string representation of their variants.
///
/// This macro provides a convenient way to define enums with a specified set of variants.
/// It automatically implements the `Debug` trait and provides a conversion from a reference
/// of the enum to a string slice that contains the variant name.
///
/// There are two forms of the macro:
///
/// 1. **Without nested types:**
///    ```rust
///    use anime_grubber::gen_enum;
///
///    gen_enum!(MyEnum, [VariantA, VariantB, VariantC]);
///    ```
///    This will generate an enum `MyEnum` with variants `VariantA`, `VariantB`, and `VariantC`.
///
/// 2. **With nested types:**
///    ```rust
///    use anime_grubber::gen_enum;
///
///    gen_enum!(MyEnumWithData, [VariantA(u32), VariantB(String)]);
///    ```
///    This will generate an enum `MyEnumWithData` with variants `VariantA` that holds a `u32`
///    and `VariantB` that holds a `String`.
///
/// # Examples
///
/// ```rust
/// use anime_grubber::gen_enum;
/// // Generating a simple enum
/// gen_enum!(SimpleEnum, [VariantOne, VariantTwo, VariantThree]);
///
/// let variant = SimpleEnum::VariantOne;
/// let converted: &str = (&variant).into();
/// assert_eq!(converted, "VariantOne");
///
/// // Generating an enum with nested types
/// gen_enum!(DataEnum, [ValueA(u32), ValueB(String)]);
///
/// let variant_with_data = DataEnum::ValueA(10);
/// let converted: &str = (&variant_with_data).into();
/// assert_eq!(converted, "ValueA");
/// ```
#[macro_export]
macro_rules! gen_enum {
    ($name:tt, [ $($variant:ident),* $(,)? ]) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub enum $name {
            $(
                $variant,
            )*
        }
        impl From<&$name> for &str {
            fn from(value: &$name) -> Self {
                match value {
                    $(
                        $name::$variant => stringify!($variant),
                    )*
                }
            }
        }
    };
    ($name:tt, [ $($variant:ident($nested:ty)),* $(,)? ]) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub enum $name {
            $(
                $variant($nested),
            )*
        }
        impl From<&$name> for &str {
            fn from(value: &$name) -> Self {
                match value {
                    $(
                        $name::$variant(value) => value.into(),
                    )*
                }
            }
        }
    };
}
